use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Client {
    addr: reqwest::Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(api_base: String, token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = reqwest::Url::parse(&api_base)?;
        let mut http = reqwest::Client::builder();
        let mut http_headers = reqwest::header::HeaderMap::new();
        http_headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?);
        http = http.default_headers(http_headers);
        Ok(Self { addr, http: http.build()? })
    }

    pub async fn appliances(&self) -> Result<Vec<Appliance>, Box<dyn std::error::Error>> {
        let response = self.http.get(self.addr.join("/1/appliances")?).send().await?.error_for_status()?;
        let appliances: Vec<Appliance> = response.json().await?;
        Ok(appliances)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appliance {
    pub id: String,
    pub nickname: String,
    pub smart_meter: Option<SmartMeter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartMeter {
    pub echonetlite_properties: Option<Vec<EchonetliteProperties>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchonetliteProperties {
    pub name: String,
    pub epc: u8,
    pub val: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
