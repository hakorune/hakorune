pub mod types;
pub mod value_id;

pub use types::{
    BarrierOp, BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, WeakRefOp,
};
pub use value_id::{LocalId, LocalIdGenerator, ValueId, ValueIdGenerator};

