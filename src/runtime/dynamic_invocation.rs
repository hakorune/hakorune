//! Runtime owner for ordinary source-bound Dynamic invocation.
//!
//! Source admission supplies the exact selector and arity. This module joins
//! those values with the receiver's runtime class through one immutable
//! provider table, freezes one plan, invokes it once, and publishes one
//! representation-neutral carrier that must be ended exactly once.

use crate::box_trait::{NyashBox, StringBox};
use crate::boxes::string_ops::{self, StringIndexMode};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};

const CARRIER_TOKEN_BRAND: u64 = 1 << 63;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicInvocationFailureV1 {
    InvalidCarrier,
    MissingProvider,
    AmbiguousProvider,
    ArgumentTypeMismatch,
    CarrierSpaceExhausted,
}

/// Opaque lifecycle identity for one published Dynamic result.
///
/// The raw word is the transport representation only. It grants no payload
/// interpretation, and a second `end` is rejected by the owner.
#[derive(Debug, PartialEq, Eq)]
pub struct DynamicCarrierTokenV1(NonZeroU64);

impl DynamicCarrierTokenV1 {
    pub fn from_raw_v1(raw: u64) -> Result<Self, DynamicInvocationFailureV1> {
        if raw & CARRIER_TOKEN_BRAND == 0 {
            return Err(DynamicInvocationFailureV1::InvalidCarrier);
        }
        NonZeroU64::new(raw)
            .map(Self)
            .ok_or(DynamicInvocationFailureV1::InvalidCarrier)
    }

    pub const fn raw_v1(&self) -> u64 {
        self.0.get()
    }

    /// Read an integer payload for a separately admitted Dynamic operator.
    /// This does not end, copy, or reclassify the carrier.
    pub fn integer_payload_v1(&self) -> Result<i64, DynamicInvocationFailureV1> {
        match self.snapshot_v1()? {
            CarrierPayloadV1::Integer(value) => Ok(value),
            _ => Err(DynamicInvocationFailureV1::ArgumentTypeMismatch),
        }
    }

    /// End the exact opaque carrier once. Forwarding keeps the same token and
    /// does not call this method at the forwarding point.
    pub fn end_v1(self) -> Result<(), DynamicInvocationFailureV1> {
        carriers()
            .lock()
            .expect("Dynamic carrier mutex poisoned")
            .remove(&self.0)
            .map(drop)
            .ok_or(DynamicInvocationFailureV1::InvalidCarrier)
    }

    fn snapshot_v1(&self) -> Result<CarrierPayloadV1, DynamicInvocationFailureV1> {
        carriers()
            .lock()
            .expect("Dynamic carrier mutex poisoned")
            .get(&self.0)
            .cloned()
            .ok_or(DynamicInvocationFailureV1::InvalidCarrier)
    }
}

/// Runtime values accepted at an ordinary Dynamic invocation boundary.
///
/// A carrier is borrowed by the invocation and remains live for the caller to
/// forward or end. Provider functions receive only a private resolved view.
#[derive(Debug)]
pub enum DynamicInvocationValueV1 {
    Null,
    Integer(i64),
    Bool(bool),
    Object(Arc<dyn NyashBox>),
    Weak(Weak<dyn NyashBox>),
    Carrier(DynamicCarrierTokenV1),
}

#[derive(Debug, Clone)]
enum CarrierPayloadV1 {
    Null,
    Integer(i64),
    Bool(bool),
    Object(Arc<dyn NyashBox>),
    Weak(Weak<dyn NyashBox>),
}

impl DynamicInvocationValueV1 {
    fn resolve(self) -> Result<CarrierPayloadV1, DynamicInvocationFailureV1> {
        match self {
            Self::Null => Ok(CarrierPayloadV1::Null),
            Self::Integer(value) => Ok(CarrierPayloadV1::Integer(value)),
            Self::Bool(value) => Ok(CarrierPayloadV1::Bool(value)),
            Self::Object(value) => Ok(CarrierPayloadV1::Object(value)),
            Self::Weak(value) => Ok(CarrierPayloadV1::Weak(value)),
            Self::Carrier(token) => token.snapshot_v1(),
        }
    }
}

