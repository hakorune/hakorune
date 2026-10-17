//! Original checked Get and Copy transport lent to an existing New argument.
use super::field_read::EmittedFieldArrayReadV1;
use crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::mir::named_array_obligation::fault;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug, Clone)]
pub(crate) struct EmittedFieldArrayReadArgumentV1 {
    source: Rc<ArgumentFieldArrayReadSourceV1>,
    read: EmittedFieldArrayReadV1,
    copies: Vec<Binding>,
    value: ValueId,
}
impl PartialEq for EmittedFieldArrayReadArgumentV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && self.read == other.read
            && self.copies == other.copies && self.value == other.value
    }
}
impl Eq for EmittedFieldArrayReadArgumentV1 {}
impl EmittedFieldArrayReadArgumentV1 {
    /// Called only with the existing lowering state's same-source read and
    /// exact binding value. Physical Copy inspection corroborates that loan.
    pub(in crate::mir) fn capture(source: Rc<ArgumentFieldArrayReadSourceV1>,
        read: &EmittedFieldArrayReadV1, function: &MirFunction, value: ValueId,
    ) -> Result<Self, String> {
        if !Rc::ptr_eq(source.read(), read.recipe().source()) {
            return Err(fault("argument-read-source-drift"));
        }
        read.validate_structure(function)?;
        let mut current = value;
        let mut copies = Vec::new();
        let budget = function.blocks.values().map(|block| block.instructions.len() + 1).sum::<usize>();
        for _ in 0..=budget {
            if current == read.dst() {
                copies.reverse();
                return Ok(Self { source, read: read.clone(), copies, value });
            }
            let mut definitions = function.blocks.iter().flat_map(|(block, body)|
                body.all_instructions().filter(move |row| row.dst_value() == Some(current))
                    .map(move |row| (*block, row.clone())));
            let original = definitions.next().ok_or_else(|| fault("argument-read-copy-missing"))?;
            if definitions.next().is_some() { return Err(fault("argument-read-copy-duplicate")); }
            let MirInstruction::Copy { src, .. } = &original.1 else { return Err(fault("argument-read-origin-drift")) };
            if *src == current { return Err(fault("argument-read-copy-cycle")); }
            current = *src;
            copies.push(original);
        }
        Err(fault("argument-read-copy-cycle"))
    }
    pub(crate) fn matches(&self, source: &Rc<ArgumentFieldArrayReadSourceV1>, value: ValueId) -> bool {
        Rc::ptr_eq(&self.source, source) && self.value == value
            && Rc::ptr_eq(self.source.read(), self.read.recipe().source())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate(
        &self, function: &MirFunction, value: ValueId,
        mut project: impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<(), String> {
        if self.value != value { return Err(fault("argument-read-value-drift")); }
        self.read.validate_structure(function)?;
        let mut current = self.read.dst();
        for original in &self.copies {
            let row = project(original)?.ok_or_else(|| fault("argument-read-copy-omitted"))?;
            let MirInstruction::Copy { dst, src } = row.1 else { return Err(fault("argument-read-copy-kind")) };
            if src != current || function.blocks.get(&row.0).is_none_or(|body|
                body.all_instructions().filter(|actual| *actual == &row.1).count() != 1) {
                return Err(fault("argument-read-copy-drift"));
            }
            current = dst;
        }
        if current != value { return Err(fault("argument-read-actual-drift")); }
        Ok(())
    }
}
