//! Property read lowering for unified member properties.

use super::ValueId;

impl super::MirBuilder {
    pub(super) fn try_lower_property_read(
        &mut self,
        object_value: ValueId,
        field: &str,
    ) -> Result<Option<ValueId>, String> {
        let Some(getter_name) = self.resolve_property_getter_name(object_value, field) else {
            return Ok(None);
        };

        self.handle_standard_method_call(object_value, getter_name, &[])
            .map(Some)
    }
}
