use std::collections::{BTreeMap, HashMap};

use crate::abi::ModuleIndicatorValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeframeSec {
    Tf1m = 60,
    Tf5m = 300,
    Tf15m = 900,
    Tf30m = 1800,
    Tf1h = 3600,
    Tf4h = 14400,
}

impl TimeframeSec {
    pub const fn as_i64(self) -> i64 {
        self as i64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicatorKey {
    Psar,
    Natr14,
    Natr30,
    Ema9,
    Ema20,
    Ema21,
    Ema26,
    Ema200,
    Smi10,
    Smi25,
    Rsi14,
    Mfi14,
    Cci20,
    Lsma50,
    ChandelierExit,
    BollingerBands20,
    ATRTralling,
    Supertrend,
    ZigZagTrend,
    Stoch1413,
    Mrc200,
    Imbalance,
    Window,
    Wave,
    EmaCross926,
    Volume50,
    Smc,
    DPSignal,
    Ntps,
    Candle,
}

impl IndicatorKey {
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::Psar => "psar",
            Self::Natr14 => "natr_14",
            Self::Natr30 => "natr_30",
            Self::Ema9 => "ema_9",
            Self::Ema20 => "ema_20",
            Self::Ema21 => "ema_21",
            Self::Ema26 => "ema_26",
            Self::Ema200 => "ema_200",
            Self::Smi10 => "smi_10",
            Self::Smi25 => "smi_25",
            Self::Rsi14 => "rsi_14",
            Self::Mfi14 => "mfi_14",
            Self::Cci20 => "cci_20",
            Self::Lsma50 => "lsma_50",
            Self::ChandelierExit => "ce",
            Self::BollingerBands20 => "bb_20",
            Self::ATRTralling => "atrtralling",
            Self::Supertrend => "supertrend",
            Self::ZigZagTrend => "zigzagtrend",
            Self::Stoch1413 => "stoch_14,1,3",
            Self::Mrc200 => "mrc_200",
            Self::Imbalance => "imbalance",
            Self::Window => "window",
            Self::Wave => "wave",
            Self::EmaCross926 => "emacross_9,26",
            Self::Volume50 => "volume_50",
            Self::Smc => "smc",
            Self::DPSignal => "dpsignal",
            Self::Ntps => "ntps",
            Self::Candle => "candle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndicatorField {
    Value,
    MeanLine,
    UpInner,
    UpSmall,
    UpOuter,
    UpBig,
    DownInner,
    DownSmall,
    DownOuter,
    DownBig,
    CurrentCross,
    PrevCross,
    Open,
    High,
    Low,
    Close,
    Direction,
    BodyChange,
    Atr,
    Drop,
    Gain,
    Max,
    Min,
}

impl IndicatorField {
    pub const fn as_name(self) -> &'static str {
        match self {
            Self::Value => "Value",
            Self::MeanLine => "MeanLine",
            Self::UpInner => "UpInner",
            Self::UpSmall => "UpSmall",
            Self::UpOuter => "UpOuter",
            Self::UpBig => "UpBig",
            Self::DownInner => "DownInner",
            Self::DownSmall => "DownSmall",
            Self::DownOuter => "DownOuter",
            Self::DownBig => "DownBig",
            Self::CurrentCross => "CurrentCross",
            Self::PrevCross => "PrevCross",
            Self::Open => "Open",
            Self::High => "High",
            Self::Low => "Low",
            Self::Close => "Close",
            Self::Direction => "Direction",
            Self::BodyChange => "BodyChange",
            Self::Atr => "Atr",
            Self::Drop => "Drop",
            Self::Gain => "Gain",
            Self::Max => "Max",
            Self::Min => "Min",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndicatorFieldKey {
    pub timeframe: TimeframeSec,
    pub indicator: IndicatorKey,
    pub field: IndicatorField,
}

pub type IndicatorSnapshot =
    BTreeMap<i64, Vec<HashMap<String, HashMap<String, ModuleIndicatorValue>>>>;

pub fn get_value(indicators: &IndicatorSnapshot, key: IndicatorFieldKey) -> Option<&ModuleIndicatorValue> {
    indicators
        .get(&key.timeframe.as_i64())
        .and_then(|candles| candles.first())
        .and_then(|by_ind| by_ind.get(key.indicator.as_name()))
        .and_then(|fields| fields.get(key.field.as_name()))
}
