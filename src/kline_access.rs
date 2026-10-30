use std::collections::{BTreeMap, VecDeque};

use crate::abi::{ModuleKline, ModuleKlineSnapshot};

/// Per-module rolling kline history initialized from `ModuleEvent::Init` and
/// advanced with `ModuleEvent::KlineUpdate` payloads.
#[derive(Debug, Default)]
pub struct KlineHistory {
    data: BTreeMap<i32, VecDeque<ModuleKline>>,
    capacities: BTreeMap<i32, usize>,
}

impl KlineHistory {
    pub fn new(requirements: &BTreeMap<i32, u32>) -> Self {
        Self {
            data: BTreeMap::new(),
            capacities: requirements
                .iter()
                .map(|(timeframe, count)| (*timeframe, *count as usize))
                .collect(),
        }
    }

    /// Replace all histories with the full snapshot sent on module init.
    pub fn init(&mut self, snapshot: &ModuleKlineSnapshot) {
        self.data.clear();
        for (timeframe, rows) in snapshot {
            let capacity = self
                .capacities
                .get(timeframe)
                .copied()
                .unwrap_or(rows.len());
            let skip = rows.len().saturating_sub(capacity);
            let history = rows.iter().skip(skip).cloned().collect();
            self.data.insert(*timeframe, history);
        }
    }

    /// Apply newly added or revised candles, retaining the most recent
    /// requested number of rows for each timeframe.
    pub fn update(&mut self, snapshot: &ModuleKlineSnapshot) {
        for (timeframe, rows) in snapshot {
            let capacity = self.capacities.get(timeframe).copied().unwrap_or(0);
            if capacity == 0 {
                continue;
            }
            let history = self.data.entry(*timeframe).or_default();
            for row in rows {
                match history.back() {
                    Some(last) if row.time > last.time => history.push_back(row.clone()),
                    Some(last) if row.time == last.time => {
                        if let Some(last) = history.back_mut() {
                            *last = row.clone();
                        }
                    }
                    _ => {
                        if let Some(existing) =
                            history.iter_mut().find(|item| item.time == row.time)
                        {
                            *existing = row.clone();
                        }
                    }
                }
            }
            while history.len() > capacity {
                history.pop_front();
            }
        }
    }

    /// Return candles in chronological order.
    pub fn get(&self, timeframe_seconds: i32) -> Option<&VecDeque<ModuleKline>> {
        self.data.get(&timeframe_seconds)
    }

    pub fn snapshot(&self) -> ModuleKlineSnapshot {
        self.data
            .iter()
            .map(|(timeframe, rows)| (*timeframe, rows.iter().cloned().collect()))
            .collect()
    }

    /// Return one candle by offset from the newest row (`0` is newest).
    pub fn get_prev(&self, timeframe_seconds: i32, offset: usize) -> Option<&ModuleKline> {
        self.data.get(&timeframe_seconds).and_then(|rows| {
            rows.len()
                .checked_sub(offset + 1)
                .and_then(|index| rows.get(index))
        })
    }
}
