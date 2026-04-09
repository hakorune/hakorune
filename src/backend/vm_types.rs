/*!
 * VM Core Types
 *
 * Purpose: Error and Value enums used by the VM backend
 * Kept separate to thin vm.rs and allow reuse across helpers.
 */

use crate::ast::Span;
use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox, VoidBox};
use crate::boxes::null_box::NullBox;
use crate::mir::{BasicBlockId, ConstValue};
use std::sync::{Arc, Weak};

/// VM execution error
#[derive(Debug)]
pub enum VMError {
    InvalidValue(String),
    InvalidInstruction(String),
    InvalidBasicBlock(String),
    DivisionByZero,
    StackUnderflow,
    TypeError(String),
    StepBudgetExceeded {
        max_steps: u64,
        steps: u64,
        function: Option<String>,
        current_block: BasicBlockId,
        last_block: Option<BasicBlockId>,
        last_inst: Option<String>,
        last_inst_index: Option<usize>,
        span: Option<Span>,
        source_file: Option<String>,
        mir_dump_path: Option<String>,
        mir_dump_snip_path: Option<String>,
        trace_tail: Option<String>,
        loop_signature: Option<String>,
    },
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            VMError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {}", msg),
            VMError::InvalidBasicBlock(msg) => write!(f, "Invalid basic block: {}", msg),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::TypeError(msg) => write!(f, "Type error: {}", msg),
            VMError::StepBudgetExceeded {
                max_steps,
                steps,
                function,
                current_block,
                last_block,
                last_inst,
                last_inst_index,
                span,
                source_file,
                mir_dump_path,
                mir_dump_snip_path,
                trace_tail,
                loop_signature,
            } => {
                write!(
                    f,
                    "vm step budget exceeded (max_steps={}, steps={}) at bb={}",
                    max_steps, steps, current_block
                )?;
                if let Some(fn_name) = function {
                    write!(f, " fn={}", fn_name)?;
                }
                if let Some(idx) = last_inst_index {
                    write!(f, " last_inst_idx={}", idx)?;
                }
                if let Some(bb) = last_block {
                    write!(f, " last_inst_bb={}", bb)?;
                }
                if let Some(inst) = last_inst {
                    write!(f, " last_inst={}", inst)?;
                }
                match (span, source_file) {
                    (Some(span), Some(file)) => {
                        write!(f, " ({}:{}:{})", file, span.line, span.column)?;
                    }
                    (Some(span), None) => {
                        write!(f, " (line {}, col {})", span.line, span.column)?;
                    }
                    (None, Some(file)) => {
                        write!(f, " ({})", file)?;
                    }
                    _ => {}
                }
                if let Some(path) = mir_dump_path {
                    write!(f, " mir_dump={}", path)?;
                }
                if let Some(path) = mir_dump_snip_path {
                    write!(f, " mir_dump_snip={}", path)?;
                }
                if let Some(trace) = trace_tail {
                    write!(f, " trace_tail=[{}]", trace)?;
                }
                if let Some(sig) = loop_signature {
                    write!(f, " loop_signature={}", sig)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for VMError {}

/// VM value representation
#[derive(Clone)]
pub enum VMValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Future(crate::boxes::future::FutureBox),
    Void,
    BoxRef(Arc<dyn NyashBox>),
    /// Phase 285A0: Weak reference to a Box (non-owning)
    WeakBox(Weak<dyn NyashBox>),
}

// Manual Debug implementation for WeakBox
impl std::fmt::Debug for VMValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMValue::Integer(i) => write!(f, "Integer({})", i),
            VMValue::Float(v) => write!(f, "Float({})", v),
            VMValue::Bool(b) => write!(f, "Bool({})", b),
            VMValue::String(s) => write!(f, "String({:?})", s),
            VMValue::Future(_) => write!(f, "Future(...)"),
            VMValue::Void => write!(f, "Void"),
            VMValue::BoxRef(arc) => write!(f, "BoxRef({})", arc.type_name()),
            VMValue::WeakBox(weak) => {
                if weak.upgrade().is_some() {
                    write!(f, "WeakBox(alive)")
                } else {
                    write!(f, "WeakBox(dropped)")
                }
            }
        }
    }
}

