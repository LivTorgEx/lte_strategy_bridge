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
    Indicators {
        timeframes: Vec<i64>,
    },
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
        #[serde(default)]
        mark: Option<String>,
    },
    /// A user-triggered named action dispatched from the UI
    /// (via `POST /bot/{id}/run_action`).  The module should react by
    /// emitting `open_positions`, `close_positions`, or other output.
    /// `values` carries any parameters the user filled in; it is empty
    /// for parameter-less actions.
    Action {
        name: String,
        #[serde(default)]
        values: HashMap<String, serde_json::Value>,
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

/// Trading rules for the current symbol, populated by the host from
/// the symbol model.  All fields are optional so old modules continue
/// to compile — use `unwrap_or_default()` / `unwrap_or(fallback)`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleSymbolInfo {
    /// Minimum price increment (e.g. 0.00001 for PENGU-USDT-SWAP).
    pub price_tick_size: f64,
    /// Minimum quantity increment per order.
    pub qty_step_size: f64,
    /// Minimum notional value per order in quote currency (USD).
    #[serde(default)]
    pub min_notional: Option<f64>,
    /// Minimum order quantity in contracts / coins.
    #[serde(default)]
    pub min_order_qty: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInput {
    pub event: ModuleEvent,
    pub price: f64,
    pub symbol: String,
    pub max_amount: f64,
    /// Symbol trading rules (tick size, lot size, min notional, …).
    #[serde(default)]
    pub symbol_info: ModuleSymbolInfo,
    /// Effective maximum position size in USD after compounding realised PnL.
    /// Formula: `auto_max_amount = max_amount + realised_pnl * reinvest_percent * leverage`
    /// Falls back to `max_amount` when no PnL has been accumulated yet.
    #[serde(default)]
    pub auto_max_amount: f64,
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
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub qty: Option<f64>,
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

/// Side of a standing limit order emitted by a WASM module.
///
/// `Buy` (default) — entry / DCA order that increases the position.
/// `Sell` — reduce-only partial-close order that decreases the position.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ModuleOrderSide {
    #[default]
    #[serde(rename = "Buy")]
    Buy,
    #[serde(rename = "Sell")]
    Sell,
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
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub qty: Option<f64>,
    /// Limit price at which the order should be placed.
    pub enter_price: f64,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    /// Unique stable identifier — used to create or update the order.
    pub mark: String,
    /// Exchange order side (`Buy` default for entries, `Sell` to reduce a long,
    /// `Buy` to reduce a short).
    #[serde(default)]
    pub order_side: ModuleOrderSide,
    /// When `true` the order must only be applied to an already-open position
    /// (reduce-only semantics).  It will never be buffered for a future position.
    /// Use this for partial-close orders so the host does not accidentally
    /// queue them as entry orders.
    #[serde(default)]
    pub reduce_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleClosePosition {
    pub direction: Direction,
    pub reason: String,
}

/// Amend fields of an already-open (or pending-limit) position.
///
/// Maps directly to the pro strategy `PositionStateChange::UpdateEnterPrice`
/// / `SetTakeProfit` / `SetStopLoss` paths, which call `update_order_info()`
/// on the exchange (amend-in-place, no cancel+replace).
///
/// All fields are optional — only non-`None` fields are applied.  If the
/// position does not yet exist in storage the update is silently dropped
/// (it cannot be applied before the exchange order is created).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUpdatePosition {
    pub direction: Direction,
    /// New limit entry price.  Triggers an exchange order amendment.
    pub enter_price: Option<f64>,
    /// New take-profit price.
    pub take_profit: Option<f64>,
    /// New stop-loss price.
    pub stop_loss: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleOutput {
    #[serde(default)]
    pub open_positions: Vec<ModuleOpenPosition>,
    #[serde(default)]
    pub close_positions: Vec<ModuleClosePosition>,
    /// Amend fields of an existing pending or open position (no cancel+replace).
    #[serde(default)]
    pub update_positions: Vec<ModuleUpdatePosition>,
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
    /// Non-empty when the module encountered an error processing the event.
    /// The host logs this at ERROR level.  The module should still return a
    /// valid (possibly empty) output — this field is informational.
    #[serde(default)]
    pub error: String,
}
