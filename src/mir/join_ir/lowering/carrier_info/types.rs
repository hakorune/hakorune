use crate::mir::ValueId;
use std::collections::BTreeSet;

#[cfg(feature = "normalized_dev")]
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

#[cfg(feature = "normalized_dev")]
use crate::mir::BindingId; // Phase 76+78: BindingId for promoted carriers

/// Phase 227: CarrierRole - Distinguishes loop state carriers from condition-only carriers
///
/// When LoopBodyLocal variables are promoted to carriers, we need to know whether
/// they carry loop state (need exit PHI) or are only used in conditions (no exit PHI).
///
/// # Example
///
/// ```ignore
/// // LoopState carrier: sum needs exit PHI (value persists after loop)
/// loop(i < n) {
///     sum = sum + i;  // sum updated in loop body
/// }
/// print(sum);  // sum used after loop
///
/// // ConditionOnly carrier: is_digit_pos does NOT need exit PHI
/// loop(p < s.length()) {
///     local digit_pos = digits.indexOf(s.substring(p, p+1));
///     if digit_pos < 0 { break; }  // Only used in condition
///     num_str = num_str + ch;
///     p = p + 1;
/// }
/// // digit_pos not used after loop
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarrierRole {
    /// Value needed after loop (sum, result, count, p, num_str)
    /// - Participates in header PHI (loop iteration)
    /// - Participates in exit PHI (final value after loop)
    LoopState,

    /// Only used for loop condition (is_digit_pos, is_whitespace)
    /// - Participates in header PHI (loop iteration)
    /// - Does NOT participate in exit PHI (not needed after loop)
    ConditionOnly,
}

/// Phase 228: Initialization policy for carrier variables
///
/// When carriers participate in header PHI, they need an initial value.
/// Most carriers use their host_id value (FromHost), but promoted LoopBodyLocal
/// carriers need explicit bool initialization (BoolConst).
///
/// # Example
///
/// ```ignore
/// // Regular carrier (sum): Use host_id value
/// CarrierVar { name: "sum", host_id: ValueId(10), init: FromHost, .. }
///
/// // ConditionOnly carrier (is_digit_pos): Initialize with false
/// CarrierVar { name: "is_digit_pos", host_id: ValueId(15), init: BoolConst(false), .. }
///
/// // Loop-local derived carrier (digit_value): Initialize with local zero (no host slot)
/// CarrierVar { name: "digit_value", host_id: ValueId(0), init: LoopLocalZero, .. }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarrierInit {
    /// No explicit initialization (use host_id value)
    FromHost,
    /// Initialize with bool constant (for ConditionOnly carriers)
    BoolConst(bool),
    /// Initialize with loop-local zero (no host slot; used for derived carriers like digit_value)
    LoopLocalZero,
}

/// Phase 131 P1.5: Exit reconnection mode for JoinInlineBoundary
///
/// Controls whether exit values are reconnected via PHI generation or direct assignment.
/// This separates Normalized shadow (DirectValue) from existing loop patterns (Phi).
///
/// # Design Principle (SSOT)
///
/// - **DirectValue**: Normalized loops prohibit PHI generation. Exit values are directly
///   wired to variable_map using remapped_exit_values from MergeResult.
/// - **Phi**: Existing loop patterns use PHI generation for exit value merging.
///
/// # Example
///
/// ```ignore
/// // Normalized shadow: loop(true) { x = 1; break } → DirectValue
/// JoinInlineBoundary { exit_reconnect_mode: ExitReconnectMode::DirectValue, .. }
///
/// // Traditional loop: loop(i < 3) { sum = sum + i } → Phi
/// JoinInlineBoundary { exit_reconnect_mode: ExitReconnectMode::Phi, .. }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitReconnectMode {
    /// Existing loop patterns: PHI generation for exit value merging
    ///
    /// Used by Pattern 1-4 loops with multiple exit paths.
    /// Exit values are collected into exit PHIs.
    Phi,

    /// Normalized shadow: Direct variable_map update, no PHI generation
    ///
    /// Used by loop(true) { <assign>*; break } pattern.
    /// Exit values are directly wired using MergeResult.remapped_exit_values.
    DirectValue,
}

impl Default for ExitReconnectMode {
    /// Default to Phi mode for backward compatibility
    fn default() -> Self {
        Self::Phi
    }
}

// Phase 229: ConditionAlias removed - redundant with promoted_loopbodylocals
// The naming convention (old_name → "is_<old_name>" or "is_<old_name>_match")
// is sufficient to resolve promoted variables dynamically.

