use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

pub const MODULE_ABI_VERSION: u32 = 2;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModuleOrderSide {
    #[default]
    #[serde(rename = "Buy")]
    Buy,
    #[serde(rename = "Sell")]
    Sell,
}

impl ModuleOrderSide {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
        }
    }
}

impl fmt::Display for ModuleOrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModuleOrderSide {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Buy" => Ok(Self::Buy),
            "Sell" => Ok(Self::Sell),
            other => Err(format!("invalid ModuleOrderSide: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModuleExchangeOrderSide {
    #[default]
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl ModuleExchangeOrderSide {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}

impl fmt::Display for ModuleExchangeOrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModuleExchangeOrderSide {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "BUY" => Ok(Self::Buy),
            "SELL" => Ok(Self::Sell),
            other => Err(format!("invalid ModuleExchangeOrderSide: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleOrderStatus {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "PartiallyFilled")]
    PartiallyFilled,
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "PendingTrigger")]
    PendingTrigger,
}

impl ModuleOrderStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::New => "New",
            Self::PartiallyFilled => "PartiallyFilled",
            Self::Filled => "Filled",
            Self::Expired => "Expired",
            Self::Canceled => "Canceled",
            Self::PendingTrigger => "PendingTrigger",
        }
    }
}

impl fmt::Display for ModuleOrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModuleOrderStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "New" => Ok(Self::New),
            "PartiallyFilled" => Ok(Self::PartiallyFilled),
            "Filled" => Ok(Self::Filled),
            "Expired" => Ok(Self::Expired),
            "Canceled" => Ok(Self::Canceled),
            "PendingTrigger" => Ok(Self::PendingTrigger),
            other => Err(format!("invalid ModuleOrderStatus: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModuleOrderType {
    #[default]
    #[serde(rename = "Market")]
    Market,
    #[serde(rename = "Limit")]
    Limit,
}

impl ModuleOrderType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Market => "Market",
            Self::Limit => "Limit",
        }
    }
}

