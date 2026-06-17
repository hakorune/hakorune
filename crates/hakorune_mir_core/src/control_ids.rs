/// Loop identity used by control-flow observation and lowering helpers.
///
/// This is a pure newtype. It must stay independent from MIR builder,
/// lowering, backend, and runtime logic.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct LoopId(pub u32);

/// Exit-edge identity used by control-flow observation helpers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExitEdgeId(pub u32);

/// Continue-edge identity used by control-flow observation helpers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ContinueEdgeId(pub u32);
