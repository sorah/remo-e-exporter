# remo-e-exporter: Prometheus exporter for Nature Remo E

## Setup

### Configuration

Follows [kenfdev/remo-exporter](https://github.com/kenfdev/remo-exporter):

- Bearer token (either one of the followings)
  - `OAUTH_TOKEN`
  - `OAUTH_TOKEN_FILE`
- `CACHE_INVALIDATION_SECONDS`
- `BIND_ADDRESS` (default to:)

## Metrics


```
# HELP remo_coefficient (smart meter epc=0x30,224)
# TYPE remo_coefficient gauge

# HELP remo_measured_instantaneous (smart meter epc=0xE7,231)
# TYPE remo_measured_instantaneous gauge

# HELP remo_cumulative_electric_energy_unit (smart meter epc=0xE1,225)
# TYPE remo_cumulative_electric_energy_unit gauge

# HELP remo_cumulative_electric_energy_effective_digits (smart meter epc=0xD7,215)
# TYPE remo_cumulative_electric_energy_effective_digits gauge

# HELP remo_normal_direction_cumulative_electric_energy (smart meter epc=0xE0,224)
# TYPE remo_normal_direction_cumulative_electric_energy counter

# HELP remo_reverse_direction_cumulative_electric_energy (smart meter epc=0xE3,227)
# TYPE remo_reverse_direction_cumulative_electric_energy counter


```

### Labels

- `id`: Appliance ID
- `name`: Appliance nickname

## Recommended recording rules
