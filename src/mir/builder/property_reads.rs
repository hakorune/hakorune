//! Property read lowering for unified member properties.

use super::ValueId;

impl super::MirBuilder {
    pub(super) fn try_lower_property_read(
        &mut self,
        object_value: ValueId,
        field: &str,
    ) -> Result<Option<ValueId>, String> {
        let Some(class_name) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        else {
            return Ok(None);
        };
        let Some(getter_name) = self
            .comp_ctx
            .property_getter_method_name(&class_name, field)
        else {
            return Ok(None);
        };

        self.handle_standard_method_call(object_value, getter_name, &[])
            .map(Some)
    }
}
