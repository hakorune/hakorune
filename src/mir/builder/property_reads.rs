//! Property read lowering for unified member properties.
use super::calls::{
    emit_standard_value_terminal_raw_v1, CatalogHelperChildV1, MethodCallArgumentDescentV1,
    MethodCallDescentPortV1, StandardMethodCallCompletionV1,
};
use super::recursive_child_lowering::RawAstChildLoweringPortV1;
use super::ValueId;

pub(in crate::mir::builder) struct PropertyGetterCompletionV1<'port, Port> {
    port: &'port mut Port,
}

impl<'port, Port> PropertyGetterCompletionV1<'port, Port> {
    pub(in crate::mir::builder) const fn new(port: &'port mut Port) -> Self {
        Self { port }
    }
}

impl<Port> MethodCallArgumentDescentV1 for PropertyGetterCompletionV1<'_, Port>
where
    Port: RawAstChildLoweringPortV1,
{
    fn lower_all(&mut self, _builder: &mut super::MirBuilder) -> Result<Vec<ValueId>, String> {
        Ok(Vec::new())
    }

    fn lower_index(
        &mut self,
        _builder: &mut super::MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        Err(format!(
            "[property-getter-descent/indexed-argument] index={index} arity=0"
        ))
    }

    fn lower_catalog_helper_child(
        &mut self,
        builder: &mut super::MirBuilder,
        child: CatalogHelperChildV1,
    ) -> Result<ValueId, String> {
        MethodCallDescentPortV1::lower_catalog_helper_child(self.port, builder, child)
    }
}

impl<Port> StandardMethodCallCompletionV1 for PropertyGetterCompletionV1<'_, Port>
where
    Port: RawAstChildLoweringPortV1,
{
    fn finish_standard_value_terminal(
        &mut self,
        builder: &mut super::MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        if !arguments.is_empty() {
            return Err(format!(
                "[property-getter-descent/nonzero-terminal-arguments] count={}",
                arguments.len()
            ));
        }
        emit_standard_value_terminal_raw_v1(builder, receiver, method, arguments)
    }
}

impl super::MirBuilder {
    pub(super) fn try_lower_property_read_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        object_value: ValueId,
        field: &str,
    ) -> Result<Option<ValueId>, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let Some(getter_name) = self.resolve_property_getter_name(object_value, field) else {
            return Ok(None);
        };

        let mut completion = PropertyGetterCompletionV1::new(port);
        self.handle_standard_method_call_with_descent(
            object_value,
            getter_name,
            &[],
            &mut completion,
        )
        .map(Some)
    }
}
