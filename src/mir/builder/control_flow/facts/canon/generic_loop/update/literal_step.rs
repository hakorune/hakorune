use crate::ast::LiteralValue;

use super::{UpdateCanon, UpdateLiteralMatch};

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
