use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::abi::Direction;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SugIndicator {
    pub ntps: i64,
    pub ntps_fast_time: Option<i64>,
    pub trandm: f64,
    pub asset_01: f64,
    pub asset_diff_01: f64,
    pub price_1h: f64,
    pub price_4h: f64,
    pub price_8h: f64,
    pub price_24h: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MovementStatusLite {
    Unknown,
    Shrink,
    SharpUp,
    CascadeUp,
    RollbackUp,
    SharpDown,
    CascadeDown,
    RollbackDown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementLite {
    pub status: MovementStatusLite,
    pub direction: Direction,
    pub break_price: Option<f64>,
    pub activate_price: f64,
    pub price_min: f64,
    pub price_max: f64,
    pub price_start: f64,
    pub price_end: f64,
    pub qtym: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleSugInfo {
    pub direction: Direction,
    pub indicator: SugIndicator,
    #[serde(default)]
    pub movements: HashMap<i64, MovementLite>,
}
