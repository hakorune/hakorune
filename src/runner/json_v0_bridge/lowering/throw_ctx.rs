use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static THROW_CTX: RefCell<Option<ThrowCtx>> = RefCell::new(None);
}

#[derive(Clone, Debug)]
pub(super) struct ThrowCtx {
    pub(super) catch_bb: BasicBlockId,
    pub(super) incoming: Vec<(BasicBlockId, ValueId)>,
    pub(super) incoming_vars: Vec<(BasicBlockId, BTreeMap<String, ValueId>)>,
}

impl ThrowCtx {
    fn new(catch_bb: BasicBlockId) -> Self {
        Self {
            catch_bb,
            incoming: Vec::new(),
            incoming_vars: Vec::new(),
        }
    }
}

pub(super) fn set(catch_bb: BasicBlockId) {
    THROW_CTX.with(|slot| {
        *slot.borrow_mut() = Some(ThrowCtx::new(catch_bb));
    });
}

pub(super) fn take() -> Option<ThrowCtx> {
    THROW_CTX.with(|slot| slot.borrow_mut().take())
}

pub(super) fn is_active() -> bool {
    THROW_CTX.with(|slot| slot.borrow().is_some())
}

/// Record a throw from `from_bb` with value `exc_val`. Sets terminator Jump to catch and
/// appends predecessor+value to the incoming list. Returns the catch block id if active.
pub(super) fn record_throw(
    f: &mut MirFunction,
    from_bb: BasicBlockId,
    exc_val: ValueId,
    vars: Option<&BTreeMap<String, ValueId>>,
) -> Option<BasicBlockId> {
    THROW_CTX.with(|slot| {
        if let Some(ctx) = slot.borrow_mut().as_mut() {
            let target = ctx.catch_bb;
            crate::mir::ssot::cf_common::set_jump(f, from_bb, target);
            ctx.incoming.push((from_bb, exc_val));
            if let Some(snapshot) = vars {
                ctx.incoming_vars.push((from_bb, snapshot.clone()));
            }
            Some(target)
        } else {
            None
        }
    })
}
