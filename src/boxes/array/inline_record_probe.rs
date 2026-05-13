use super::*;

pub(super) struct ArrayInlineRecordProbe;

impl ArrayInlineRecordProbe {
    pub(super) fn build(layout_id: u32, columns: Vec<ArrayInlineRecordColumn>) -> Option<ArrayBox> {
        let storage = ArrayInlineRecordStorage::new(layout_id, columns)?;
        Some(ArrayBox::new_with_inline_record_storage(storage))
    }
}
