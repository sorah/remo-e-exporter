use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown cumulative electric energy unit: {0}")]
    UnknownCumulativeElectricEnergyUnit(String),
    #[error("Unsupported kind for rollout")]
    UnsupportedRolloutKind,
}