// Manual PartialEq implementation to avoid requiring PartialEq on FutureBox
impl PartialEq for VMValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VMValue::Integer(a), VMValue::Integer(b)) => a == b,
            (VMValue::Float(a), VMValue::Float(b)) => a == b,
            (VMValue::Bool(a), VMValue::Bool(b)) => a == b,
            (VMValue::String(a), VMValue::String(b)) => a == b,
            (VMValue::Void, VMValue::Void) => true,
            (VMValue::Future(_), VMValue::Future(_)) => false,
            (VMValue::BoxRef(_), VMValue::BoxRef(_)) => false,
            // Phase 285A0: WeakBox equality (compare by pointer if both alive)
            (VMValue::WeakBox(a), VMValue::WeakBox(b)) => {
                match (a.upgrade(), b.upgrade()) {
                    (Some(arc_a), Some(arc_b)) => Arc::ptr_eq(&arc_a, &arc_b),
                    (None, None) => true, // Both dropped
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl VMValue {
    /// Convert to NyashBox for output
    pub fn to_nyash_box(&self) -> Box<dyn NyashBox> {
        match self {
            VMValue::Integer(i) => Box::new(IntegerBox::new(*i)),
            VMValue::Float(f) => Box::new(crate::boxes::FloatBox::new(*f)),
            VMValue::Bool(b) => Box::new(BoolBox::new(*b)),
            VMValue::String(s) => Box::new(StringBox::new(s)),
            VMValue::Future(f) => Box::new(f.clone()),
            VMValue::Void => Box::new(VoidBox::new()),
            VMValue::BoxRef(arc_box) => arc_box.share_box(),
            VMValue::WeakBox(weak) => {
                // Upgrade or return void if dropped
                if let Some(arc) = weak.upgrade() {
                    arc.share_box()
                } else {
                    Box::new(VoidBox::new())
                }
            }
        }
    }

    /// Get string representation for printing
    pub fn to_string(&self) -> String {
        match self {
            VMValue::Integer(i) => i.to_string(),
            VMValue::Float(f) => f.to_string(),
            VMValue::Bool(b) => b.to_string(),
            VMValue::String(s) => s.clone(),
            VMValue::Future(f) => f.to_string_box().value,
            VMValue::Void => "void".to_string(),
            VMValue::BoxRef(arc_box) => arc_box.to_string_box().value,
            VMValue::WeakBox(weak) => {
                if weak.upgrade().is_some() {
                    "WeakRef(alive)".to_string()
                } else {
                    "WeakRef(dropped)".to_string()
                }
            }
        }
    }

    /// Phase 285A0: Downgrade a strong BoxRef to a weak reference
    /// Returns None if not a BoxRef
    pub fn downgrade_to_weak(&self) -> Option<VMValue> {
        match self {
            VMValue::BoxRef(arc) => Some(VMValue::WeakBox(Arc::downgrade(arc))),
            _ => None,
        }
    }

    /// Phase 285A0: Upgrade a weak reference to a strong BoxRef
    /// Returns Some(BoxRef) if target is alive, None if dropped
    pub fn upgrade_weak(&self) -> Option<VMValue> {
        match self {
            VMValue::WeakBox(weak) => weak.upgrade().map(VMValue::BoxRef),
            // Non-weak values: return self (already strong)
            _ => Some(self.clone()),
        }
    }

    /// Attempt to convert to integer
    pub fn as_integer(&self) -> Result<i64, VMError> {
        match self {
            VMValue::Integer(i) => Ok(*i),
            _ => Err(VMError::TypeError(format!(
                "Expected integer, got {:?}",
                self
            ))),
        }
    }

    /// Attempt to convert to bool
    pub fn as_bool(&self) -> Result<bool, VMError> {
        match self {
            VMValue::Bool(b) => Ok(*b),
            VMValue::Integer(i) => Ok(*i != 0),
            // Pragmatic coercions for dynamic boxes (preserve legacy semantics)
            VMValue::BoxRef(b) => {
                if let Some(bb) = b.as_any().downcast_ref::<BoolBox>() {
                    return Ok(bb.value);
                }
                if let Some(ib) = b.as_any().downcast_ref::<IntegerBox>() {
                    return Ok(ib.value != 0);
                }
                if let Some(ib) = b
                    .as_any()
                    .downcast_ref::<crate::boxes::integer_box::IntegerBox>()
                {
                    return Ok(ib.value != 0);
                }
                if NullBox::check_null(b.as_ref()) {
                    return Ok(false);
                }
                Err(VMError::TypeError(format!(
                    "Expected bool, got BoxRef({})",
                    b.type_name()
                )))
            }
            VMValue::Void => Ok(false),
            VMValue::Float(f) => Ok(*f != 0.0),
            VMValue::String(s) => Ok(!s.is_empty()),
            VMValue::Future(_) => Ok(true),
            // Phase 285A0: WeakBox truthiness is TypeError (SSOT: types.md:26)
            VMValue::WeakBox(_) => Err(VMError::TypeError(
                "WeakRef cannot be used in boolean context; use upgrade() first".to_string(),
            )),
        }
    }

    /// Convert from NyashBox to VMValue
    pub fn from_nyash_box(nyash_box: Box<dyn crate::box_trait::NyashBox>) -> VMValue {
        if nyash_box
            .as_any()
            .downcast_ref::<crate::boxes::null_box::NullBox>()
            .is_some()
        {
            // Treat NullBox as Void in VMValue to align with `null` literal semantics
            VMValue::Void
        } else if let Some(int_box) = nyash_box.as_any().downcast_ref::<IntegerBox>() {
            VMValue::Integer(int_box.value)
        } else if let Some(bool_box) = nyash_box.as_any().downcast_ref::<BoolBox>() {
            VMValue::Bool(bool_box.value)
        } else if let Some(string_box) = nyash_box.as_any().downcast_ref::<StringBox>() {
            VMValue::String(string_box.value.clone())
        } else if let Some(future_box) = nyash_box
            .as_any()
            .downcast_ref::<crate::boxes::future::FutureBox>()
        {
            VMValue::Future(future_box.clone())
        } else {
            VMValue::BoxRef(Arc::from(nyash_box))
        }
    }
}

impl From<&ConstValue> for VMValue {
    fn from(const_val: &ConstValue) -> Self {
        match const_val {
            ConstValue::Integer(i) => VMValue::Integer(*i),
            ConstValue::Float(f) => VMValue::Float(*f),
            ConstValue::Bool(b) => VMValue::Bool(*b),
            ConstValue::String(s) => VMValue::String(s.clone()),
            ConstValue::Null => VMValue::Void,
            ConstValue::Void => VMValue::Void,
        }
    }
}
