//! Session-owned destination capability for one strict Compare append.
//!
//! The capability is deliberately narrower than a general `ValueId` issuer:
//! it reserves one fresh destination for the bounded Compare writer and does
//! not publish a type or inspect source meaning.

use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct ReservedCanonicalCompareDestinationV1 {
    owner: FunctionOwnerIdV1,
    value: ValueId,
    _seal: ReservedCanonicalCompareDestinationSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReservedCanonicalCompareDestinationSealV1;

impl ReservedCanonicalCompareDestinationV1 {
    pub(in crate::mir::builder) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn value(self) -> ValueId {
        self.value
    }

    fn from_session(owner: FunctionOwnerIdV1, value: ValueId) -> Self {
        Self {
            owner,
            value,
            _seal: ReservedCanonicalCompareDestinationSealV1,
        }
    }
}

impl<'source> super::CanonicalSsaFunctionSessionV2<'source> {
    /// Reserve one fresh physical destination through the canonical SSA
    /// allocator. The strict writer receives only this capability, never a
    /// free `ValueId`.
    pub(in crate::mir::builder::resolved_lowering) fn reserve_compare_destination(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ReservedCanonicalCompareDestinationV1, String> {
        let candidate = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| "canonical Compare destination requires current function".to_owned())?
            .next_value_id;
        let candidate = ValueId::new(candidate);
        if builder
            .function_state
            .type_ctx
            .get_type(candidate)
            .is_some()
        {
            return Err("[freeze:contract][canonical_compare/destination_not_fresh]".to_owned());
        }
        let value = self.issue_physical_value_id(builder)?;
        Ok(ReservedCanonicalCompareDestinationV1::from_session(
            self.owner, value,
        ))
    }
}
