//! Backend-neutral WeakRef target-token and lifecycle policy.

use crate::box_trait::NyashBox;
use std::sync::{Arc, Weak};

pub const WEAKREF_LOAD_INVALID_INPUT_TAG: &str = "[type/weakref_load_invalid_input]";

/// Weak allocation identity remains comparable after the strong target drops.
pub fn target_token_eq<T: ?Sized>(left: &Weak<T>, right: &Weak<T>) -> bool {
    Weak::ptr_eq(left, right)
}

/// Upgrade only a target that is both allocated and logically usable.
pub fn upgrade_usable_target(target: &Weak<dyn NyashBox>) -> Option<Arc<dyn NyashBox>> {
    let strong = target.upgrade()?;
    if target_is_logically_dead(strong.as_ref()) {
        None
    } else {
        Some(strong)
    }
}

fn target_is_logically_dead(target: &dyn NyashBox) -> bool {
    if crate::finalization::is_finalized(target.box_id()) {
        return true;
    }
    target
        .as_any()
        .downcast_ref::<crate::instance_v2::InstanceBox>()
        .is_some_and(crate::instance_v2::InstanceBox::is_finalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn instance(name: &str) -> Arc<crate::instance_v2::InstanceBox> {
        Arc::new(crate::instance_v2::InstanceBox::from_declaration(
            name.to_string(),
            Vec::new(),
            HashMap::new(),
        ))
    }

    #[test]
    fn target_token_survives_target_drop() {
        let target: Arc<dyn NyashBox> = instance("TokenTarget");
        let left = Arc::downgrade(&target);
        let right = Arc::downgrade(&target);
        drop(target);

        assert!(target_token_eq(&left, &right));
    }

    #[test]
    fn different_dropped_targets_remain_unequal() {
        let first: Arc<dyn NyashBox> = instance("FirstTarget");
        let second: Arc<dyn NyashBox> = instance("SecondTarget");
        let first_weak = Arc::downgrade(&first);
        let second_weak = Arc::downgrade(&second);
        drop(first);
        drop(second);

        assert!(!target_token_eq(&first_weak, &second_weak));
    }

    #[test]
    fn finalized_instance_cannot_upgrade_while_arc_remains() {
        let instance = instance("FinalizedTarget");
        let target: Arc<dyn NyashBox> = instance.clone();
        let weak = Arc::downgrade(&target);
        instance.fini().expect("fini should succeed");

        assert!(weak.upgrade().is_some());
        assert!(upgrade_usable_target(&weak).is_none());
    }
}
