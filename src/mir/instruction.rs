/*!
 * MIR Instruction Set (canonical enum for current MIR vocabulary)
 *
 * SSA-form instructions with effect tracking for optimization
 * Backend-specific acceptance is defined in `src/mir/contracts/backend_core_ops.rs`.
 */

use super::{EdgeArgs, EffectMask, ValueId};
use crate::mir::definitions::Callee; // Import Callee from unified definitions
use crate::mir::types::{
    BarrierOp, BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, WeakRefOp,
};

// (unused imports removed)

/// MIR instruction types (full enum; backend allowlists are contract-driven)
#[derive(Debug, Clone, PartialEq)]
pub enum MirInstruction {
    // === Constants and Values ===
    /// Load a constant value
    /// `%dst = const value`
    Const { dst: ValueId, value: ConstValue },

    // === Arithmetic Operations ===
    /// Binary arithmetic operation
    /// `%dst = %lhs op %rhs`
    BinOp {
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    },

    /// Unary operation
    /// `%dst = op %operand`
    UnaryOp {
        dst: ValueId,
        op: UnaryOp,
        operand: ValueId,
    },

    // === Comparison Operations ===
    /// Compare two values
    /// `%dst = %lhs cmp %rhs`
    Compare {
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    },

    // === Memory Operations ===
    /// Load from memory/variable
    /// `%dst = load %ptr`
    Load { dst: ValueId, ptr: ValueId },

    /// Store to memory/variable
    /// `store %value -> %ptr`
    Store { value: ValueId, ptr: ValueId },

    /// Canonical object field read.
    /// `%dst = field.get %base .field`
    FieldGet {
        dst: ValueId,
        base: ValueId,
        field: String,
        declared_type: Option<MirType>,
    },

    /// Canonical object field write.
    /// `field.set %base .field = %value`
    FieldSet {
        base: ValueId,
        field: String,
        value: ValueId,
        declared_type: Option<MirType>,
    },

    // === Function Calls ===
    /// Call a function with type-safe target resolution
    /// `%dst = call %func(%args...)` (legacy)
    /// `%dst = call Global("print")(%args...)` (new)
    ///
    /// Phase 1 Migration: Both func and callee fields present
    /// - callee: Some(_) -> Use new type-safe resolution (preferred)
    /// - callee: None -> Fall back to legacy string-based resolution
    Call {
        dst: Option<ValueId>,
        func: ValueId,          // Legacy: string-based resolution (deprecated)
        callee: Option<Callee>, // New: type-safe resolution (preferred)
        args: Vec<ValueId>,
        effects: EffectMask,
    },

    /// Create a function value (FunctionBox) from params/body and optional captures
    /// `%dst = new_closure [params] {body} [captures...]`
    /// Minimal lowering support: captures may be empty; 'me' is optional.
    NewClosure {
        dst: ValueId,
        params: Vec<String>,
        /// NCL-1 canonical: points to `MirModule.metadata.closure_bodies`.
        /// Legacy route may still carry inline body temporarily.
        body_id: Option<super::function::ClosureBodyId>,
        /// Legacy inline body (will be externalized by canonicalization).
        body: Vec<crate::ast::ASTNode>,
        /// Pairs of (name, value) to capture by value
        captures: Vec<(String, ValueId)>,
        /// Optional 'me' value to capture weakly if it is a BoxRef at runtime
        me: Option<ValueId>,
    },

    // === Control Flow ===
    /// Conditional branch
    /// `br %condition -> %then_bb, %else_bb`
    Branch {
        condition: ValueId,
        then_bb: super::BasicBlockId,
        else_bb: super::BasicBlockId,
        /// Optional edge args for then branch (Phase 260 P0)
        then_edge_args: Option<EdgeArgs>,
        /// Optional edge args for else branch (Phase 260 P0)
        else_edge_args: Option<EdgeArgs>,
    },

    /// Unconditional jump
    /// `jmp %target_bb`
    Jump {
        target: super::BasicBlockId,
        /// Optional edge args for jump (Phase 260 P0)
        edge_args: Option<EdgeArgs>,
    },

    /// Return from function
    /// `ret %value` or `ret void`
    Return { value: Option<ValueId> },

