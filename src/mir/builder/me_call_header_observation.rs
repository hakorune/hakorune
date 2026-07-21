//! ACCESS0-MEHEADER-S0: typed, short-lived `me` header observation.
//!
//! This box selects one header source and turns only the parameter facts needed
//! by `me.method(...)` classification into an owned value.  It does not perform
//! argument descent, call emission, result annotation, or module publication.
//! Production ports consume this vocabulary only in the later I0 cutover.

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
    pub(in crate::mir::builder) fn from_lookup(
        source: MeCallHeaderSourceV1,
        symbol: &str,
        lookup: &dyn FunctionSignatureLookupV1,
    ) -> Self {
        let Some(signature) = lookup.signature(symbol) else {
            return Self::Missing {
                source,
                symbol: symbol.into(),
            };
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
/// before argument descent.  This trait is intentionally disconnected until
/// `ACCESS0-MEHEADER-I0`.
pub(in crate::mir::builder) trait MeCallHeaderObservationPortV1 {
    fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1;
}

/// Capability bundle reserved for the later method-call cutover.
pub(in crate::mir::builder) trait MethodCallLoweringPortV1:
    MethodCallValueTerminalPortV1 + MeCallHeaderObservationPortV1
{
}

impl<T> MethodCallLoweringPortV1 for T where
    T: MethodCallValueTerminalPortV1 + MeCallHeaderObservationPortV1
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
    pub(in crate::mir::builder) fn expected_params(&self) -> usize {
        self.expected_params
    }

    pub(in crate::mir::builder) fn receiver(&self) -> &PreparedMeReceiverV1 {
        &self.receiver
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
}
