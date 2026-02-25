use crate::mir::builder::MirBuilder;

pub(super) fn current_span_location(builder: &MirBuilder) -> String {
    builder.metadata_ctx.current_span().location_string()
}
