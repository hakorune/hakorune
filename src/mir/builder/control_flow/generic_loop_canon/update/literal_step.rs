use crate::ast::LiteralValue;
use crate::mir::builder::control_flow::facts::canon::generic_loop::UpdateCanon;

use super::UpdateLiteralMatch;

pub(super) fn build_update_canon(matched: UpdateLiteralMatch) -> Option<UpdateCanon> {
    let LiteralValue::Integer(step) = matched.literal else {
        return None;
    };
    Some(UpdateCanon {
        op: matched.op,
        step,
        commuted: matched.commuted,
    })
}