/// Information about a single carrier variable
#[derive(Debug, Clone)]
pub struct CarrierVar {
    /// Variable name (e.g., "sum", "printed", "is_digit_pos")
    pub name: String,
    /// Host ValueId for this variable (MIR側)
    pub host_id: ValueId,
    /// Phase 177-STRUCT: JoinIR側でこのキャリアを表すValueId
    ///
    /// ヘッダPHIのdstや、exitで使う値を記録する。
    /// これにより、index ベースのマッチングを名前ベースに置き換えられる。
    ///
    /// - `Some(vid)`: Header PHI生成後にセットされる
    /// - `None`: まだPHI生成前、または該当なし
    pub join_id: Option<ValueId>,
    /// Phase 227: Role of this carrier (LoopState or ConditionOnly)
    ///
    /// - `LoopState`: Value needed after loop (participates in exit PHI)
    /// - `ConditionOnly`: Only used for loop condition (no exit PHI)
    pub role: CarrierRole,
    /// Phase 228: Initialization policy for header PHI
    ///
    /// - `FromHost`: Use host_id value (regular carriers)
    /// - `BoolConst(false)`: Initialize with false (promoted LoopBodyLocal carriers)
    pub init: CarrierInit,
    /// Phase 78: BindingId for this carrier (dev-only)
    ///
    /// For promoted carriers (e.g., is_digit_pos), this is allocated separately
    /// by CarrierBindingAssigner. For source-derived carriers, this comes from
    /// builder.binding_map.
    ///
    /// Enables type-safe lookup: BindingId → ValueId (join_id) in ConditionEnv.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Source-derived carrier
    /// CarrierVar {
    ///     name: "sum",
    ///     binding_id: Some(BindingId(5)), // from builder.binding_map["sum"]
    ///     ..
    /// }
    ///
    /// // Promoted carrier
    /// CarrierVar {
    ///     name: "is_digit_pos",
    ///     binding_id: Some(BindingId(10)), // allocated by CarrierBindingAssigner
    ///     ..
    /// }
    /// ```
    #[cfg(feature = "normalized_dev")]
    pub binding_id: Option<BindingId>,
}

/// Complete carrier information for a loop
#[derive(Debug, Clone)]
pub struct CarrierInfo {
    /// Loop control variable name (e.g., "i")
    pub loop_var_name: String,
    /// Loop control variable ValueId in host
    pub loop_var_id: ValueId,
    /// Additional carrier variables (e.g., sum, printed)
    pub carriers: Vec<CarrierVar>,
    /// Phase 171-C-5: Trim pattern helper (if this CarrierInfo was created from Trim promotion)
    pub trim_helper: Option<crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper>,
    /// Phase 224: Promoted LoopBodyLocal variables (e.g., "digit_pos" promoted to "is_digit_pos")
    ///
    /// These variables were originally LoopBodyLocal but have been promoted to carriers
    /// during condition promotion (e.g., DigitPosPromoter). The lowerer should skip
    /// LoopBodyLocal checks for these variables.
    ///
    /// Phase 229: Naming convention for promoted carriers:
    /// - DigitPos pattern: "var" → "is_var" (e.g., "digit_pos" → "is_digit_pos")
    /// - Trim pattern: "var" → "is_var_match" (e.g., "ch" → "is_ch_match")
    ///
    /// Condition variable resolution dynamically infers the carrier name from this list.
    pub promoted_loopbodylocals: Vec<String>,

    /// Phase 76: Type-safe promotion tracking (dev-only)
    ///
    /// Maps original BindingId to promoted BindingId, eliminating name-based hacks.
    ///
    /// # Example
    ///
    /// DigitPos promotion:
    /// - Original: BindingId(5) for `"digit_pos"`
    /// - Promoted: BindingId(10) for `"is_digit_pos"`
    /// - Map entry: `promoted_bindings[BindingId(5)] = BindingId(10)`
    ///
    /// This enables type-safe resolution:
    /// ```ignore
    /// if let Some(promoted_bid) = carrier_info.promoted_bindings.get(&original_bid) {
    ///     // Lookup promoted carrier by BindingId (no string matching!)
    /// }
    /// ```
    ///
    /// # Migration Strategy (Phase 76)
    ///
    /// - **Dual Path**: BindingId lookup (NEW) OR name-based fallback (LEGACY)
    /// - **Populated by**: DigitPosPromoter, TrimLoopHelper (Phase 76)
    /// - **Used by**: ConditionEnv::resolve_var_with_binding (Phase 75+)
    /// - **Phase 77**: Remove name-based fallback after full migration
    ///
    /// # Design Notes
    ///
    /// **Q: Why BindingId map instead of name map?**
    /// - **Type Safety**: Compiler-checked binding identity (no typos)
    /// - **Shadowing-Aware**: BindingId distinguishes inner/outer scope vars
    /// - **No Name Collisions**: BindingId is unique even if names shadow
    ///
    /// **Q: Why not remove `promoted_loopbodylocals` immediately?**
    /// - **Legacy Compatibility**: Existing code uses name-based lookup
    /// - **Gradual Migration**: Phase 76 adds BindingId, Phase 77 removes name-based
    /// - **Fail-Safe**: Dual path ensures no regressions during transition
    #[cfg(feature = "normalized_dev")]
    pub promoted_bindings: BTreeMap<BindingId, BindingId>,
}

