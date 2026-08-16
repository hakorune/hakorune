//! HEADERPORT0-S0: external owner for one module-lowering invocation.
//!
//! This vocabulary deliberately stays outside `MirBuilder` and
//! `CompilationContext`. It owns one unpublished-draft collector and
//! lends only a short-lived, read-only header port to an explicit lowering closure.

use super::calls::{
    CanonicalFunctionSessionErrorV1, LegacyFunctionPendingSessionV1, PendingFunctionSessionCloseV1,
};
use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::module_draft_collector::{
    CollectorReceiptBrandErrorV1, CompletedDraftSignatureViewV1, ModuleDraftCollectorV1,
};
use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1,
    PreparedFunctionDraftAdmissionV1,
};
use super::module_lowering_invocation_access::ModuleLoweringInvocationAccessPortV1;
use super::module_lowering_invocation_state::ModuleLoweringInvocationStateV1;
use super::module_lowering_shell::ModuleLoweringShellV1;
use crate::ast::ASTNode;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{FunctionSignature, MirBuilder, MirFunction};

#[path = "module_lowering_invocation_resolved_loan.rs"]
mod resolved_loan;

/// Owned canonical child identity before collector admission.
///
/// It carries no collector borrow, draft, Builder, or header view.  The
/// module port is the sole owner which turns it into a prepared admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ResolvedChildDraftAdmissionV1 {
    CanonicalResolvedOwner {
        owner: FunctionOwnerIdV1,
        symbol: String,
        arity: usize,
    },
}

impl ResolvedChildDraftAdmissionV1 {
    pub(in crate::mir::builder) fn canonical_resolved_owner(
        owner: FunctionOwnerIdV1,
        symbol: String,
        arity: usize,
    ) -> Self {
        Self::CanonicalResolvedOwner {
            owner,
            symbol,
            arity,
        }
    }

    pub(in crate::mir::builder) fn collector_parts(self) -> (FunctionDraftKeyV1, String, usize) {
        match self {
            Self::CanonicalResolvedOwner {
                owner,
                symbol,
                arity,
            } => (
                FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
                symbol,
                arity,
            ),
        }
    }

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::CanonicalResolvedOwner { owner, .. } => *owner,
        }
    }
}

/// Owned legacy child identity before collector admission.
///
/// Legacy lowering preserves replace-by-symbol behavior.  This request is not
/// Clone and cannot name a canonical owner, so a raw child cannot accidentally
/// enter the resolved-child duplicate policy.
#[derive(Debug)]
pub(in crate::mir::builder) struct LegacyChildDraftAdmissionV1 {
    symbol: String,
    arity: usize,
    _seal: LegacyChildDraftAdmissionSealV1,
}

#[derive(Debug)]
struct LegacyChildDraftAdmissionSealV1;

impl LegacyChildDraftAdmissionV1 {
    pub(in crate::mir::builder) fn legacy_symbol(symbol: String, arity: usize) -> Self {
        Self {
            symbol,
            arity,
            _seal: LegacyChildDraftAdmissionSealV1,
        }
    }

    pub(in crate::mir::builder) fn collector_parts(self) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            symbol,
            arity,
            _seal: _,
        } = self;
        (
            FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
            symbol,
            arity,
        )
    }
}

/// Failure while a port-owned resolved child completes before parent restore.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum ModuleLoweringPortChildErrorV1 {
    Session(CanonicalFunctionSessionErrorV1),
    Admission(ModuleDraftAdmissionErrorV1),
    ReceiptBrand(CollectorReceiptBrandErrorV1),
    PhysicalSignatureMismatch,
}

#[cfg(test)]
impl ModuleLoweringPortChildErrorV1 {
    pub(in crate::mir) const fn is_primary_session(&self) -> bool {
        matches!(
            self,
            Self::Session(CanonicalFunctionSessionErrorV1::Primary(_))
        )
    }
}

impl std::fmt::Display for ModuleLoweringPortChildErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session(error) => error.fmt(formatter),
            Self::Admission(error) => error.fmt(formatter),
            Self::ReceiptBrand(error) => error.fmt(formatter),
            Self::PhysicalSignatureMismatch => {
                write!(
                    formatter,
                    "[freeze:contract][mir/resolved-child/physical-signature-mismatch]"
                )
            }
        }
    }
}

