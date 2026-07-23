//! RAW-SOURCE0-LOWER0-S0: one disconnected Raw child-draft owner.
//!
//! This is the first source-to-draft seam.  It consumes the compiler-owned
//! Raw package, opens one candidate session/shell/collector/ledger, and proves
//! one source-derived child admission.  Root completion and public ingress are
//! deliberately outside this module.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::compiler::raw_source_binding::{
    RawSourceContinuationV1, SourceBoundRawPackageV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use crate::mir::{MirBuilder, MirModule};

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_invocation_owner_chain::InvocationBranded;
use super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringPortChildErrorV1, ModuleLoweringInvocationV1,
};
use super::module_lowering_invocation_state::ModuleLoweringInvocationStateV1;
use super::module_lowering_shell::ModuleLoweringShellV1;
use super::raw_expansion_receipt_ledger::{
    AbortedRawExpansionReceiptLedgerV1, RawExpansionAbortReasonV1,
    RawExpansionDraftRequestV1, RawExpansionDraftRoleV1, RawExpansionReceiptLedgerErrorV1,
    RawExpansionReceiptLedgerV1,
};
use super::raw_source_projection::OwnedRawSourceV1;

#[derive(Debug)]
enum RawDraftLedgerStateV1 {
    Open(RawExpansionReceiptLedgerV1),
    Aborted(AbortedRawExpansionReceiptLedgerV1),
}

