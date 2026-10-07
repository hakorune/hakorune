//! Selected source carrier identity and direct physical Copy reuse.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    BorrowedCompareCarrierOperandLoanV1 as Loan, BorrowedCompareSourceLoanV1 as Source,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;

#[derive(Debug, Default)]
pub(super) struct CarrierReuse {
    loans: BTreeMap<(OwnedExprSiteV1, OwnedExprSiteV1), Rc<Loan>>,
    copies: BTreeMap<ValueId, (Rc<Loan>, Binding)>,
}
impl CarrierReuse {
    pub(super) fn is_empty(&self) -> bool {
        self.loans.is_empty() && self.copies.is_empty()
    }
    pub(super) fn contains(&self, value: ValueId) -> bool {
        self.loans.values().any(|loan| loan.value() == value) || self.copies.contains_key(&value)
    }
    fn source(&self, value: ValueId) -> Result<Option<(Rc<Loan>, Option<Binding>)>, String> {
        if let Some(loan) = self.loans.values().find(|loan| loan.value() == value) {
            return Ok(Some((
                Rc::clone(loan),
                loan.original_copies()?.last().cloned(),
            )));
        }
        Ok(self
            .copies
            .get(&value)
            .map(|(loan, binding)| (Rc::clone(loan), Some(binding.clone()))))
    }
    pub(super) fn observations(&self) -> impl Iterator<Item = (&Rc<Loan>, Vec<Binding>)> {
        self.loans.values().map(|loan| {
            (
                loan,
                self.copies
                    .values()
                    .filter(|(source, _)| source.value() == loan.value())
                    .map(|(_, binding)| binding.clone())
                    .collect(),
            )
        })
    }
    pub(super) fn verify(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("function-missing"))?;
        for loan in self.loans.values() {
            check_entry_owner(builder, loan.owner())?;
            verify_root(function, loan)?;
        }
        for (value, (loan, copy)) in &self.copies {
            exact_definition(function, *value, copy)?;
            if !matches!(copy.1, MirInstruction::Copy { dst, src } if dst == *value && src == loan.value())
            {
                return Err(fault("carrier-copy"));
            }
            verify_use(function, loan, copy.0)?;
            if let Some(root) = loan.original_copies()?.last() {
                require_definition_order(function, root, copy)?;
            }
        }
        Ok(())
    }
}
fn verify_root(function: &MirFunction, loan: &Loan) -> Result<(), String> {
    if function
        .params
        .iter()
        .filter(|value| **value == loan.entry_value())
        .count()
        != 1
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|row| row.dst_value() == Some(loan.entry_value()))
    {
        return Err(fault("carrier-parameter"));
    }
    let mut previous: Option<&Binding> = None;
    for copy in loan.original_copies()? {
        let value = copy
            .1
            .dst_value()
            .ok_or_else(|| fault("carrier-original-copy"))?;
        exact_definition(function, value, copy)?;
        if let Some(previous) = previous {
            require_definition_order(function, previous, copy)?;
        } else {
            require_dominance(function, function.entry_block, copy.0)?;
        }
        previous = Some(copy);
    }
    Ok(())
}
fn verify_use(function: &MirFunction, loan: &Loan, block: BasicBlockId) -> Result<(), String> {
    verify_root(function, loan)?;
    let definition = loan
        .original_copies()?
        .last()
        .map_or(function.entry_block, |copy| copy.0);
    require_dominance(function, definition, block)
}
pub(in crate::mir::builder) fn install_carriers(
    builder: &mut MirBuilder,
    source: &Source,
    children: (ValueId, ValueId),
    loans: Vec<Loan>,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    let (left, right) = source.operand_sites();
    let mut sites = std::collections::BTreeSet::new();
    for loan in &loans {
        if !sites.insert((loan.binary().clone(), loan.site().clone())) {
            return Err(fault("duplicate-carrier"));
        }
        check_entry_owner(builder, loan.owner())?;
        let child = if loan.site() == left {
            children.0
        } else if loan.site() == right {
            children.1
        } else {
            return Err(fault("carrier-side"));
        };
        if loan.binary() != source.site() || child != loan.value() {
            return Err(fault("carrier-child"));
        }
        verify_root(function, loan)?;
        if builder
            .function_state
            .checked_compare_reuse
            .carriers
            .loans
            .contains_key(&(loan.binary().clone(), loan.site().clone()))
        {
            return Err(fault("duplicate-carrier"));
        }
    }
    for loan in loans {
        builder
            .function_state
            .checked_compare_reuse
            .carriers
            .loans
            .insert((loan.binary().clone(), loan.site().clone()), Rc::new(loan));
    }
    Ok(())
}
pub(super) fn materialize(builder: &mut MirBuilder, value: ValueId) -> Result<ValueId, Error> {
    let (loan, definition) = builder
        .function_state
        .checked_compare_reuse
        .carriers
        .source(value)
        .map_err(Error::Contract)?
        .ok_or_else(|| Error::Contract(fault("carrier-missing")))?;
    check_entry_owner(builder, loan.owner()).map_err(Error::Contract)?;
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| Error::Contract(fault("consumer-block")))?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| Error::Contract(fault("function-missing")))?;
    if function
        .blocks
        .get(&block)
        .is_none_or(|block| block.terminator.is_some())
    {
        return Err(Error::Contract(fault("consumer-closed")));
    }
    verify_use(function, &loan, block).map_err(Error::Contract)?;
    if let Some(definition) = &definition {
        exact_definition(function, value, definition).map_err(Error::Contract)?;
        require_dominance(function, definition.0, block).map_err(Error::Contract)?;
        if definition.0 == block {
            return Ok(value);
        }
    } else if function.entry_block == block {
        return Ok(value);
    }
    let source_type =
        LocalSsaSourceTypeEntryV1::classify(builder.function_state.type_ctx.get_type(loan.value()));
    let dst = builder.next_value_id();
    let prepared = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
        &source_type,
        LocalSsaMaterializationKindV1::PhysicalCopy(
            LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
        ),
        builder.function_state.type_ctx.get_type(dst),
    )
    .map_err(|error| Error::Contract(error.to_string()))?;
    let instruction = MirInstruction::Copy {
        dst,
        src: loan.value(),
    };
    builder
        .emit_instruction(instruction.clone())
        .map_err(Error::InstructionEmission)?;
    prepared.commit(dst, &mut builder.function_state.type_ctx);
    builder
        .function_state
        .checked_compare_reuse
        .carriers
        .copies
        .insert(dst, (loan, (block, instruction)));
    Ok(dst)
}
