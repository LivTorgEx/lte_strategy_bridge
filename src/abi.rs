use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::sug_info::SuggestionInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,
    #[serde(rename = "BOTH")]
    Both,
}

impl Direction {
    pub const ALL: [Direction; 3] = [Direction::Long, Direction::Short, Direction::Both];

    pub const fn index(&self) -> usize {
        match self {
            Direction::Long => 0,
            Direction::Short => 1,
            Direction::Both => 2,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Direction::Long,
            1 => Direction::Short,
            _ => Direction::Both,
        }
    }

    pub fn factor(&self) -> f64 {
        match self {
            Direction::Short => -1.0,
            _ => 1.0,
        }
    }

    pub fn oposite(&self) -> Self {
        match self {
            Direction::Short => Direction::Long,
            Direction::Long => Direction::Short,
            Direction::Both => Direction::Both,
        }
    }

    pub fn is_long(&self) -> bool {
        self == &Direction::Long
    }

    pub fn is_short(&self) -> bool {
        self == &Direction::Short
    }

    pub fn is_both(&self) -> bool {
        self == &Direction::Both
    }

    pub fn is_price_bigger(&self, current_price: &f64, price: &f64) -> bool {
        match self {
            Direction::Long => current_price < price,
            _ => current_price > price,
        }
    }

    pub fn from_value(value: f64) -> Self {
        if value > 0.0 {
            Direction::Long
        } else if value < 0.0 {
            Direction::Short
        } else {
            Direction::Both
        }
    }

    pub fn fix_between(&self, value: f64) -> f64 {
        match self {
            Direction::Long => value,
            _ => 1.0 - value,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Both
    }
}

impl From<f64> for Direction {
    fn from(value: f64) -> Self {
        Self::from_value(value)
    }
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        if value > 0 {
            Direction::Long
        } else if value < 0 {
            Direction::Short
        } else {
            Direction::Both
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleEvent {
    SugInfo,
    Indicators { timeframes: Vec<i64> },
    Signal,
    NewPosition {
        direction: Direction,
        entry_price: f64,
        qty: f64,
    },
    FinishPosition {
        direction: Direction,
        pnl: f64,
    },
    OrderUpdate {
        direction: Direction,
        order_side: String,
        role: OrderRole,
        status: String,
        fill_price: f64,
        filled_qty: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderRole {
    Entry,
    TakeProfit,
    StopLoss,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePositionSummary {
    pub entry_price: f64,
    pub notional: f64,
    pub pnl: f64,
    pub qty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePositions {
    pub long: Option<ModulePositionSummary>,
    pub short: Option<ModulePositionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleIndicatorValue {
    Float(f64),
    String(String),
    Direction(Direction),
    Cross(ModuleIndicatorCross),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleIndicatorCross {
    Cross,
    Body,
    Shadow,
    No,
    Above,
    Below,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInput {
    pub event: ModuleEvent,
    pub price: f64,
    pub symbol: String,
    pub max_amount: f64,
    pub leverage: i32,
    /// Indicators: tf_seconds → [all_candles] where each candle is
    /// [indicator_name → [field → typed value]]
    /// Values preserve original indicator formats (float/string/enums).
    pub indicators: BTreeMap<i64, Vec<HashMap<String, HashMap<String, ModuleIndicatorValue>>>>,
    pub positions: ModulePositions,
    pub sug_info: Option<SuggestionInfo>,
    pub state: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOpenPosition {
    pub direction: Direction,
    pub amount_ratio: f64,
    pub enter_price: Option<f64>,
    #[serde(default = "default_order_type")]
    pub order_type: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub note: String,
}

fn default_order_type() -> String {
    "Market".to_string()
}

/// A standing limit order placed by the module.
///
/// `mark` is the unique stable identifier for this order — the platform uses it
/// to create the order on first appearance and update the price/amounts on
/// subsequent ticks.  To cancel the order, include its `mark` in
/// `ModuleOutput::cancel_orders`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePlaceOrder {
    pub direction: Direction,
    pub amount_ratio: f64,
    /// Limit price at which the order should be placed.
    pub enter_price: f64,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    /// Unique stable identifier — used to create or update the order.
    pub mark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleClosePosition {
    pub direction: Direction,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleOutput {
    #[serde(default)]
    pub open_positions: Vec<ModuleOpenPosition>,
    #[serde(default)]
    pub close_positions: Vec<ModuleClosePosition>,
    /// Standing limit orders to create or update (matched by `mark`).
    #[serde(default)]
    pub place_orders: Vec<ModulePlaceOrder>,
    /// Marks of standing limit orders to cancel.
    #[serde(default)]
    pub cancel_orders: Vec<String>,
    #[serde(default)]
    pub stop_bot: bool,
    pub state: Option<serde_json::Value>,
    #[serde(default)]
    pub debug: String,
}