impl std::error::Error for ModuleLoweringPortChildErrorV1 {}

/// Read-only completed-function header capability for one lowering closure.
///
/// The port borrows the invocation's collector. It owns neither a draft nor a
/// cache and cannot prepare admission or collect. Its lifetime prevents a
/// header view from surviving the closure that received it.
pub(in crate::mir::builder) struct LoweringHeaderPortV1<'collector> {
    view: &'collector dyn CompletedDraftSignatureViewV1,
    _seal: LoweringHeaderPortSealV1,
}

struct LoweringHeaderPortSealV1;

impl LoweringHeaderPortV1<'_> {
    pub(in crate::mir::builder) fn from_view(
        view: &dyn CompletedDraftSignatureViewV1,
    ) -> LoweringHeaderPortV1<'_> {
        LoweringHeaderPortV1 {
            view,
            _seal: LoweringHeaderPortSealV1,
        }
    }

    pub(in crate::mir::builder) fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
        self.view.signature(symbol)
    }

    pub(in crate::mir::builder) fn contains_symbol(&self, symbol: &str) -> bool {
        self.view.contains_symbol(symbol)
    }

    pub(in crate::mir::builder) fn symbol_count(&self) -> usize {
        self.view.symbol_count()
    }

    pub(in crate::mir::builder) fn visit_symbols(&self, visitor: &mut dyn FnMut(&str)) {
        self.view.visit_symbols(visitor);
    }
}

impl FunctionSignatureLookupV1 for LoweringHeaderPortV1<'_> {
    fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
        self.signature(symbol)
    }

    fn contains_symbol(&self, symbol: &str) -> bool {
        self.contains_symbol(symbol)
    }

    fn symbol_count(&self) -> usize {
        self.symbol_count()
    }

    fn visit_symbols(&self, visitor: &mut dyn FnMut(&str)) {
        self.visit_symbols(visitor)
    }
}

/// Stack-owned capability for one recursive module-lowering descent.
///
/// The port borrows only the invocation collector. It cannot retain a Builder,
/// so every lowering caller must pass its Builder explicitly. Header reads are
/// scoped to `with_headers`; a mutable admission can begin only after that
/// callback returns.
pub(in crate::mir::builder) struct ModuleLoweringPortV1<'collector> {
    collector: &'collector mut ModuleDraftCollectorV1,
    _seal: ModuleLoweringPortSealV1,
}

