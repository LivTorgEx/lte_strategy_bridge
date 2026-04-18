use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::sug_info::ModuleSugInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,
    #[serde(rename = "BOTH")]
    Both,
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
pub struct ModuleInput {
    pub event: ModuleEvent,
    pub price: f64,
    pub symbol: String,
    pub max_amount: f64,
    pub leverage: i32,
    pub indicators: BTreeMap<i64, HashMap<String, HashMap<String, f64>>>,
    pub positions: ModulePositions,
    pub sug_info: Option<ModuleSugInfo>,
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
    #[serde(default)]
    pub stop_bot: bool,
    pub state: Option<serde_json::Value>,
    #[serde(default)]
    pub debug: String,
}
