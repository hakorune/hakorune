/// Language-wide obligation carried by a published opaque Dynamic carrier.
///
/// The producing semantic envelope decides whether a Normal result publishes
/// a carrier. Runtime payload kind does not alter this obligation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicCarrierLifecycleObligationV1 {
    EndExactlyOnceUnlessForwarded,
}