struct ModuleLoweringPortSealV1;

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn from_collector(
        collector: &mut ModuleDraftCollectorV1,
    ) -> ModuleLoweringPortV1<'_> {
        ModuleLoweringPortV1 {
            collector,
            _seal: ModuleLoweringPortSealV1,
        }
    }

    /// Lend the invocation-owned collector brand for one non-escaping use.
    /// The collector is the sole source; an unbranded invocation cannot enter
    /// a brand-sensitive admission boundary.
    pub(in crate::mir::builder) fn with_invocation_brand<R>(
        &self,
        observe: impl FnOnce(ModuleInvocationBrandV1) -> R,
    ) -> Result<R, CollectorReceiptBrandErrorV1> {
        let brand = self
            .collector
            .receipt_brand()
            .ok_or(CollectorReceiptBrandErrorV1::CollectorUnbranded)?;
        Ok(observe(brand))
    }

    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        let headers = LoweringHeaderPortV1 {
            view: &*self.collector,
            _seal: LoweringHeaderPortSealV1,
        };
        observe(&headers)
    }

    pub(in crate::mir::builder) fn take_static_result_publication_handoff(
        &mut self,
        declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
        caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        target: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) -> Result<
        crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1,
        crate::mir::callable_result_representation::StaticCallResultPublicationOwnerTakeErrorV1,
    > {
        self.collector
            .take_static_result_publication_handoff(declarations, caller, site, target)
    }

    pub(in crate::mir::builder) fn prepare_draft_admission(
        &mut self,
        key: FunctionDraftKeyV1,
        expected_symbol: String,
        expected_arity: usize,
        policy: DraftPublicationPolicyV1,
    ) -> Result<PreparedFunctionDraftAdmissionV1<'_>, ModuleDraftAdmissionErrorV1> {
        self.collector
            .prepare_admission(key, expected_symbol, expected_arity, policy)
    }

    /// Complete one resolved child through this exact invocation's collector.
    ///
    /// The port owns the only pairing between the pending child and admission:
    /// `capture -> prepare -> seal -> collect -> restore`.  No caller can
    /// supply a foreign prepared admission or observe the draft after parent
    /// restoration.
    pub(in crate::mir::builder) fn complete_resolved_child(
        &mut self,
        builder: &mut MirBuilder,
        admission: ResolvedChildDraftAdmissionV1,
        lower: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let (key, symbol, arity) = admission.collector_parts();
        let pending = builder
            .capture_resolved_function_pending_session_v1(&symbol, lower)
            .map_err(ModuleLoweringPortChildErrorV1::Session)?;
        pending.complete_before_restore(|draft| {
            let prepared = self
                .prepare_draft_admission(
                    key,
                    symbol,
                    arity,
                    DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                )
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?;
            prepared
                .seal(draft)
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?
                .collect();
            Ok(())
        })
    }

    /// Commit one already-captured resolved child after all body/header loans
    /// have ended.  S0 exposes the commit-only terminal but keeps it
    /// disconnected from production callers until the re-entrant cutover.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn commit_resolved_pending(
        &mut self,
        pending: PendingFunctionSessionCloseV1<'_>,
        admission: ResolvedChildDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let (key, symbol, arity) = admission.collector_parts();
        pending.complete_before_restore(|draft| {
            let prepared = self
                .prepare_draft_admission(
                    key,
                    symbol,
                    arity,
                    DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                )
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?;
            prepared
                .seal(draft)
                .map_err(ModuleLoweringPortChildErrorV1::Admission)?
                .collect();
            Ok(())
        })
    }

    /// Capture-only seam used by P0 to prove that collector mutation starts
    /// only after the child body and header loans have ended.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn capture_resolved_pending<'builder>(
        &self,
        builder: &'builder mut MirBuilder,
        function_name: &str,
        lower: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<PendingFunctionSessionCloseV1<'builder>, ModuleLoweringPortChildErrorV1> {
        builder
            .capture_resolved_function_pending_session_v1(function_name, lower)
            .map_err(ModuleLoweringPortChildErrorV1::Session)
    }

    /// Legacy counterpart to `capture_resolved_pending`; it owns no identity
    /// and cannot prepare or collect a draft.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn capture_legacy_pending<'builder>(
        &self,
        builder: &'builder mut MirBuilder,
        function_name: &str,
        body_snapshot: Vec<ASTNode>,
        lower: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
    ) -> Result<LegacyFunctionPendingSessionV1<'builder>, ModuleLoweringPortChildErrorV1> {
        builder
            .capture_legacy_function_pending_session_v1(function_name, body_snapshot, lower)
            .map_err(ModuleLoweringPortChildErrorV1::Session)
    }
}

/// The external owner of one module-lowering invocation.
///
/// It borrows the route's Builder and owns the invocation-local collector, but
/// is never stored in either of them. The only public transition in S0 loans a
/// read-only header capability. M0-T0 adds only one disconnected resolved
/// child terminal; production collection remains an I0 responsibility.
pub(in crate::mir::builder) struct ModuleLoweringInvocationV1<'builder> {
    builder: &'builder mut MirBuilder,
    state: ModuleLoweringInvocationStateV1,
    _seal: ModuleLoweringInvocationSealV1,
}

struct ModuleLoweringInvocationSealV1;

impl<'builder> ModuleLoweringInvocationV1<'builder> {
    pub(in crate::mir::builder) fn open(builder: &'builder mut MirBuilder) -> Self {
        Self::with_collector(builder, ModuleDraftCollectorV1::default())
    }

    /// Test and future invocation construction use the same owned collector.
    ///
    /// This does not expose a second store: ownership moves into the invocation
    /// and no collector reference is retained by the caller.
    pub(in crate::mir::builder) fn with_collector(
        builder: &'builder mut MirBuilder,
        collector: ModuleDraftCollectorV1,
    ) -> Self {
        let shell = ModuleLoweringShellV1::from_empty_module(super::MirModule::new(
            "disconnected-invocation".to_owned(),
        ))
        .expect("empty disconnected invocation shell");
        Self::with_shell_collector(builder, shell, collector)
    }

