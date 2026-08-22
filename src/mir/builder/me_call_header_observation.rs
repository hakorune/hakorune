//! ACCESS0-MEHEADER-S0: typed, short-lived `me` header observation.
//!
//! This box selects one header source and turns only the parameter facts needed
//! by `me.method(...)` classification into an owned value.  It does not perform
//! argument descent, call emission, result annotation, or module publication.
//! The shared method policy consumes this vocabulary at I0; production
//! module draft/fact capture remains a separate cutover.

use super::calls::MethodCallDescentPortV1;
use super::calls::MethodCallValueTerminalPortV1;
use super::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::{MirBuilder, MirType, ValueId};

/// The authority selected before a `me` header is observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum MeCallHeaderSourceV1 {
    ModuleCompatibility,
    InvocationCollector,
}

/// Borrow-free parameter evidence consumed by the single `me` policy owner.
///
/// `Missing` is source-branded: an invocation collector miss is not the same
/// state as a compatibility route that does not own collector authority.
/// This product deliberately omits return/effect/body/metadata facts.
#[derive(Debug)]
pub(in crate::mir::builder) enum MeCallParameterObservationV1 {
    Missing {
        source: MeCallHeaderSourceV1,
        symbol: Box<str>,
    },
    Present {
        source: MeCallHeaderSourceV1,
        symbol: Box<str>,
        parameter_count: usize,
        first_parameter: Option<MirType>,
    },
}

impl MeCallParameterObservationV1 {
    pub(in crate::mir::builder) fn missing(source: MeCallHeaderSourceV1, symbol: &str) -> Self {
        Self::Missing {
            source,
            symbol: symbol.into(),
        }
    }

    pub(in crate::mir::builder) fn from_optional_lookup(
        source: MeCallHeaderSourceV1,
        symbol: &str,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Self {
        lookup.map_or_else(
            || Self::missing(source, symbol),
            |lookup| Self::from_lookup(source, symbol, lookup),
        )
    }

    pub(in crate::mir::builder) fn from_lookup(
        source: MeCallHeaderSourceV1,
        symbol: &str,
        lookup: &dyn FunctionSignatureLookupV1,
    ) -> Self {
        let Some(signature) = lookup.signature(symbol) else {
            return Self::missing(source, symbol);
        };

        let parameter_count = signature.params.len();
        let first_parameter = signature.params.first().cloned();
        debug_assert_eq!(parameter_count == 0, first_parameter.is_none());
        Self::Present {
            source,
            symbol: symbol.into(),
            parameter_count,
            first_parameter,
        }
    }

    pub(in crate::mir::builder) fn source(&self) -> MeCallHeaderSourceV1 {
        match self {
            Self::Missing { source, .. } | Self::Present { source, .. } => *source,
        }
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        match self {
            Self::Missing { symbol, .. } | Self::Present { symbol, .. } => symbol,
        }
    }
}

/// Construction-only header observation capability for method-call routes.
///
/// The returned product owns its small snapshot, so the source loan ends
/// before argument descent.  Invocation uses collector authority only;
/// compatibility routes remain module-backed.
pub(in crate::mir::builder) trait MeCallHeaderObservationPortV1 {
    fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1;
}

/// Capability bundle used by the shared method-call policy.  Terminal lookup
/// and pre-argument header observation remain separate authorities.
pub(in crate::mir::builder) trait MethodCallLoweringPortV1:
    MethodCallDescentPortV1 + MethodCallValueTerminalPortV1 + MeCallHeaderObservationPortV1
{
}

impl<T> MethodCallLoweringPortV1 for T where
    T: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1 + MeCallHeaderObservationPortV1
{
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedMeLoweredCallV1 {
    expected_params: usize,
    receiver: PreparedMeReceiverV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedMeReceiverV1 {
    Instance { me: Option<ValueId> },
    Static,
}

impl PreparedMeLoweredCallV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test_parts(
        expected_params: usize,
        receiver: PreparedMeReceiverV1,
    ) -> Self {
        Self {
            expected_params,
            receiver,
        }
    }

    pub(in crate::mir::builder) fn expected_params(&self) -> usize {
        self.expected_params
    }

    pub(in crate::mir::builder) fn receiver(&self) -> &PreparedMeReceiverV1 {
        &self.receiver
    }

    pub(in crate::mir::builder) fn into_parts(self) -> (usize, PreparedMeReceiverV1) {
        (self.expected_params, self.receiver)
    }
}

/// Consume the owned observation and prepare only the receiver/arity policy.
/// A source-branded miss returns `None`; callers must not perform a second
/// header lookup or reinterpret that miss as a collector fallback.
pub(in crate::mir::builder) fn prepare_me_lowered_call_v1(
    observation: MeCallParameterObservationV1,
    me: Option<ValueId>,
) -> Option<PreparedMeLoweredCallV1> {
    let MeCallParameterObservationV1::Present {
        parameter_count,
        first_parameter,
        ..
    } = observation
    else {
        return None;
    };

    let receiver = if matches!(first_parameter, Some(MirType::Box(_))) {
        PreparedMeReceiverV1::Instance { me }
    } else {
        PreparedMeReceiverV1::Static
    };
    Some(PreparedMeLoweredCallV1 {
        expected_params: parameter_count,
        receiver,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{EffectMask, FunctionSignature};
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct FakeHeaders {
        signatures: BTreeMap<String, FunctionSignature>,
    }

    impl FakeHeaders {
        fn insert(&mut self, symbol: &str, params: Vec<MirType>) {
            self.signatures.insert(
                symbol.to_string(),
                FunctionSignature {
                    name: symbol.to_string(),
                    params,
                    return_type: MirType::Void,
                    effects: EffectMask::PURE,
                },
            );
        }
    }

    impl FunctionSignatureLookupV1 for FakeHeaders {
        fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
            self.signatures.get(symbol)
        }

        fn contains_symbol(&self, symbol: &str) -> bool {
            self.signatures.contains_key(symbol)
        }

        fn symbol_count(&self) -> usize {
            self.signatures.len()
        }

        fn visit_symbols(&self, visitor: &mut dyn FnMut(&str)) {
            for symbol in self.signatures.keys() {
                visitor(symbol);
            }
        }
    }

    struct TestObservationPort {
        source: MeCallHeaderSourceV1,
        headers: FakeHeaders,
        observations: usize,
    }

    impl MeCallHeaderObservationPortV1 for TestObservationPort {
        fn observe_me_call_parameters(
            &mut self,
            _builder: &MirBuilder,
            symbol: &str,
        ) -> MeCallParameterObservationV1 {
            self.observations += 1;
            MeCallParameterObservationV1::from_lookup(self.source, symbol, &self.headers)
        }
    }

    #[test]
    fn source_branded_missing_does_not_become_present() {
        let headers = FakeHeaders::default();
        let observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::InvocationCollector,
            "Box.m/1",
            &headers,
        );
        assert_eq!(
            observation.source(),
            MeCallHeaderSourceV1::InvocationCollector
        );
        assert_eq!(observation.symbol(), "Box.m/1");
        assert!(prepare_me_lowered_call_v1(observation, None).is_none());
    }

    #[test]
    fn first_box_parameter_prepares_instance_receiver() {
        let mut headers = FakeHeaders::default();
        headers.insert(
            "Box.m/1",
            vec![MirType::Box("Box".to_string()), MirType::Integer],
        );
        let observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::InvocationCollector,
            "Box.m/1",
            &headers,
        );
        let prepared = prepare_me_lowered_call_v1(observation, Some(ValueId(7))).unwrap();
        assert_eq!(prepared.expected_params(), 2);
        assert_eq!(
            prepared.receiver(),
            &PreparedMeReceiverV1::Instance {
                me: Some(ValueId(7))
            }
        );
    }

    #[test]
    fn non_box_or_empty_parameters_prepare_static_receiver() {
        let mut headers = FakeHeaders::default();
        headers.insert("Box.static/0", vec![]);
        let observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::ModuleCompatibility,
            "Box.static/0",
            &headers,
        );
        let prepared = prepare_me_lowered_call_v1(observation, Some(ValueId(9))).unwrap();
        assert_eq!(prepared.expected_params(), 0);
        assert_eq!(prepared.receiver(), &PreparedMeReceiverV1::Static);
    }

