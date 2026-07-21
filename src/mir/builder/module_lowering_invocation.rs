//! HEADERPORT0-S0: external owner for one module-lowering invocation.
//!
//! This vocabulary deliberately stays outside `MirBuilder` and
//! `CompilationContext`. It owns the sole unpublished-draft collector and
//! lends only a short-lived, read-only header port to an explicit lowering
//! closure. Production roots and child terminals remain disconnected until the
//! atomic MODULEDRAFT0-HEADERPORT0-I0 cutover.

use crate::mir::{FunctionSignature, MirBuilder};

use super::module_draft_collector::{CompletedDraftSignatureViewV1, ModuleDraftCollectorV1};
use super::module_draft_collector::{
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1,
    PreparedFunctionDraftAdmissionV1,
};

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
}

/// The external owner of one module-lowering invocation.
///
/// It borrows the route's Builder and owns the invocation-local collector, but
/// is never stored in either of them. The only public transition in S0 loans a
/// read-only header capability; collection remains an I0 responsibility.
pub(in crate::mir::builder) struct ModuleLoweringInvocationV1<'builder> {
    builder: &'builder mut MirBuilder,
    collector: ModuleDraftCollectorV1,
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
        Self {
            builder,
            collector,
            _seal: ModuleLoweringInvocationSealV1,
        }
    }

    /// Lend one stack-owned lowering port to an explicit recursive descent.
    ///
    /// This remains disconnected in RAWPORT0-S0. Future routes use the port to
    /// alternate header reads with child terminal collection without ambient
    /// collector lookup.
    pub(in crate::mir::builder) fn with_module_port<R>(
        &mut self,
        lower: impl for<'port> FnOnce(&mut MirBuilder, &'port mut ModuleLoweringPortV1<'port>) -> R,
    ) -> R {
        let builder = &mut *self.builder;
        let mut port = ModuleLoweringPortV1 {
            collector: &mut self.collector,
            _seal: ModuleLoweringPortSealV1,
        };
        lower(builder, &mut port)
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
        self.collector
            .prepare_admission(key, expected_symbol, expected_arity, policy)
    }
}

#[cfg(test)]
mod tests {
    use super::ModuleLoweringInvocationV1;
    use crate::mir::builder::module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
    };
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType,
    };

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
}