    // === SSA Phi Function ===
    /// SSA phi function for merging values from different paths
    /// `%dst = phi [%val1 from %bb1, %val2 from %bb2, ...]`
    ///
    /// # Phase 63-6: Type Hint Support
    ///
    /// `type_hint` field stores type information from JoinIR (Select/IfMerge)
    /// to enable type inference without scanning PHI inputs.
    /// - `Some(MirType)`: Type is known from JoinIR (P1 cases: IfSelectTest.simple/local)
    /// - `None`: Type must be inferred from PHI inputs (legacy behavior)
    Phi {
        dst: ValueId,
        inputs: Vec<(super::BasicBlockId, ValueId)>,
        type_hint: Option<super::MirType>, // Phase 63-6: JoinIR type hint
    },

    // === Box Operations ===
    /// Create a new Box instance
    /// `%dst = new_box "BoxType"(%args...)`
    NewBox {
        dst: ValueId,
        box_type: String,
        args: Vec<ValueId>,
    },

    // === Type Operations (Unified PoC) ===
    /// Unified type operation (PoC): Check or Cast
    /// `%dst = typeop(check|cast, %value, Type)`
    TypeOp {
        dst: ValueId,
        op: TypeOpKind,
        value: ValueId,
        ty: MirType,
    },

    // === Special Operations ===
    /// Copy a value (for optimization passes)
    /// `%dst = copy %src`
    Copy { dst: ValueId, src: ValueId },

    /// Debug/introspection instruction
    /// `debug %value "message"`
    Debug { value: ValueId, message: String },

    /// Phase 287: Keep values alive until scope end (for PHI, liveness analysis)
    /// `keepalive %v1 %v2 ...`
    /// Effect: PURE (no side effects, only affects DCE/liveness)
    /// Prevents DCE from removing values that may be needed for PHI nodes.
    KeepAlive { values: Vec<ValueId> },

    /// Phase 287: Release strong references (for variable overwrite, scope exit)
    /// `release_strong %v1 %v2 ...`
    /// Effect: WRITE (modifies reference count, may trigger deallocation)
    /// Releases all strong references to the specified values, including SSA aliases.
    ReleaseStrong { values: Vec<ValueId> },

    // === Control Flow & Exception Handling (Phase 5) ===
    /// Throw an exception
    /// `throw %exception_value`
    Throw {
        exception: ValueId,
        effects: EffectMask,
    },

    /// Catch handler setup (landing pad for exceptions)
    /// `catch %exception_type -> %handler_bb`
    Catch {
        exception_type: Option<String>, // None = catch-all
        exception_value: ValueId,       // Where to store caught exception
        handler_bb: super::BasicBlockId,
    },

    /// Safepoint instruction (no-op for now, can be used for GC/debugging)
    /// `safepoint`
    Safepoint,

    // === Phase 6: Box Reference Operations ===
    /// Create a new reference to a Box
    /// `%dst = ref_new %box`
    RefNew { dst: ValueId, box_val: ValueId },

    // === Unified PoC: WeakRef/Barrier (flags-only scaffolding) ===
    /// Unified weak reference op (PoC)
    /// `%dst = weakref new %box` or `%dst = weakref load %weak`
    WeakRef {
        dst: ValueId,
        op: WeakRefOp,
        value: ValueId,
    },

    /// Unified barrier op (PoC)
    /// `barrier read %ptr` or `barrier write %ptr`
    Barrier { op: BarrierOp, ptr: ValueId },

    // === Phase 7: Async/Future Operations ===
    /// Create a new Future with initial value
    /// `%dst = future_new %value`
    FutureNew { dst: ValueId, value: ValueId },

    /// Set Future value and mark as ready
    /// `future_set %future = %value`
    FutureSet { future: ValueId, value: ValueId },

    /// Wait for Future completion and get value
    /// `%dst = await %future`
    Await { dst: ValueId, future: ValueId },

    /// Phase 256 P1.5: Select instruction (ternary conditional)
    /// Equivalent to: dst = cond ? then_val : else_val
    /// Used by JoinIR for conditional carrier updates in loop routes
    Select {
        dst: ValueId,
        cond: ValueId,     // Boolean condition
        then_val: ValueId, // Value when cond is true
        else_val: ValueId, // Value when cond is false
    },
}

// Method implementations have been moved to src/mir/instruction/methods.rs
#[path = "instruction/methods.rs"]
mod methods;

// Display implementation has been moved to src/mir/instruction/display.rs
#[path = "instruction/display.rs"]
mod display;

// Tests have been moved to src/mir/instruction/tests.rs for better organization
#[cfg(test)]
#[path = "instruction/tests.rs"]
mod tests;
