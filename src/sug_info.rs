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
    pub symbol_key: String,
    pub status: AlgoSuggestionTradeStatus,
    pub price: f64,
    pub candle: SuggestionInfoCandle,
    #[serde(deserialize_with = "de_int_key", default)]
    pub klines: HashMap<i64, SuggestionKline>,
    #[serde(rename = "fc")]
    pub fast_candle: Option<SuggestionInfoCandle>,
    pub direction: Direction,
    pub indicator: SuggestionInfoIndicator,
    pub order_book: Option<SuggestionInfoOrderBook>,
    pub oi: Option<SuggestionInfoOpenInterest>,
}

// Backward-compat aliases used by existing host / module code.
pub type ModuleSugInfo = SuggestionInfo;
pub type SugIndicator = SuggestionInfoIndicator;
