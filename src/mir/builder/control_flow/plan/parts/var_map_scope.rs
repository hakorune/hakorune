use crate::mir::builder::MirBuilder;

/// Run `f` with a saved variable_map snapshot and always restore afterward.
///
/// This keeps branch-local lowering failures from leaking partially-mutated
/// bindings into outer lowering paths.
pub(super) fn with_saved_variable_map<T, F>(builder: &mut MirBuilder, f: F) -> Result<T, String>
where
    F: FnOnce(&mut MirBuilder) -> Result<T, String>,
{
    let saved = builder.variable_ctx.variable_map.clone();
    let result = f(builder);
    builder.variable_ctx.variable_map = saved;
    result
}
