use std::collections::HashMap;

use serde::{de::Error as _, Deserialize, Deserializer, Serialize};

use crate::abi::Direction;

fn to_measure_price(a: &f64, b: &f64) -> f64 {
    if *a == 0.0 {
        0.0
    } else {
        ((b - a) / a) * 100.0
    }
}

fn to_percentage_between(value: &f64, min: &f64, max: &f64) -> f64 {
    let span = max - min;
    if span == 0.0 {
        0.5
    } else {
        ((value - min) / span).clamp(0.0, 1.0)
    }
}

fn de_int_key<'de, D, V>(deserializer: D) -> Result<HashMap<i64, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    let raw: HashMap<String, V> = HashMap::deserialize(deserializer)?;
    raw.into_iter()
        .map(|(k, v)| {
            let parsed = k
                .parse::<i64>()
                .map_err(|_| D::Error::custom(format!("invalid integer map key: {k}")))?;
            Ok((parsed, v))
        })
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfoCandle {
    pub min: f64,
    pub max: f64,
    pub enter: f64,
    pub exit: f64,
    pub qty: f64,
    pub qty_asset: f64,
    pub qtym: f64,
    pub qtym_asset: f64,
}

impl SuggestionInfoCandle {
    pub fn new(price: f64) -> Self {
        Self {
            enter: price,
            exit: price,
            max: price,
            min: price,
            qty: 0.0,
            qty_asset: 0.0,
            qtym: 0.0,
            qtym_asset: 0.0,
        }
    }

    pub fn update_price(&mut self, price: f64) {
        self.exit = price;
        self.max = self.max.max(price);
        self.min = self.min.min(price);
    }

    pub fn update_from_candle(&mut self, candle: &Self) {
        self.update_price(candle.exit);
        self.qty += candle.qty;
        self.qty_asset += candle.qty_asset;
        self.qtym += candle.qtym;
        self.qtym_asset += candle.qtym_asset;
    }

    pub fn get_prc(&self) -> f64 {
        to_measure_price(&self.enter, &self.exit)
    }

    pub fn get_mm_prc(&self) -> f64 {
        to_measure_price(&self.min, &self.max)
    }