    #[test]
    fn route_source_matrix_keeps_missing_header_typed() {
        let builder = MirBuilder::new();
        let mut routes = [
            TestObservationPort {
                source: MeCallHeaderSourceV1::ModuleCompatibility,
                headers: FakeHeaders::default(),
                observations: 0,
            },
            TestObservationPort {
                source: MeCallHeaderSourceV1::InvocationCollector,
                headers: FakeHeaders::default(),
                observations: 0,
            },
        ];

        for route in &mut routes {
            let observation = route.observe_me_call_parameters(&builder, "Box.m/1");
            assert!(matches!(
                observation,
                MeCallParameterObservationV1::Missing { .. }
            ));
            assert_eq!(observation.source(), route.source);
            assert_eq!(route.observations, 1);
            assert!(prepare_me_lowered_call_v1(observation, None).is_none());
        }
    }

    #[test]
    fn owned_observation_ends_header_loan_before_mutation() {
        let mut headers = FakeHeaders::default();
        headers.insert("Box.m/1", vec![MirType::Box("Box".to_string())]);
        let mut port = TestObservationPort {
            source: MeCallHeaderSourceV1::InvocationCollector,
            headers,
            observations: 0,
        };
        let builder = MirBuilder::new();
        let observation = port.observe_me_call_parameters(&builder, "Box.m/1");

        // This mutation is legal immediately after observation: no lookup loan
        // is retained by the owned product.
        port.headers.insert("Box.m/1", vec![]);

        let prepared = prepare_me_lowered_call_v1(observation, Some(ValueId(3))).unwrap();
        assert_eq!(prepared.expected_params(), 1);
        assert_eq!(
            prepared.receiver(),
            &PreparedMeReceiverV1::Instance {
                me: Some(ValueId(3))
            }
        );
        assert_eq!(port.observations, 1);
    }

    #[test]
    fn compatibility_and_invocation_sources_keep_their_own_header_truth() {
        let mut module_headers = FakeHeaders::default();
        module_headers.insert("Box.m/1", vec![MirType::Integer]);
        let mut collector_headers = FakeHeaders::default();
        collector_headers.insert("Box.m/1", vec![MirType::Box("Box".to_string())]);
        let module_observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::ModuleCompatibility,
            "Box.m/1",
            &module_headers,
        );
        let located_observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::ModuleCompatibility,
            "Box.m/1",
            &module_headers,
        );
        let invocation_observation = MeCallParameterObservationV1::from_lookup(
            MeCallHeaderSourceV1::InvocationCollector,
            "Box.m/1",
            &collector_headers,
        );

        assert_eq!(
            prepare_me_lowered_call_v1(module_observation, None)
                .unwrap()
                .receiver(),
            &PreparedMeReceiverV1::Static
        );
        assert_eq!(
            prepare_me_lowered_call_v1(located_observation, None)
                .unwrap()
                .receiver(),
            &PreparedMeReceiverV1::Static
        );
        assert_eq!(
            prepare_me_lowered_call_v1(invocation_observation, Some(ValueId(11)))
                .unwrap()
                .receiver(),
            &PreparedMeReceiverV1::Instance {
                me: Some(ValueId(11))
            }
        );
    }
}
