# remo-e-exporter: Prometheus exporter for Nature Remo E

## Setup

### Configuration

Follows [kenfdev/remo-exporter](https://github.com/kenfdev/remo-exporter):

- Bearer token (either one of the followings)
  - `OAUTH_TOKEN`
  - `OAUTH_TOKEN_FILE`
- `CACHE_INVALIDATION_SECONDS`
- `BIND_ADDRESS` (default to: `[::]:9742`)

## Metrics


```
# HELP remo_coefficient Coefficient for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xD3)
# TYPE remo_coefficient gauge
remo_coefficient{id="[deducted]",name="Home smart meter"} 1

# HELP remo_cumulative_electric_energy_effective_digits Number of effective digits for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xD7)
# TYPE remo_cumulative_electric_energy_effective_digits gauge
remo_cumulative_electric_energy_effective_digits{id="[deducted]",name="Home smart meter"} 6

# HELP remo_cumulative_electric_energy_unit Unit in kWh for remo_*_cumulative_electric_energy metrics (echonet lite property, smart meter EPC=0xE1)
# TYPE remo_cumulative_electric_energy_unit gauge
remo_cumulative_electric_energy_unit{id="[deducted]",name="Home smart meter"} 0.1

# HELP remo_measured_instantaneous Measured instantaneous usage in W (echonet lite property, smart meter EPC=0xE7)
# TYPE remo_measured_instantaneous gauge
remo_measured_instantaneous{id="[deducted]",name="Home smart meter"} 961

# HELP remo_normal_direction_cumulative_electric_energy Raw value for cumulative electric energy usage in positive direction (echonet lite property, smart meter EPC=0xE0)
# TYPE remo_normal_direction_cumulative_electric_energy counter
remo_normal_direction_cumulative_electric_energy{id="[deducted]",name="Home smart meter"} 240430

# HELP remo_reverse_direction_cumulative_electric_energy Raw value for cumulative electric energy usage in reverse direction (echonet lite property, smart meter EPC=0xE3)
# TYPE remo_reverse_direction_cumulative_electric_energy counter
remo_reverse_direction_cumulative_electric_energy{id="[deducted]",name="Home smart meter"} 27
```

### Labels

- `id`: Appliance ID
- `name`: Appliance nickname

## Recommended recording rules
