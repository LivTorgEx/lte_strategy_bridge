# TODO — lte_strategy_bridge

## Post-MVP: Selective Indicator Request

**Goal**: Minimize ModuleInput payload size by allowing strategies to request only specific indicators.

**Current State**: 
- `flatten_all_indicators()` sends ALL indicators for ALL timeframes (10 candles each)
- This is comprehensive but can be large for many indicators

**Future Enhancement**:
1. Add an optional `IndicatorRequest` enum/struct to `ModuleEvent` or ModuleInput initialization
2. Allow strategies to specify: `["mrc_200", "rsi_14", "ema_9"]` + required timeframes
3. Filter indicators in `flatten_all_indicators()` based on the request
4. Reduce payload size while maintaining full indicator access for modules that need it

**Impact**: Reduces bandwidth/serialization overhead; particularly valuable for high-frequency strategies.