    pub fn get_mm_direction(&self, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => self.max,
            _ => self.min,
        }
    }

    pub fn get_mm_odirection(&self, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => self.min,
            _ => self.max,
        }
    }

    pub fn get_mm_dir_price(&self, direction: &Direction) -> f64 {
        self.get_mm_direction(direction)
    }

    pub fn get_direction(&self) -> Direction {
        Direction::from_value(self.exit - self.enter)
    }

    pub fn include_price(&self, price: &f64) -> bool {
        &self.min <= price && price <= &self.max
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionKline {
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfoWave {
    pub enter_time: i64,
    pub enter_price: f64,
    pub exit_time: i64,
    pub exit_price: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub qtym: f64,
    pub percentile: f64,
    pub direction: Direction,
    pub over_candles: Vec<(usize, f64)>,
}

impl SuggestionInfoWave {
    pub fn get_mm_prc(&self) -> f64 {
        to_measure_price(&self.min_price, &self.max_price)
    }

    pub fn get_min_price(&self) -> f64 {
        self.min_price
    }

    pub fn get_max_price(&self) -> f64 {
        self.max_price
    }

    pub fn get_mm_from_direction(&self, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => self.max_price,
            _ => self.min_price,
        }
    }

    pub fn get_mm_dir_price(&self, direction: &Direction) -> f64 {
        self.get_mm_from_direction(direction)
    }

    pub fn get_mm_dir_time(&self, direction: &Direction) -> i64 {
        match direction {
            Direction::Long => self.exit_time,
            _ => self.enter_time,
        }
    }

    pub fn get_tfs(&self, tf: i64) -> f64 {
        (self.exit_time - self.enter_time) as f64 / (tf * 1_000) as f64
    }

    pub fn get_peak_prc(&self, price: &f64) -> f64 {
        let peak_price = self.get_mm_from_direction(&self.direction);
        to_measure_price(price, &peak_price)
    }

    pub fn get_peak_prc_by_direction(&self, price: &f64, direction: &Direction) -> f64 {
        let peak_price = self.get_mm_from_direction(direction);
        to_measure_price(price, &peak_price)
    }

    pub fn get_diff(&self) -> i64 {
        self.exit_time - self.enter_time
    }

    pub fn get_price_diff(&self) -> f64 {
        self.exit_price - self.enter_price
    }

    pub fn get_diff_value_prc(&self, base: f64) -> f64 {
        if base == 0.0 {
            0.0
        } else {
            (self.get_price_diff() / base) * 100.0
        }
    }

    pub fn get_price_prc(&self) -> f64 {
        to_measure_price(&self.enter_price, &self.exit_price)
    }

    pub fn include_price(&self, price: &f64) -> bool {
        &self.min_price <= price && price <= &self.max_price
    }

    pub fn include_time(&self, time: &i64) -> bool {
        &self.enter_time <= time && time <= &self.exit_time
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfoOrderBookLevel {
    pub price: f64,
    pub quantity: f64,
    #[serde(default)]
    pub total_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<f64>,
    pub time_start: i64,
    pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfoOrderBook {
    #[serde(default = "Vec::new")]
    pub long_levels: Vec<SuggestionInfoOrderBookLevel>,
    #[serde(default = "Vec::new")]
    pub short_levels: Vec<SuggestionInfoOrderBookLevel>,
    #[serde(default)]
    pub sell_amount: f64,
    pub sell_price: Option<f64>,
    #[serde(default)]
    pub buy_amount: f64,
    pub buy_price: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfoOpenInterest {
    pub usd: f64,
    pub change: f64,
    pub change_qty: f64,
    pub avg: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SuggestionInfoIndicator {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum LineConsolidationMoveStatus {
    #[default]
    Unknown,
    Shrink,
    SharpUp,
    CascadeUp,
    RollbackUp,
    SharpDown,
    CascadeDown,
    RollbackDown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum LineConsolidationMoveReason {
    #[default]
    Default,
    BreakMove,
    AfterSave,
    Cascade,
    Sharp,
    Unknown,
    Rollback,
    InitFromPrev,
    MoveOutRollback,
    MoveOutContinue,
    Accumulate,
    OnWaveStatus,
    MoveTooLong,
    FromBreakRollback,
    FromFreeze,
}

impl LineConsolidationMoveStatus {
    pub fn get_direction(&self) -> Direction {
        match self {
            Self::SharpUp => Direction::Long,
            Self::CascadeUp => Direction::Long,
            Self::RollbackUp => Direction::Both,
            Self::SharpDown => Direction::Short,
            Self::CascadeDown => Direction::Short,
            Self::RollbackDown => Direction::Both,
            Self::Shrink => Direction::Both,
            Self::Unknown => Direction::Both,
        }
    }

    pub fn get_number(&self) -> f64 {
        match self {
            Self::SharpUp => 3.0,
            Self::CascadeUp => 2.0,
            Self::RollbackUp => 1.0,
            Self::SharpDown => -3.0,
            Self::CascadeDown => -2.0,
            Self::RollbackDown => -1.0,
            Self::Shrink => 0.0,
            Self::Unknown => 0.0,
        }
    }

    pub fn is_oposite(&self, status: &Self) -> bool {
        let direction = self.get_direction();
        let status_direction = status.get_direction();
        !direction.is_both() && direction.oposite() == status_direction
    }

    pub fn is_window(&self) -> bool {
        self == &Self::Shrink
    }

    pub fn is_active(&self) -> bool {
        self.is_cascade() || self.is_sharp()
    }

    pub fn can_out(&self) -> bool {
        matches!(self, Self::Shrink | Self::Unknown)
    }

    pub fn is_sharp(&self) -> bool {
        self == &Self::SharpDown || self == &Self::SharpUp
    }

    pub fn is_cascade(&self) -> bool {
        self == &Self::CascadeDown || self == &Self::CascadeUp
    }

    pub fn is_up(&self) -> bool {
        self == &Self::SharpUp || self == &Self::CascadeUp || self == &Self::RollbackUp
    }

    pub fn is_down(&self) -> bool {
        self == &Self::SharpDown || self == &Self::CascadeDown || self == &Self::RollbackDown
    }

    pub fn is_rollback(&self) -> bool {
        self == &Self::RollbackDown || self == &Self::RollbackUp
    }
}

impl LineConsolidationMoveReason {
    pub fn is_out(&self) -> bool {
        self == &LineConsolidationMoveReason::BreakMove
            || self == &LineConsolidationMoveReason::FromBreakRollback
            || self == &LineConsolidationMoveReason::MoveOutRollback
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineConsolidationMoveSuggestion {
    pub status: LineConsolidationMoveStatus,
    pub direction: Direction,
    pub break_price: Option<f64>,
    pub time_start: i64,
    pub time_end: i64,
    pub activate_time: i64,
    pub activate_price: f64,
    pub activate_time_changes: Vec<i64>,
    pub activate_price_changes: Vec<f64>,
    pub price_min: f64,
    pub price_max: f64,
    pub price_start: f64,
    pub price_end: f64,
    pub qtym: f64,
    pub reason: LineConsolidationMoveReason,
    pub rollbacks: i64,
    pub rollback_time_changes: Vec<i64>,
    pub rollback_price_changes: Vec<f64>,
    pub mov_qtym_min: f64,
    pub mov_qtym_max: f64,
}

impl LineConsolidationMoveSuggestion {
    pub fn get_mm_prc(&self) -> f64 {
        to_measure_price(&self.price_min, &self.price_max)
    }

    pub fn get_mm_activate_prc(&self) -> f64 {
        match self.direction {
            Direction::Long => to_measure_price(&self.activate_price, &self.price_max),
            _ => to_measure_price(&self.activate_price, &self.price_min) * -1.0,
        }
    }

    pub fn get_price_between_by_direction(&self, price: &f64, direction: &Direction) -> f64 {
        let prc = to_percentage_between(price, &self.price_min, &self.price_max);
        match direction {
            Direction::Long => prc,
            _ => 1.0 - prc,
        }
    }

    pub fn get_price_between(&self, price: &f64) -> f64 {
        self.get_price_between_by_direction(price, &self.direction)
    }

    pub fn get_active_price_between(&self) -> f64 {
        self.get_price_between(&self.activate_price)
    }

    pub fn include_price(&self, price: &f64) -> bool {
        &self.price_min <= price && price <= &self.price_max
    }

    pub fn include_time(&self, time: &i64) -> bool {
        &self.time_start <= time && time <= &self.time_end
    }

    pub fn get_mm_direction(&self) -> f64 {
        match self.direction {
            Direction::Long => self.price_max,
            _ => self.price_min,
        }
    }

    pub fn get_mm_from_direction(&self, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => self.price_max,
            _ => self.price_min,
        }
    }

    pub fn get_mm_odirection(&self) -> f64 {
        match self.direction {
            Direction::Long => self.price_min,
            _ => self.price_max,
        }
    }

    pub fn get_mm_from_odirection(&self, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => self.price_min,
            _ => self.price_max,
        }
    }

    pub fn get_time_diff(&self) -> i64 {
        self.time_end - self.time_start
    }

    pub fn get_time(&self) -> i64 {
        if self.rollbacks > 0 {
            self.activate_time
        } else {
            self.time_start
        }
    }

    pub fn is_good_rollback(&self) -> bool {
        const TF_1M: i64 = 60;
        (self.time_end - self.time_start) > TF_1M * 3_000 || self.get_active_price_between() <= 0.5
    }

    pub fn get_mid_price(&self) -> f64 {
        (self.price_min + self.price_max) / 2.0
    }

    pub fn get_rest_prc(&self, price: &f64, direction: &Direction) -> f64 {
        match direction {
            Direction::Long => to_measure_price(price, &self.price_max),
            _ => to_measure_price(price, &self.price_min) * -1.0,
        }
    }

    pub fn format_log(&self, name: &str, price: &f64) -> String {
        let activate_changes = self
            .activate_price_changes
            .iter()
            .map(|v| format!("{v:.2}%"))
            .collect::<Vec<_>>()
            .join("|");
        let rollback_changes = self
            .rollback_price_changes
            .iter()
            .map(|v| format!("{v:.2}%"))
            .collect::<Vec<_>>()
            .join("|");

        format!(
            "{name}: {:?} price:{:.2}% Activate: {:.2}% :: {activate_changes} Rollback: {rollback_changes}",
            self.status,
            to_measure_price(&self.price_min, &self.price_max),
            to_measure_price(&self.activate_price, price),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlgoSuggestionTradeStatus {
    Normal,
    FastTrade,
}

impl AlgoSuggestionTradeStatus {
    pub fn is_fast(&self) -> bool {
        self == &Self::FastTrade
    }

    pub fn is_normal(&self) -> bool {
        self == &Self::Normal
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestionInfo {
    pub time: i64,
    pub system_time: i64,
    pub symbol: String,
    pub symbol_id: i32,
    pub trade_group_id: i32,
    pub status: AlgoSuggestionTradeStatus,
    pub price: f64,
    pub candle: SuggestionInfoCandle,
    #[serde(deserialize_with = "de_int_key", default)]
    pub klines: HashMap<i64, SuggestionKline>,
    #[serde(rename = "fc")]
    pub fast_candle: Option<SuggestionInfoCandle>,
    pub direction: Direction,
    pub indicator: SuggestionInfoIndicator,
    #[serde(deserialize_with = "de_int_key", default)]
    pub movements: HashMap<i64, LineConsolidationMoveSuggestion>,
    #[serde(deserialize_with = "de_int_key", default)]
    pub waves: HashMap<i64, SuggestionInfoWave>,
    pub order_book: Option<SuggestionInfoOrderBook>,
    pub oi: Option<SuggestionInfoOpenInterest>,
}

// Backward-compat aliases used by existing host / module code.
pub type ModuleSugInfo = SuggestionInfo;
pub type SugIndicator = SuggestionInfoIndicator;
pub type MovementStatusLite = LineConsolidationMoveStatus;
pub type MovementLite = LineConsolidationMoveSuggestion;
