//! HEADERPORT0-S0: disconnected port-aware draft/body/finalizer vocabulary.
//!
//! These request types describe the future thin sibling entrypoints without
//! duplicating the existing lowering implementation.  S0 intentionally has no
//! lowering implementation or production caller; P0 will thread the request
//! through recursive body descent and short-lived header loans.

use crate::ast::ASTNode;

/// Which legacy body driver a port-aware draft builder must use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum PortAwareBodyKindV1 {
    Static,
    Instance,
}

/// Owned request for one port-aware function/method body lowering.
///
/// The request carries syntax only.  It does not carry mutable lowering state,
/// collector state, a header view, metadata, or publication state.
#[derive(Debug)]
pub(in crate::mir::builder) struct PortAwareFunctionBodyRequestV1 {
    pub(in crate::mir::builder) body_kind: PortAwareBodyKindV1,
    pub(in crate::mir::builder) body: Vec<ASTNode>,
}

impl PortAwareFunctionBodyRequestV1 {
    pub(in crate::mir::builder) fn new(body_kind: PortAwareBodyKindV1, body: Vec<ASTNode>) -> Self {
        Self { body_kind, body }
    }
}

/// Read-only finalizer input after the port-aware body loan ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct PortAwareFinalizerRequestV1 {
    pub(in crate::mir::builder) returns_value: bool,
}

impl PortAwareFinalizerRequestV1 {
    pub(in crate::mir::builder) fn new(returns_value: bool) -> Self {
        Self { returns_value }
    }
}

/// Future thin sibling surface for a port-aware draft builder.
///
/// The associated types keep this protocol independent of the current
/// builder and function storage.  P0 supplies the one real
/// implementation and threads the same recursive port through body descent.
pub(in crate::mir::builder) trait PortAwareFunctionDraftSurfaceV1 {
    type Port;
    type Header;
    type Draft;

    fn build_static_method_draft_with_port_v1(
        &mut self,
        port: &mut Self::Port,
        request: PortAwareFunctionBodyRequestV1,
    ) -> Result<Self::Draft, String>;

    fn build_instance_method_draft_with_port_v1(
        &mut self,
        port: &mut Self::Port,
        request: PortAwareFunctionBodyRequestV1,
    ) -> Result<Self::Draft, String>;

    fn lower_function_body_with_port_v1(
        &mut self,
        port: &mut Self::Port,
        request: PortAwareFunctionBodyRequestV1,
    ) -> Result<(), String>;

    fn lower_method_body_with_port_v1(
        &mut self,
        port: &mut Self::Port,
        request: PortAwareFunctionBodyRequestV1,
    ) -> Result<(), String>;

    fn finalize_function_draft_with_headers(
        &mut self,
        headers: &Self::Header,
        request: PortAwareFinalizerRequestV1,
    ) -> Result<Self::Draft, String>;
}
