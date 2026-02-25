//! LoopForm helper alias
//!
//! Alias to ControlForm::LoopShape so generic JoinIR lowering can depend on a
//! stable loop shape representation without pulling in builder details.

pub type LoopForm = crate::mir::control_form::LoopShape;
