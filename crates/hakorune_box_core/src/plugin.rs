/// Plugin execution route policy for Box factory lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginExecMode {
    /// Prefer core module implementations before dynamic plugin lookup.
    ModuleFirst,
    /// Use only dynamic plugin lookup.
    DynamicOnly,
    /// Try dynamic plugin lookup before core module implementations.
    DynamicFirst,
}