#[derive(Debug)]
struct RawChildWorkRequestV1 {
    symbol: Box<str>,
    arity: usize,
    box_name: Box<str>,
    method_name: Box<str>,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl RawChildWorkRequestV1 {
    fn from_static_method(
        box_name: &str,
        method_name: &str,
        declaration: &ASTNode,
    ) -> Result<Self, RawDraftInvocationErrorV1> {
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            is_static,
            ..
        } = declaration
        else {
            return Err(RawDraftInvocationErrorV1::SourceShape(
                "static child is not a function declaration",
            ));
        };
        if !is_static {
            return Err(RawDraftInvocationErrorV1::SourceShape(
                "S0 requires a static child",
            ));
        }
        let arity = params.len();
        Ok(Self {
            symbol: crate::mir::naming::encode_static_method(box_name, method_name, arity)
                .into_boxed_str(),
            arity,
            box_name: box_name.into(),
            method_name: method_name.into(),
            params: params.clone(),
            param_decls: param_decls.clone(),
            return_type_name: return_type_name.clone(),
            body: body.clone(),
            uses: uses.clone(),
            attrs: attrs.clone(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawDraftInvocationErrorV1 {
    NoStaticChild,
    SourceShape(&'static str),
    Request(RawExpansionReceiptLedgerErrorV1),
    Child(ModuleLoweringPortChildErrorV1),
    Ledger(RawExpansionReceiptLedgerErrorV1),
}

impl std::fmt::Display for RawDraftInvocationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_draft_s0] {self:?}")
    }
}

impl std::error::Error for RawDraftInvocationErrorV1 {}

#[derive(Debug)]
pub(in crate::mir) struct RawDraftInvocationV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawSourceContinuationV1,
    session: ModuleBuilderInvocationSessionV1,
    state: ModuleLoweringInvocationStateV1,
    ledger: RawDraftLedgerStateV1,
    _seal: RawDraftInvocationSealV1,
}

#[derive(Debug)]
struct RawDraftInvocationSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawDraftChildReceiptV1 {
    brand: ModuleInvocationBrandV1,
    symbol: Box<str>,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    _seal: RawDraftChildReceiptSealV1,
}

#[derive(Debug)]
struct RawDraftChildReceiptSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawDraftChildStepV1 {
    owner: RawDraftInvocationV1,
    receipt: RawDraftChildReceiptV1,
    _seal: RawDraftChildStepSealV1,
}

#[derive(Debug)]
struct RawDraftChildStepSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawDraftInvocationV1 {
    owner: RawDraftInvocationV1,
    error: RawDraftInvocationErrorV1,
    _seal: RejectedRawDraftInvocationSealV1,
}

#[derive(Debug)]
struct RejectedRawDraftInvocationSealV1;

impl RawDraftInvocationV1 {
    pub(in crate::mir) fn open(
        package: SourceBoundRawPackageV1,
        current: &MirBuilder,
    ) -> Self {
        let (token, source, continuation, config, module_name) = package.into_parts();
        let session = ModuleBuilderInvocationSessionV1::open_for_token(&token, current, config);
        let shell = ModuleLoweringShellV1::from_empty_module(MirModule::new(module_name.to_string()))
            .expect("Raw S0 opens an empty module shell");
        let collector = ModuleDraftCollectorV1::with_brand(token.brand());
        let state = ModuleLoweringInvocationStateV1::new(shell, collector);
        let ledger = RawExpansionReceiptLedgerV1::new_for_token(
            &token,
            continuation.callable_main(),
        );
        Self {
            token,
            source,
            continuation,
            session,
            state,
            ledger: RawDraftLedgerStateV1::Open(ledger),
            _seal: RawDraftInvocationSealV1,
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    /// S0 source traversal: one PLAN0-projected static child only.  App-wide
    /// declaration inventory and Script root completion are later rows.
    fn first_static_child(&self) -> Result<RawChildWorkRequestV1, RawDraftInvocationErrorV1> {
        let Some(locator) = self.source.projection().first_static_child() else {
            return Err(RawDraftInvocationErrorV1::NoStaticChild);
        };
        let ASTNode::Program { statements, .. } = self.source.ast() else {
            return Err(RawDraftInvocationErrorV1::SourceShape(
                "Raw source root is not a program",
            ));
        };
        let declaration = statements.iter().find_map(|statement| {
            let ASTNode::BoxDeclaration {
                name,
                methods,
                is_static: true,
                ..
            } = statement
            else {
                return None;
            };
            (name == locator.box_name())
                .then(|| methods.get(locator.method_name()))
                .flatten()
        });
        let declaration = declaration.ok_or(RawDraftInvocationErrorV1::SourceShape(
            "PLAN0 static child locator has no AST declaration",
        ))?;
        let request = RawChildWorkRequestV1::from_static_method(
            locator.box_name(),
            locator.method_name(),
            declaration,
        )?;
        if request.symbol.as_ref() != locator.symbol() || request.arity != locator.arity() {
            return Err(RawDraftInvocationErrorV1::SourceShape(
                "PLAN0 static child locator drift",
            ));
        }
        Ok(request)
    }

    pub(in crate::mir::builder) fn lower_first_static_child(
        self,
    ) -> Result<RawDraftChildStepV1, RejectedRawDraftInvocationV1> {
        let request = match self.first_static_child() {
            Ok(request) => request,
            Err(error) => return Err(self.reject(error)),
        };
        let RawDraftInvocationV1 {
            token,
            source,
            continuation,
            mut session,
            state,
            ledger,
            _seal,
        } = self;
        let RawDraftLedgerStateV1::Open(mut ledger) = ledger else {
            let owner = Self {
                token,
                source,
                continuation,
                session,
                state,
                ledger,
                _seal,
            };
            return Err(owner.reject(RawDraftInvocationErrorV1::Ledger(
                RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
            )));
        };
        let raw_request = match RawExpansionDraftRequestV1::legacy_discovered(
            RawExpansionDraftRoleV1::StaticMethod,
            request.symbol.clone(),
            request.arity,
        ) {
            Ok(request) => request,
            Err(error) => {
                let owner = Self {
                    token,
                    source,
                    continuation,
                    session,
                    state,
                    ledger: RawDraftLedgerStateV1::Open(ledger),
                    _seal,
                };
                return Err(owner.reject(RawDraftInvocationErrorV1::Request(error)));
            }
        };
        let reservation = match ledger.reserve(raw_request) {
            Ok(reservation) => reservation,
            Err(error) => {
                let owner = Self {
                    token,
                    source,
                    continuation,
                    session,
                    state,
                    ledger: RawDraftLedgerStateV1::Open(ledger),
                    _seal,
                };
                return Err(owner.reject(RawDraftInvocationErrorV1::Ledger(error)));
            }
        };

        let (shell, collector, _root) = state.into_parts();
        let mut invocation = ModuleLoweringInvocationV1::with_shell_collector(
            session.builder_mut(),
            shell,
            collector,
        );
        let admission = LegacyChildDraftAdmissionV1::legacy_symbol(
            request.symbol.to_string(),
            request.arity,
        );
        let child_result = invocation.with_module_port(|builder, port| {
            let mut child_port = super::recursive_child_lowering::RawInvocationChildPortV1::new(port);
            child_port.complete_static_box_method_branded(
                builder,
                admission,
                request.symbol.to_string(),
                request.params,
                request.param_decls,
                request.return_type_name,
                request.body,
                request.uses,
                request.attrs,
            )
        });
        let state = invocation.into_state();
        let brand = token.brand();
        let receipt = match child_result {
            Ok(receipt) => receipt,
            Err(error) => {
                let aborted = ledger
                    .abort(reservation, RawExpansionAbortReasonV1::Primary)
                    .unwrap_or_else(|_| panic!("Raw S0 reservation abort must be valid"));
                let owner = Self {
                    token,
                    source,
                    continuation,
                    session,
                    state,
                    ledger: RawDraftLedgerStateV1::Aborted(aborted),
                    _seal,
                };
                return Err(owner.reject(RawDraftInvocationErrorV1::Child(error)));
            }
        };
        if let Err(error) = ledger.complete_branded(reservation, &receipt) {
            let owner = Self {
                token,
                source,
                continuation,
                session,
                state,
                ledger: RawDraftLedgerStateV1::Open(ledger),
                _seal,
            };
            return Err(owner.reject(RawDraftInvocationErrorV1::Ledger(error)));
        }
        let owner = Self {
            token,
            source,
            continuation,
            session,
            state,
            ledger: RawDraftLedgerStateV1::Open(ledger),
            _seal,
        };
        Ok(RawDraftChildStepV1 {
            owner,
            receipt: RawDraftChildReceiptV1 {
                brand,
                symbol: request.symbol,
                receipt,
                _seal: RawDraftChildReceiptSealV1,
            },
            _seal: RawDraftChildStepSealV1,
        })
    }

    fn reject(self, error: RawDraftInvocationErrorV1) -> RejectedRawDraftInvocationV1 {
        RejectedRawDraftInvocationV1 {
            owner: self,
            error,
            _seal: RejectedRawDraftInvocationSealV1,
        }
    }
}

impl RawDraftChildStepV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (RawDraftInvocationV1, RawDraftChildReceiptV1) {
        (self.owner, self.receipt)
    }
}

impl RawDraftChildReceiptV1 {
    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl RejectedRawDraftInvocationV1 {
    pub(in crate::mir::builder) fn error(&self) -> &RawDraftInvocationErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {}
}
