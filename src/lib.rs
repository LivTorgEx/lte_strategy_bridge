pub mod abi;
pub mod indicator_access;
pub mod kline_access;

pub use abi::MODULE_ABI_VERSION;
pub use abi::{
    Direction, InternalIndicator, InternalIndicatorKind, ModuleAlert, ModuleClosePosition,
    ModuleDataRequirements, ModuleEvent, ModuleExchangeOrderSide, ModuleInput,
    ModuleInternalIndicatorRecord, ModuleInternalIndicatorRequirements,
    ModuleInternalIndicatorSnapshot, ModuleKline, ModuleKlineSnapshot, ModuleMarketCandle,
    ModuleMarketDecision, ModuleMarketOpenInterest, ModuleMarketOrderBook,
    ModuleMarketOrderBookLevel, ModuleOpenPosition, ModuleOrderSide, ModuleOrderStatus,
    ModuleOrderType, ModuleOutput, ModulePlaceOrder, ModulePositionSummary, ModulePositions,
    ModuleSymbolInfo, ModuleUpdatePosition, OrderRole,
};
pub use indicator_access::{
    get_value, IndicatorField, IndicatorFieldKey, IndicatorHistory, IndicatorKey,
    IndicatorSnapshot, TimeframeSec,
};
pub use kline_access::KlineHistory;

/// Run a persistent stdin/stdout loop.
///
/// The host writes one JSON-encoded [`ModuleInput`] per line to stdin.  The
/// handler processes it and returns a [`ModuleOutput`].  This function
/// serialises the output as a single JSON line on stdout and loops.
///
/// The loop exits when stdin is closed (host drops the bridge).
///
/// Errors (parse failures, handler panics) are reported back to the host via
/// the `ModuleOutput::error` field so they surface in the bot's logs.
pub fn run_loop(mut handler: impl FnMut(ModuleInput) -> ModuleOutput) {
    use std::io::{self, BufRead, Write};
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.is_empty() {
            continue;
        }
        let input: ModuleInput = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let err_output = ModuleOutput {
                    error: format!("input parse error: {e}"),
                    ..Default::default()
                };
                let json = serde_json::to_string(&err_output)
                    .unwrap_or_else(|_| format!("{{\"error\":\"input parse error: {e}\"}}"));
                if writeln!(stdout, "{json}").is_err() {
                    break;
                }
                let _ = stdout.flush();
                continue;
            }
        };
        let output = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(input)))
        {
            Ok(out) => out,
            Err(panic) => {
                let msg = panic
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| panic.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "unknown panic".to_string());
                ModuleOutput {
                    error: format!("handler panicked: {msg}"),
                    ..Default::default()
                }
            }
        };
        let json = match serde_json::to_string(&output) {
            Ok(j) => j,
            Err(e) => {
                format!("{{\"error\":\"output serialise error: {e}\"}}")
            }
        };
        if writeln!(stdout, "{json}").is_err() {
            break;
        }
        let _ = stdout.flush();
    }
}