type ProviderV1 = fn(
    &CarrierPayloadV1,
    &[CarrierPayloadV1],
) -> Result<CarrierPayloadV1, DynamicInvocationFailureV1>;

struct ProviderEntryV1 {
    runtime_class: &'static str,
    selector: &'static str,
    arity: usize,
    invoke: ProviderV1,
}

/// Provider behavior is registered as immutable data. Selection uses only
/// the exact runtime class, source selector, and source arity.
static PROVIDERS_V1: &[ProviderEntryV1] = &[
    ProviderEntryV1 {
        runtime_class: "StringBox",
        selector: "length",
        arity: 0,
        invoke: string_length_v1,
    },
    ProviderEntryV1 {
        runtime_class: "StringBox",
        selector: "substring",
        arity: 2,
        invoke: string_substring_v1,
    },
];

/// A frozen provider selection. Its fields are private and execution consumes
/// the plan, so callers cannot replace the selected provider or invoke twice.
pub struct DynamicInvocationPlanV1 {
    provider: &'static ProviderEntryV1,
    receiver: CarrierPayloadV1,
    arguments: Box<[CarrierPayloadV1]>,
}

impl DynamicInvocationPlanV1 {
    pub fn execute_once_v1(self) -> Result<DynamicCarrierTokenV1, DynamicInvocationFailureV1> {
        let result = (self.provider.invoke)(&self.receiver, &self.arguments)?;
        publish_carrier_v1(result)
    }
}

/// Admit one exact source message against the runtime receiver class and
/// freeze the unique matching provider before any provider effect occurs.
pub fn plan_dynamic_invocation_v1(
    receiver: DynamicInvocationValueV1,
    selector: &str,
    arguments: Vec<DynamicInvocationValueV1>,
) -> Result<DynamicInvocationPlanV1, DynamicInvocationFailureV1> {
    let receiver = receiver.resolve()?;
    let runtime_class = match &receiver {
        CarrierPayloadV1::Object(value) => value.type_name(),
        CarrierPayloadV1::Weak(_) => "Weak",
        CarrierPayloadV1::Null => "Null",
        CarrierPayloadV1::Integer(_) => "Integer",
        CarrierPayloadV1::Bool(_) => "Bool",
    };
    let mut matches = PROVIDERS_V1.iter().filter(|entry| {
        entry.runtime_class == runtime_class
            && entry.selector == selector
            && entry.arity == arguments.len()
    });
    let provider = matches
        .next()
        .ok_or(DynamicInvocationFailureV1::MissingProvider)?;
    if matches.next().is_some() {
        return Err(DynamicInvocationFailureV1::AmbiguousProvider);
    }
    let arguments = arguments
        .into_iter()
        .map(DynamicInvocationValueV1::resolve)
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    Ok(DynamicInvocationPlanV1 {
        provider,
        receiver,
        arguments,
    })
}

static NEXT_CARRIER: AtomicU64 = AtomicU64::new(1);
static CARRIERS: OnceLock<Mutex<HashMap<NonZeroU64, CarrierPayloadV1>>> = OnceLock::new();

fn carriers() -> &'static Mutex<HashMap<NonZeroU64, CarrierPayloadV1>> {
    CARRIERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn publish_carrier_v1(
    payload: CarrierPayloadV1,
) -> Result<DynamicCarrierTokenV1, DynamicInvocationFailureV1> {
    let raw = NEXT_CARRIER
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current
                .checked_add(1)
                .filter(|next| *next < CARRIER_TOKEN_BRAND)
        })
        .map_err(|_| DynamicInvocationFailureV1::CarrierSpaceExhausted)?;
    let token = NonZeroU64::new(CARRIER_TOKEN_BRAND | raw)
        .ok_or(DynamicInvocationFailureV1::CarrierSpaceExhausted)?;
    let mut table = carriers().lock().expect("Dynamic carrier mutex poisoned");
    match table.entry(token) {
        Entry::Vacant(entry) => {
            entry.insert(payload);
        }
        Entry::Occupied(_) => {
            return Err(DynamicInvocationFailureV1::CarrierSpaceExhausted);
        }
    }
    Ok(DynamicCarrierTokenV1(token))
}

