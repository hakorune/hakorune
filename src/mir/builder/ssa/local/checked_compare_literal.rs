//! Original source-bound Integer Const reuse for selected compare operands.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralMaterializationV1 as Literal;

#[derive(Debug, Default)]
pub(super) struct LiteralReuse {
    records: BTreeMap<ValueId, Rc<Literal>>,
    copies: BTreeMap<ValueId, (Rc<Literal>, Binding)>,
}
impl LiteralReuse {
    pub(super) fn is_empty(&self) -> bool {
        self.records.is_empty() && self.copies.is_empty()
    }
    pub(super) fn contains(&self, value: ValueId) -> bool {
        self.records.contains_key(&value) || self.copies.contains_key(&value)
    }
    pub(super) fn observations(&self) -> impl Iterator<Item = (&Rc<Literal>, Vec<Binding>)> {
        self.records.values().map(|record| {
            (
                record,
                self.copies
                    .values()
                    .filter(|(source, _)| Rc::ptr_eq(source, record))
                    .map(|(_, binding)| binding.clone())
                    .collect(),
            )
        })
    }
    fn source(&self, value: ValueId) -> Option<(Rc<Literal>, Binding)> {
        self.records
            .get(&value)
            .map(|r| (Rc::clone(r), r.original().clone()))
            .or_else(|| {
                self.copies
                    .get(&value)
                    .map(|(r, b)| (Rc::clone(r), b.clone()))
            })
    }
    pub(super) fn verify(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("function-missing"))?;
        for record in self.records.values() {
            check_literal(builder, record)?;
            exact_definition(function, record.value(), record.original())?;
        }
        for (value, (record, copy)) in &self.copies {
            check_literal(builder, record)?;
            exact_definition(function, *value, copy)?;
            if !matches!(copy.1, MirInstruction::Copy { dst, src } if dst == *value && src == record.value())
                || builder.function_state.type_ctx.get_type(*value) != Some(&MirType::Integer)
            {
                return Err(fault("literal-copy-identity"));
            }
            require_definition_order(function, record.original(), copy)?;
        }
        Ok(())
    }
}
fn check_literal(builder: &MirBuilder, record: &Literal) -> Result<(), String> {
    check_entry_owner(builder, record.owner())?;
    if builder.function_state.type_ctx.get_type(record.value()) != Some(&MirType::Integer)
        || !matches!(record.original().1, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(_) } if dst == record.value())
    {
        return Err(fault("literal-type"));
    }
    Ok(())
}
pub(in crate::mir::builder) fn install_literal(
    builder: &mut MirBuilder,
    record: Rc<Literal>,
) -> Result<(), String> {
    check_literal(builder, &record)?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    if builder.function_state.current_block != Some(record.original().0) {
        return Err(fault("literal-install-block"));
    }
    exact_definition(function, record.value(), record.original())?;
    if builder
        .function_state
        .checked_compare_reuse
        .contains(record.value())
    {
        return Err(fault("duplicate-literal-install"));
    }
    builder
        .function_state
        .checked_compare_reuse
        .literals
        .records
        .insert(record.value(), record);
    Ok(())
}
pub(super) fn materialize(builder: &mut MirBuilder, value: ValueId) -> Result<ValueId, Error> {
    let (record, definition) = builder
        .function_state
        .checked_compare_reuse
        .literals
        .source(value)
        .ok_or_else(|| Error::Contract(fault("literal-record-missing")))?;
    check_literal(builder, &record).map_err(Error::Contract)?;
    let consumer = builder
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
        .get(&consumer)
        .is_none_or(|b| b.terminator.is_some())
    {
        return Err(Error::Contract(fault("consumer-closed")));
    }
    exact_definition(function, record.value(), record.original()).map_err(Error::Contract)?;
    exact_definition(function, value, &definition).map_err(Error::Contract)?;
    if value != record.value() {
        require_definition_order(function, record.original(), &definition)
            .map_err(Error::Contract)?;
    }
    require_dominance(function, record.original().0, consumer).map_err(Error::Contract)?;
    require_dominance(function, definition.0, consumer).map_err(Error::Contract)?;
    if definition.0 == consumer {
        return Ok(value);
    }
    let dst = builder.next_value_id();
    let prepared = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
        &LocalSsaSourceTypeEntryV1::Exact(MirType::Integer),
        LocalSsaMaterializationKindV1::PhysicalCopy(
            LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
        ),
        builder.function_state.type_ctx.get_type(dst),
    )
    .map_err(|e| Error::Contract(e.to_string()))?;
    let instruction = MirInstruction::Copy {
        dst,
        src: record.value(),
    };
    builder
        .emit_instruction(instruction.clone())
        .map_err(Error::InstructionEmission)?;
    prepared.commit(dst, &mut builder.function_state.type_ctx);
    builder
        .function_state
        .checked_compare_reuse
        .literals
        .copies
        .insert(dst, (record, (consumer, instruction)));
    Ok(dst)
}
