//! Stable, opt-in timing output for MIR compilation stages.

use std::cell::RefCell;
use std::time::Duration;

pub(crate) fn trace_stage(stage: &str, elapsed: Duration) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!(
            "[mir-compile/timing] stage={} elapsed_ms={}",
            stage,
            elapsed.as_millis()
        );
    }
}

pub(crate) fn trace_count(stage: &str, count: usize) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!("[mir-compile/timing] stage={} count={}", stage, count);
    }
}

/// Classifies the data a refresh stage observes or mutates. This is an
/// observation label only; it does not authorize a pass to change its owner.
#[derive(Clone, Copy)]
pub(crate) enum RefreshAccessClass {
    MetadataReadWrite,
    MirReadWrite,
}

impl RefreshAccessClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::MetadataReadWrite => "metadata_read_write",
            Self::MirReadWrite => "mir_read_write",
        }
    }
}

#[derive(Default)]
struct RefreshWalkObservation {
    caller: &'static str,
    family: &'static str,
    stage: &'static str,
    access: &'static str,
    intermediate_consumer: bool,
    function_visits: usize,
    block_visits: usize,
    instruction_visits: usize,
}

thread_local! {
    static ACTIVE_REFRESH_WALK: RefCell<Option<RefreshWalkObservation>> = const {
        RefCell::new(None)
    };
}

/// Observe one existing refresh stage without adding a semantic product or
/// changing the stage's return value. The scope is only active for the
/// opt-in compile trace, so the default path has no counter bookkeeping.
pub(crate) fn with_refresh_walk_scope<T>(
    caller: &'static str,
    family: &'static str,
    stage: &'static str,
    access: RefreshAccessClass,
    intermediate_consumer: bool,
    f: impl FnOnce() -> T,
) -> T {
    if !crate::config::env::builder_mir_compile_trace() {
        return f();
    }

    let previous = ACTIVE_REFRESH_WALK.with(|slot| {
        slot.replace(Some(RefreshWalkObservation {
            caller,
            family,
            stage,
            access: access.as_str(),
            intermediate_consumer,
            ..RefreshWalkObservation::default()
        }))
    });
    let result = f();
    let observation = ACTIVE_REFRESH_WALK.with(|slot| slot.replace(previous));
    if let Some(observation) = observation {
        eprintln!(
            "[mir-compile/walk] caller={} family={} stage={} access={} intermediate_consumer={} functions={} blocks={} instructions={}",
            observation.caller,
            observation.family,
            observation.stage,
            observation.access,
            observation.intermediate_consumer,
            observation.function_visits,
            observation.block_visits,
            observation.instruction_visits,
        );
    }
    result
}

pub(crate) fn trace_refresh_function_visit() {
    ACTIVE_REFRESH_WALK.with(|slot| {
        if let Some(observation) = slot.borrow_mut().as_mut() {
            observation.function_visits += 1;
        }
    });
}

pub(crate) fn trace_refresh_block_visit() {
    ACTIVE_REFRESH_WALK.with(|slot| {
        if let Some(observation) = slot.borrow_mut().as_mut() {
            observation.block_visits += 1;
        }
    });
}

pub(crate) fn trace_refresh_instruction_visit() {
    ACTIVE_REFRESH_WALK.with(|slot| {
        if let Some(observation) = slot.borrow_mut().as_mut() {
            observation.instruction_visits += 1;
        }
    });
}