fn string_length_v1(
    receiver: &CarrierPayloadV1,
    arguments: &[CarrierPayloadV1],
) -> Result<CarrierPayloadV1, DynamicInvocationFailureV1> {
    if !arguments.is_empty() {
        return Err(DynamicInvocationFailureV1::ArgumentTypeMismatch);
    }
    let CarrierPayloadV1::Object(receiver) = receiver else {
        return Err(DynamicInvocationFailureV1::ArgumentTypeMismatch);
    };
    let string = receiver
        .as_any()
        .downcast_ref::<StringBox>()
        .ok_or(DynamicInvocationFailureV1::ArgumentTypeMismatch)?;
    Ok(CarrierPayloadV1::Integer(
        string.value.chars().count() as i64
    ))
}

fn string_substring_v1(
    receiver: &CarrierPayloadV1,
    arguments: &[CarrierPayloadV1],
) -> Result<CarrierPayloadV1, DynamicInvocationFailureV1> {
    let [CarrierPayloadV1::Integer(start), CarrierPayloadV1::Integer(end)] = arguments else {
        return Err(DynamicInvocationFailureV1::ArgumentTypeMismatch);
    };
    let CarrierPayloadV1::Object(receiver) = receiver else {
        return Err(DynamicInvocationFailureV1::ArgumentTypeMismatch);
    };
    let string = receiver
        .as_any()
        .downcast_ref::<StringBox>()
        .ok_or(DynamicInvocationFailureV1::ArgumentTypeMismatch)?;
    let result = string_ops::substring(
        &string.value,
        *start,
        Some(*end),
        StringIndexMode::CodePoint,
    );
    Ok(CarrierPayloadV1::Object(Arc::new(StringBox::new(result))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_string_provider_plan_publishes_end_authorized_results() {
        let receiver = DynamicInvocationValueV1::Object(Arc::new(StringBox::new("猫🐾")));
        let length = plan_dynamic_invocation_v1(receiver, "length", vec![])
            .expect("StringBox.length/0 provider")
            .execute_once_v1()
            .expect("Dynamic carrier");
        let length_value = length.snapshot_v1().expect("live length carrier");
        assert!(matches!(length_value, CarrierPayloadV1::Integer(2)));

        let receiver = DynamicInvocationValueV1::Object(Arc::new(StringBox::new("猫🐾")));
        let substring = plan_dynamic_invocation_v1(
            receiver,
            "substring",
            vec![
                DynamicInvocationValueV1::Integer(0),
                DynamicInvocationValueV1::Integer(1),
            ],
        )
        .expect("StringBox.substring/2 provider")
        .execute_once_v1()
        .expect("Dynamic carrier");
        let CarrierPayloadV1::Object(value) =
            substring.snapshot_v1().expect("live substring carrier")
        else {
            panic!("substring publishes an owner-bearing Dynamic carrier");
        };
        assert_eq!(
            value.as_any().downcast_ref::<StringBox>().unwrap().value,
            "猫"
        );
        length.end_v1().expect("end trivial carrier");
        substring.end_v1().expect("end owner-bearing carrier");
    }

    #[test]
    fn missing_provider_and_repeated_end_are_terminal() {
        let receiver = DynamicInvocationValueV1::Object(Arc::new(StringBox::new("x")));
        assert!(matches!(
            plan_dynamic_invocation_v1(receiver, "substring", vec![]),
            Err(DynamicInvocationFailureV1::MissingProvider)
        ));

        let receiver = DynamicInvocationValueV1::Object(Arc::new(StringBox::new("x")));
        let carrier = plan_dynamic_invocation_v1(receiver, "length", vec![])
            .unwrap()
            .execute_once_v1()
            .unwrap();
        let raw = carrier.raw_v1();
        carrier.end_v1().unwrap();
        assert!(matches!(
            DynamicCarrierTokenV1::from_raw_v1(raw).unwrap().end_v1(),
            Err(DynamicInvocationFailureV1::InvalidCarrier)
        ));
    }
}
