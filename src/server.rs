use crate::nature_client;
use std::time;
use tokio::sync::{RwLock, Mutex};
use std::sync::Arc;

pub struct Server {
    pub last: Arc<RwLock<time::Instant>>,
    pub cache_duration: time::Duration,
	registry: prometheus::Registry,
	engine: Arc<Mutex<Engine>>,
}

struct Engine {
    client: nature_client::Client,
    gauge_measured_instantaneous: prometheus::GaugeVec,
    gauge_coefficient: prometheus::GaugeVec,
    gauge_cumulative_electric_energy_unit: prometheus::GaugeVec,
    gauge_cumulative_electric_energy_effective_digits: prometheus::GaugeVec,
    counter_normal_direction_cumulative_electric_energy: prometheus::CounterVec,
    counter_reverse_direction_cumulative_electric_energy: prometheus::CounterVec,
}

impl Server {
    pub fn new(client: nature_client::Client, cache_duration: time::Duration) -> Self {
        use prometheus::{Opts, GaugeVec, CounterVec, Registry};
        let registry = Registry::new();

        let gauge_measured_instantaneous = GaugeVec::new(
            Opts::new("remo_measured_instantaneous", "Measured instantaneous usage in W (echonet lite property, smart meter EPC=0xE7)"),
            &["name", "id"],
        ).unwrap();
        let gauge_coefficient = GaugeVec::new(
            Opts::new(
                "remo_coefficient",
                "Coefficient for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xD3)",
            ),
            &["name", "id"],
        ).unwrap();
        let gauge_cumulative_electric_energy_effective_digits = GaugeVec::new(
            Opts::new(
                "remo_cumulative_electric_energy_effective_digits",
                "Number of effective digits for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xD7)",
            ),
            &["name", "id"],
        ).unwrap();
        let gauge_cumulative_electric_energy_unit = GaugeVec::new(
            Opts::new(
                "remo_cumulative_electric_energy_unit",
                "Unit in kWh for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xE1)",
            ),
            &["name", "id"],
        ).unwrap();
        let counter_normal_direction_cumulative_electric_energy = CounterVec::new(
            Opts::new(
                "remo_normal_direction_cumulative_electric_energy",
                "Raw value for cumulative electric energy usage in positive direction (echonet lite property, smart meter EPC=0xE0)",
            ),
            &["name", "id"],
        ).unwrap();
        let counter_reverse_direction_cumulative_electric_energy = CounterVec::new(
            Opts::new(
                "remo_reverse_direction_cumulative_electric_energy",
                "Raw value for cumulative electric energy usage in reverse direction (echonet lite property, smart meter EPC=0xE3)",
            ),
            &["name", "id"],
        ).unwrap();

        registry.register(Box::new(gauge_measured_instantaneous.clone())).unwrap();
        registry.register(Box::new(gauge_coefficient.clone())).unwrap();
        registry.register(Box::new(gauge_cumulative_electric_energy_unit.clone())).unwrap();
        registry.register(Box::new(gauge_cumulative_electric_energy_effective_digits.clone())).unwrap();
        registry.register(Box::new(counter_normal_direction_cumulative_electric_energy.clone())).unwrap();
        registry.register(Box::new(counter_reverse_direction_cumulative_electric_energy.clone())).unwrap();

		let engine = Engine {
            client,
            gauge_measured_instantaneous,
            gauge_coefficient,
            gauge_cumulative_electric_energy_unit,
            gauge_cumulative_electric_energy_effective_digits,
            counter_normal_direction_cumulative_electric_energy,
            counter_reverse_direction_cumulative_electric_energy,
		};

        Self {
            registry,
            last: Arc::new(RwLock::new(time::Instant::now() - cache_duration - time::Duration::new(1, 0))),
            cache_duration,
			engine: Arc::new(Mutex::new(engine)),
        }
    }

    pub async fn serve(&self, _req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
		match self.serve_internal().await {
			Ok(response) => Ok(response),
			Err(err) => {
				log::error!("Error while serving request: {}", err);
				let mut err_response = hyper::Response::new(hyper::Body::empty());
				*err_response.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
				Ok(err_response)
			}
		}
    }

	pub async fn serve_internal(&self) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error>> {
		use prometheus::Encoder;
        self.update().await?;
        let encoder = prometheus::TextEncoder::new();
        let mut buf = vec![];
        encoder.encode(&self.registry.gather(), &mut buf)?;
        let response = hyper::Response::builder()
            .status(200)
            .header(hyper::header::CONTENT_TYPE, encoder.format_type())
            .body(hyper::Body::from(buf))?;
        Ok(response)

	}

	pub async fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
		if (*self.last.read().await).elapsed() < self.cache_duration {
			return Ok(());
		}

		let mut last = self.last.write().await;
		*last = time::Instant::now();

		let engine = self.engine.lock().await;
		engine.update().await
	}
}

impl Engine {
    pub async fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
		log::info!("Updating metrics");
        let appliances = self.client.appliances().await?;
		for appliance in appliances.into_iter() {
            if !appliance.smart_meter.is_some() {
                continue;
            }
			let smart_meter = appliance.smart_meter.unwrap();
            if !smart_meter.echonetlite_properties.is_some() {
                continue;
            }
            let labels = vec![appliance.nickname.as_ref(), appliance.id.as_ref()];
            let properties = smart_meter.echonetlite_properties.unwrap();
			for prop in properties.into_iter() {
                let value_f: Result<f64, std::num::ParseFloatError> = prop.val.parse();
                match prop.epc {
                    0xE7 => self.gauge_measured_instantaneous.with_label_values(labels.as_slice()).set(value_f?),
                    0xD3 => self.gauge_coefficient.with_label_values(labels.as_slice()).set(value_f?),
                    0xD7 => self.gauge_cumulative_electric_energy_effective_digits.with_label_values(labels.as_slice()).set(value_f?),
                    0xE1 => {
                        let value_i: u8 = prop.val.parse()?;
                        let unit: f64 = match value_i {
                            0x00 => 1.0,
                            0x01 => 0.1,
                            0x02 => 0.01,
                            0x03 => 0.001,
                            0x04 => 0.0001,
                            0x0A => 10.0,
                            0x0B => 100.0,
                            0x0C => 1000.0,
                            0x0D => 10000.0,
                            _ => return Err(Box::new(crate::errors::Error::UnknownCumulativeElectricEnergyUnit(prop.val))),
                        };
                        self.gauge_cumulative_electric_energy_unit.with_label_values(labels.as_slice()).set(unit);
                    }
                    0xE0 => {
						let counter = self.counter_normal_direction_cumulative_electric_energy.with_label_values(labels.as_ref());
						// XXX:
						counter.reset();
						counter.inc_by(value_f?);
					}
                    0xE3 => {
						let counter = self.counter_reverse_direction_cumulative_electric_energy.with_label_values(labels.as_ref());
						// XXX:
						counter.reset();
						counter.inc_by(value_f?);
					}
                    _ => {}
                }
            }
        }
		Ok(())
    }

}

