use serde::{Deserialize, Serialize};

/// The Currency code corresponding to the amount of Money.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    GBP,
    USD,
    EUR,
    JPY,
    SGD
}