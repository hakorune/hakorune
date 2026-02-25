/// Factory Priority Policy for Box creation (Phase 15.5 "Everything is Plugin")
///
/// Determines the order in which different Box factories are consulted
/// during Box creation to solve the StringBox/IntegerBox plugin priority issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryPolicy {
    /// Strict Plugin Priority: plugins > user > builtin
    /// ⚡ SOLVES THE CORE PROBLEM: Plugins have highest priority
    /// Use when plugins should completely replace builtins (Phase 15.5)
    StrictPluginFirst,

    /// Compatible Plugin Priority: plugins > builtin > user
    /// 🔧 Compatibility mode: Plugins first, but builtins before user-defined
    /// Use for gradual migration scenarios
    CompatPluginFirst,

    /// Legacy Builtin Priority: builtin > user > plugin (CURRENT DEFAULT)
    /// ⚠️ PROBLEMATIC: Plugins can never override builtins
    /// Only use for compatibility with existing setups
    BuiltinFirst,
}

/// Factory type classification for policy-based ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryType {
    /// Built-in factory (StringBox, IntegerBox, etc.)
    Builtin,
    /// User-defined Box factory
    User,
    /// Plugin-provided Box factory
    Plugin,
}