/// Exit metadata returned by lowerers
///
/// This structure captures the mapping from JoinIR exit values to
/// carrier variable names, enabling dynamic binding generation.
#[derive(Debug, Clone)]
pub struct ExitMeta {
    /// Exit value bindings: (carrier_name, join_exit_value_id)
    ///
    /// Example for Pattern 4:
    /// ```
    /// vec![("sum".to_string(), ValueId(15))]
    /// ```
    /// where ValueId(15) is the k_exit parameter in JoinIR-local space.
    pub exit_values: Vec<(String, ValueId)>,
}

/// Phase 33-14: JoinFragmentMeta - Distinguishes expr result from carrier updates
///
/// ## Purpose
///
/// Separates two distinct use cases for JoinIR loops:
///
/// 1. **Expr Result Pattern** (joinir_min_loop.hako):
///    ```nyash
///    local result = loop(...) { ... }  // Loop used as expression
///    return result
///    ```
///    Here, the k_exit return value is the "expr result" that should go to exit_phi_inputs.
///
/// 2. **Carrier Update Pattern** (trim pattern):
///    ```nyash
///    loop(...) { start = start + 1 }   // Loop used for side effects
///    print(start)                      // Use carrier after loop
///    ```
///    Here, there's no "expr result" - only carrier variable updates.
///
/// ## SSA Correctness
///
/// Previously, exit_phi_inputs mixed expr results with carrier updates, causing:
/// - PHI inputs that referenced undefined remapped values
/// - SSA-undef errors in VM execution
///
/// With JoinFragmentMeta:
/// - `expr_result`: Only goes to exit_phi_inputs (generates PHI for expr value)
/// - `exit_meta`: Only goes to carrier_inputs (updates variable_map via carrier PHIs)
///
/// ## Example: Pattern 2 (joinir_min_loop.hako)
///
/// ```rust
/// JoinFragmentMeta {
///     expr_result: Some(i_exit),  // k_exit returns i as expr value
///     exit_meta: ExitMeta::single("i".to_string(), i_exit),  // Also a carrier
/// }
/// ```
///
/// ## Example: Pattern 3 (trim pattern)
///
/// ```rust
/// JoinFragmentMeta {
///     expr_result: None,  // Loop doesn't return a value
///     exit_meta: ExitMeta::multiple(vec![
///         ("start".to_string(), start_exit),
///         ("end".to_string(), end_exit),
///     ]),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct JoinFragmentMeta {
    /// Expression result ValueId from k_exit (JoinIR-local)
    ///
    /// - `Some(vid)`: Loop is used as expression, k_exit's return value → exit_phi_inputs
    /// - `None`: Loop is used for side effects only, no PHI for expr value
    pub expr_result: Option<ValueId>,

    /// Carrier variable exit bindings (existing ExitMeta)
    ///
    /// Maps carrier names to their JoinIR-local exit values.
    /// These go to carrier_inputs for carrier PHI generation.
    pub exit_meta: ExitMeta,

    /// Phase 132 P1: Continuation contract (SSOT)
    /// Phase 256 P1.7: Changed from BTreeSet<JoinFuncId> to BTreeSet<String>
    ///
    /// JoinIR merge must NOT "guess" continuation functions by name.
    /// Normalized shadow (and other frontends) must explicitly declare which function names
    /// are continuations for the fragment, and merge must follow this contract.
    ///
    /// Merge may still choose to *skip* some continuation functions if and only if they
    /// are structurally "skippable" (pure exit stubs). See merge/instruction_rewriter.rs.
    ///
    /// **Why Strings instead of JoinFuncIds**: The bridge uses JoinFunction.name as the
    /// MirModule function key (e.g., "k_exit"), not "join_func_{id}". The merge code
    /// looks up functions by name, so we must use actual function names here.
    pub continuation_funcs: BTreeSet<String>,
}