    pub(in crate::mir::builder) fn with_shell_collector(
        builder: &'builder mut MirBuilder,
        shell: ModuleLoweringShellV1,
        collector: ModuleDraftCollectorV1,
    ) -> Self {
        Self {
            builder,
            state: ModuleLoweringInvocationStateV1::new(shell, collector),
            _seal: ModuleLoweringInvocationSealV1,
        }
    }

    /// Lend one stack-owned lowering port to an explicit recursive descent.
    ///
    /// M0 keeps this disconnected while it proves raw recursive propagation.
    /// Future routes use the port to alternate header reads with child terminal
    /// collection without ambient collector lookup.
    pub(in crate::mir::builder) fn with_module_port<R>(
        &mut self,
        lower: impl for<'port> FnOnce(&mut MirBuilder, &'port mut ModuleLoweringPortV1<'port>) -> R,
    ) -> R {
        let builder = &mut *self.builder;
        let mut port = ModuleLoweringPortV1 {
            collector: self.state.collector_mut(),
            _seal: ModuleLoweringPortSealV1,
        };
        lower(builder, &mut port)
    }

    /// Lend one short-lived shell/header bundle for wiring-only construction.
    /// Production roots remain disconnected until the all-route I0 cutover.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn with_access_port<R>(
        &mut self,
        lower: impl for<'shell, 'collector> FnOnce(
            &mut MirBuilder,
            &mut ModuleLoweringInvocationAccessPortV1<'shell, 'collector>,
        ) -> R,
    ) -> R {
        let builder = &mut *self.builder;
        let (shell, collector) = self.state.split_shell_collector_mut();
        let mut access = ModuleLoweringInvocationAccessPortV1::new(shell, collector);
        lower(builder, &mut access)
    }

    /// Lend the same-owned collector header view to one explicit lowering step.
    ///
    /// When `lower` returns, the port is dropped before any later mutable
    /// collector transition can begin.
    pub(in crate::mir::builder) fn with_header_port<R>(
        &mut self,
        lower: impl for<'header> FnOnce(&mut MirBuilder, &'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        self.with_module_port(|builder, port| port.with_headers(|headers| lower(builder, headers)))
    }

    /// Begin the one future terminal transition after every header borrow ends.
    ///
    /// This is still disconnected in P0. Returning the prepared admission
    /// exclusively borrows the collector, so a caller cannot overlap header
    /// observation with seal/collect.
    pub(in crate::mir::builder) fn prepare_draft_admission(
        &mut self,
        key: FunctionDraftKeyV1,
        expected_symbol: String,
        expected_arity: usize,
        policy: DraftPublicationPolicyV1,
    ) -> Result<PreparedFunctionDraftAdmissionV1<'_>, ModuleDraftAdmissionErrorV1> {
        self.state
            .collector_mut()
            .prepare_admission(key, expected_symbol, expected_arity, policy)
    }

    pub(in crate::mir::builder) fn into_state(self) -> ModuleLoweringInvocationStateV1 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::{ModuleLoweringInvocationV1, ResolvedChildDraftAdmissionV1};
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
    };
    use crate::mir::resolved_semantics::{
        FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, VerifiedResolvedFunctionV1,
    };
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType,
    };
    use std::sync::Arc;

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.into(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn resolved_function(symbol: &str) -> Arc<VerifiedResolvedFunctionV1> {
        let declaration = ASTNode::FunctionDeclaration {
            name: symbol.to_string(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let view = FunctionSyntaxViewV1::from_ast(&declaration).unwrap();
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(view)
            .unwrap()
    }

    fn finish_resolved_authority(builder: &mut MirBuilder, function: &VerifiedResolvedFunctionV1) {
        let owner = function.owner();
        builder
            .function_state
            .resolved_binding_state
            .install(function)
            .unwrap();
        builder
            .function_state
            .resolved_binding_state
            .finish(owner)
            .unwrap();
    }

    fn collected_prefix() -> ModuleDraftCollectorV1 {
        let mut collector = ModuleDraftCollectorV1::default();
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("Prefix.f/1".into()),
                "Prefix.f/1".into(),
                1,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("Prefix.f/1", 1))
            .unwrap()
            .collect();
        collector
    }

    #[test]
    fn header_port_reads_only_the_invocation_owned_collected_prefix() {
        let mut builder = MirBuilder::new();
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(&mut builder, collected_prefix());

        invocation.with_header_port(|builder, headers| {
            assert!(headers.contains_symbol("Prefix.f/1"));
            assert_eq!(headers.signature("Prefix.f/1").unwrap().params.len(), 1);
            assert_eq!(headers.symbol_count(), 1);
            assert!(!headers.contains_symbol("missing/0"));

            let mut names = Vec::new();
            headers.visit_symbols(&mut |symbol| names.push(symbol.to_owned()));
            assert_eq!(names, ["Prefix.f/1"]);

            // The port is explicit: Builder mutation remains possible, but no
            // Builder field supplies completed-function header authority.
            assert_eq!(builder.next_value_id().0, 0);
        });
    }

    #[test]
    fn fresh_invocation_exposes_the_same_empty_prefix_as_a_new_module() {
        let mut builder = MirBuilder::new();
        let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 0);
            assert!(headers.signature("first/0").is_none());
            assert!(!headers.contains_symbol("first/0"));
        });
    }

    #[test]
    fn read_borrow_ends_before_prepared_admission_and_updates_the_same_prefix() {
        let mut builder = MirBuilder::new();
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(&mut builder, collected_prefix());

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 1);
            assert!(headers.contains_symbol("Prefix.f/1"));
        });

        invocation
            .prepare_draft_admission(
                FunctionDraftKeyV1::LegacySymbol("Later.g/0".into()),
                "Later.g/0".into(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("Later.g/0", 0))
            .unwrap()
            .collect();

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 2);
            assert!(headers.contains_symbol("Prefix.f/1"));
            assert!(headers.contains_symbol("Later.g/0"));
        });
    }

    #[test]
    fn rejected_post_read_admission_keeps_the_collected_prefix_unchanged() {
        let mut builder = MirBuilder::new();
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(&mut builder, collected_prefix());

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 1);
        });

        let duplicate = invocation.prepare_draft_admission(
            FunctionDraftKeyV1::LegacySymbol("Prefix.f/1".into()),
            "Prefix.f/1".into(),
            1,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        );
        assert!(duplicate.is_err());

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 1);
            assert_eq!(headers.signature("Prefix.f/1").unwrap().params.len(), 1);
        });
    }

    #[test]
    fn module_port_alternates_header_reads_and_collector_mutation() {
        let mut builder = MirBuilder::new();
        let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

        invocation.with_module_port(|builder, port| {
            port.with_headers(|headers| {
                assert_eq!(headers.symbol_count(), 0);
            });
            assert_eq!(builder.next_value_id().0, 0);

            port.prepare_draft_admission(
                FunctionDraftKeyV1::Main,
                "main".into(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("main", 0))
            .unwrap()
            .collect();

            port.with_headers(|headers| {
                assert!(headers.contains_symbol("main"));
                assert_eq!(headers.signature("main").unwrap().params.len(), 0);
            });
        });
    }

    #[test]
    fn resolved_child_terminal_collects_before_parent_restore() {
        let resolved = resolved_function("resolved_child");
        let owner = resolved.owner();
        let mut builder = MirBuilder::new();
        let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

        invocation
            .with_module_port(|builder, port| {
                port.complete_resolved_child(
                    builder,
                    ResolvedChildDraftAdmissionV1::canonical_resolved_owner(
                        owner,
                        "resolved_child/0".into(),
                        0,
                    ),
                    |builder| {
                        finish_resolved_authority(builder, &resolved);
                        Ok(draft("resolved_child/0", 0))
                    },
                )
            })
            .unwrap();

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 1);
            assert_eq!(
                headers.signature("resolved_child/0").unwrap().params.len(),
                0
            );
        });
    }

    #[test]
    fn resolved_child_admission_failure_restores_without_collection() {
        let resolved = resolved_function("rejected_child");
        let owner = resolved.owner();
        let mut builder = MirBuilder::new();
        let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);

        let error = invocation.with_module_port(|builder, port| {
            port.complete_resolved_child(
                builder,
                ResolvedChildDraftAdmissionV1::canonical_resolved_owner(
                    owner,
                    "rejected_child/0".into(),
                    0,
                ),
                |builder| {
                    finish_resolved_authority(builder, &resolved);
                    Ok(draft("wrong_symbol/0", 0))
                },
            )
        });
        assert!(error.is_err());

        invocation.with_header_port(|_builder, headers| {
            assert_eq!(headers.symbol_count(), 0);
            assert!(headers.signature("rejected_child/0").is_none());
        });
    }
}