impl fmt::Display for ModuleOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ModuleOrderType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Market" => Ok(Self::Market),
            "Limit" => Ok(Self::Limit),
            other => Err(format!("invalid ModuleOrderType: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleEvent {
    /// Sent once after the WASM process starts so it can declare the
    /// historical kline data required before normal initialization.
    Requirements,
    /// Sent once after requested data has been prepared. `klines` and
    /// `internal_indicators` contain the requested histories. Modules calculate
    /// TA privately from klines; the host does not calculate dynamic indicators.
    Init,
    /// A newly closed kline is available for the listed timeframes. The
    /// `klines` field contains only the new rows; modules can apply them to
    /// `KlineHistory` without receiving the full history on every update.
    KlineUpdate {
        timeframes: Vec<i32>,
    },
    /// Reserved for future module-requested dynamic indicators. Unlike
    /// `InternalIndicators`, this event is not for fixed platform schemas.
    /// The host does not currently emit it.
    Indicators {
        timeframes: Vec<i64>,
    },
    /// Incremental update from the platform's internal, well-known indicators,
    /// carried in `ModuleInput::internal_indicators`. Only new or revised
    /// requested records are sent. These have fixed typed schemas; for example,
    /// NTPS has value/min/max fields. This is separate from future dynamic
    /// module-requested `Indicators`.
    InternalIndicators {
        timeframes: Vec<i64>,
    },
    /// A new public market-decision snapshot is available in
    /// `ModuleInput::market_decision`.
    MarketDecision,
    Signal {
        direction: Direction,
        code: String,
        #[serde(default)]
        values: HashMap<String, serde_json::Value>,
    },
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
        order_side: ModuleExchangeOrderSide,
        role: OrderRole,
        status: ModuleOrderStatus,
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
    /// Raw string value emitted by indicator data or suggestion metadata.
    /// Use [`ModuleIndicatorValue::as_str`] and [`ModuleIndicatorValue::parse`]
    /// to convert it into a typed enum.
    String(String),
    Direction(Direction),
    Cross(ModuleIndicatorCross),
}

impl ModuleIndicatorValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    pub fn parse<T: FromStr>(&self) -> Option<T> {
        self.as_str()?.parse().ok()
    }
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

/// Lightweight WASM ABI representation of the platform market-decision
/// contract. These local wire types keep strategy modules independent of the
/// host's market-decision service dependencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleMarketDecision {
    pub time: i64,
    #[serde(default)]
    pub system_time: i64,
    pub symbol_key: String,
    pub price: f64,
    pub candle: ModuleMarketCandle,
    #[serde(default)]
    pub fc: Option<ModuleMarketCandle>,
    #[serde(default)]
    pub ntp_5s: i64,
    #[serde(default)]
    pub tdt: Option<i64>,
    #[serde(default)]
    pub order_book: Option<ModuleMarketOrderBook>,
    #[serde(default)]
    pub oi: Option<ModuleMarketOpenInterest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleMarketCandle {
    pub min: f64,
    pub max: f64,
    pub enter: f64,
    pub exit: f64,
    pub qty: f64,
    pub asset: f64,
    pub qtym: f64,
    pub masset: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleMarketOrderBookLevel {
    pub price: f64,
    pub quantity: f64,
    pub total_amount: f64,
    pub filled: Option<f64>,
    pub time_start: i64,
    pub duration: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleMarketOrderBook {
    pub asks: Vec<ModuleMarketOrderBookLevel>,
    pub bids: Vec<ModuleMarketOrderBookLevel>,
    pub ask_amount: f64,
    pub ask_price: Option<f64>,
    pub bid_amount: f64,
    pub bid_price: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleMarketOpenInterest {
    pub usd: f64,
    pub change: f64,
    pub change_qty: f64,
    pub avg: f64,
}

/// Normalized OHLCV kline passed to a strategy module. `time` is the candle
/// open timestamp in milliseconds since the Unix epoch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleKline {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub qty_buy: f64,
    pub qty_sell: f64,
    pub asset_buy: f64,
    pub asset_sell: f64,
}

/// Timeframe seconds -> candles ordered oldest to newest.
pub type ModuleKlineSnapshot = BTreeMap<i32, Vec<ModuleKline>>;

/// Fixed schema for the platform's known NTPS indicator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalIndicator {
    Ntps { value: f64, min: f64, max: f64 },
}

/// Fixed platform indicator kinds that modules may request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalIndicatorKind {
    Ntps,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleInternalIndicatorRecord {
    /// Aggregation timestamp in milliseconds since the Unix epoch.
    pub time: i64,
    pub indicator: InternalIndicator,
}

/// Timeframe seconds -> internal indicator records, oldest to newest.
pub type ModuleInternalIndicatorSnapshot = BTreeMap<i64, Vec<ModuleInternalIndicatorRecord>>;

/// Fixed internal indicator kind -> timeframe seconds -> required record count.
pub type ModuleInternalIndicatorRequirements = BTreeMap<InternalIndicatorKind, BTreeMap<i64, u32>>;

/// Data requested by a module during the `Requirements` handshake.
///
/// Modules currently request klines and calculate technical indicators
/// privately with their TA library. Well-known platform indicators use the
/// separate `InternalIndicators` event and fixed schemas. A future dynamic
/// `Indicators` feature should use an explicit typed request with timeframe
/// and calculation parameters (for example, MRC settings).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ModuleDataRequirements {
    /// ABI understood by the module during this handshake.
    pub abi_version: u32,
    /// Timeframe seconds -> number of most recent klines required.
    pub klines: BTreeMap<i32, u32>,
    /// Fixed platform indicator -> timeframe seconds -> required record count.
    #[serde(default)]
    pub internal_indicators: ModuleInternalIndicatorRequirements,
}

impl Default for ModuleDataRequirements {
    fn default() -> Self {
        Self {
            abi_version: MODULE_ABI_VERSION,
            klines: BTreeMap::new(),
            internal_indicators: BTreeMap::new(),
        }
    }
}

impl ModuleDataRequirements {
    pub fn validate(&self) -> Result<(), String> {
        if self.abi_version != MODULE_ABI_VERSION {
            return Err(format!(
                "module ABI version {} is unsupported; host requires {}",
                self.abi_version, MODULE_ABI_VERSION
            ));
        }
        let mut requested_bytes = 0usize;
        for (timeframe_seconds, count) in &self.klines {
            if *timeframe_seconds < 60
                || *timeframe_seconds > 86_400
                || *timeframe_seconds % 60 != 0
            {
                return Err(format!(
                    "unsupported kline timeframe {timeframe_seconds}; expected a whole number of minutes up to 24 hours"
                ));
            }
            if *count == 0 || *count > 5_000 {
                return Err(format!(
                    "kline count for timeframe {timeframe_seconds} must be between 1 and 5000"
                ));
            }
            requested_bytes = requested_bytes
                .checked_add(*count as usize * std::mem::size_of::<ModuleKline>())
                .ok_or_else(|| "kline request size overflowed".to_owned())?;
        }
        for timeframes in self.internal_indicators.values() {
            for (timeframe_seconds, count) in timeframes {
                if *timeframe_seconds < 60
                    || *timeframe_seconds > 86_400
                    || *timeframe_seconds % 60 != 0
                {
                    return Err(format!(
                        "unsupported internal-indicator timeframe {timeframe_seconds}; expected a whole number of minutes up to 24 hours"
                    ));
                }
                if *count == 0 || *count > 5_000 {
                    return Err(format!(
                        "internal-indicator count for timeframe {timeframe_seconds} must be between 1 and 5000"
                    ));
                }
                requested_bytes = requested_bytes
                    .checked_add(
                        *count as usize * std::mem::size_of::<ModuleInternalIndicatorRecord>(),
                    )
                    .ok_or_else(|| "module data request size overflowed".to_owned())?;
            }
        }
        if requested_bytes > 100 * 1024 * 1024 {
            return Err("module data request exceeds the 100 MiB cache budget".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInput {
    pub event: ModuleEvent,
    pub price: f64,
    /// Latest market-decision snapshot. Older hosts can omit this field.
    #[serde(default)]
    pub market_decision: Option<ModuleMarketDecision>,
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
    /// Payload for the future dynamic `ModuleEvent::Indicators` feature.
    /// Current bot and backtest hosts leave it empty.
    pub indicators: BTreeMap<i64, Vec<HashMap<String, HashMap<String, ModuleIndicatorValue>>>>,
    /// Typed, well-known platform indicator data. Init receives the requested
    /// history; InternalIndicators events receive only new or revised records.
    #[serde(default)]
    pub internal_indicators: ModuleInternalIndicatorSnapshot,
    /// Requested timeframe seconds -> klines ordered oldest to newest. Init
    /// receives the full requested history; KlineUpdate receives only rows
    /// added or revised since the previous update.
    #[serde(default)]
    pub klines: ModuleKlineSnapshot,
    pub positions: ModulePositions,
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
    #[serde(default)]
    pub order_type: ModuleOrderType,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub note: String,
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
    /// Human-readable note attached to the order.
    ///
    /// Omit this field to leave the host note empty.
    #[serde(default)]
    pub note: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleAlert {
    pub message: String,
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
    pub alerts: Vec<ModuleAlert>,
    #[serde(default)]
    pub stop_bot: bool,
    pub state: Option<serde_json::Value>,
    /// Kline history requested by the module during `Requirements`.
    #[serde(default)]
    pub data_requirements: ModuleDataRequirements,
    #[serde(default)]
    pub debug: String,
    /// Non-empty when the module encountered an error processing the event.
    /// The host logs this at ERROR level.  The module should still return a
    /// valid (possibly empty) output — this field is informational.
    #[serde(default)]
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_indicator_value_string_parse_helpers() {
        let value = ModuleIndicatorValue::String("Filled".to_string());

        assert_eq!(value.as_str(), Some("Filled"));
        assert_eq!(
            value.parse::<ModuleOrderStatus>(),
            Some(ModuleOrderStatus::Filled)
        );
        assert_eq!(value.parse::<ModuleOrderType>(), None);
    }

    #[test]
    fn module_open_position_order_type_deserializes_exact_value() {
        let json = r#"{
            "direction":"LONG",
            "amount":1.0,
            "qty":null,
            "enter_price":null,
            "order_type":"Limit",
            "take_profit":null,
            "stop_loss":null,
            "note":"n"
        }"#;

        let position: ModuleOpenPosition = serde_json::from_str(json).unwrap();

        assert_eq!(position.order_type, ModuleOrderType::Limit);
        assert_eq!(position.order_type.as_str(), "Limit");
    }

    #[test]
    fn module_order_update_serializes_typed_fields() {
        let event = ModuleEvent::OrderUpdate {
            direction: Direction::Long,
            order_side: ModuleExchangeOrderSide::Buy,
            role: OrderRole::Entry,
            status: ModuleOrderStatus::Filled,
            fill_price: 1.23,
            filled_qty: 4.56,
            mark: Some("m1".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();

        assert!(json.contains(r#""order_update""#));
        assert!(json.contains(r#""order_side":"BUY""#));
        assert!(json.contains(r#""status":"Filled""#));
    }
}
