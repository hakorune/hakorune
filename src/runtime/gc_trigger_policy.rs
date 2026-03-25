#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GcTriggerPolicy {
    collect_sp_interval: Option<u64>,
    collect_alloc_bytes: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GcTriggerDecision {
    reason_bits: u64,
}

impl GcTriggerPolicy {
    pub(crate) fn from_env() -> Self {
        Self {
            collect_sp_interval: crate::config::env::gc_collect_sp_interval(),
            collect_alloc_bytes: crate::config::env::gc_collect_alloc_bytes(),
        }
    }

    pub(crate) fn decide(
        self,
        sp_since_last: u64,
        bytes_since_last: u64,
    ) -> Option<GcTriggerDecision> {
        let sp_hit = self
            .collect_sp_interval
            .map(|n| n > 0 && sp_since_last >= n)
            .unwrap_or(false);
        let alloc_hit = self
            .collect_alloc_bytes
            .map(|n| n > 0 && bytes_since_last >= n)
            .unwrap_or(false);
        if !sp_hit && !alloc_hit {
            return None;
        }
        let mut reason_bits = 0u64;
        if sp_hit {
            reason_bits |= 1;
        }
        if alloc_hit {
            reason_bits |= 2;
        }
        Some(GcTriggerDecision { reason_bits })
    }

    #[cfg(test)]
    fn new_for_tests(collect_sp_interval: Option<u64>, collect_alloc_bytes: Option<u64>) -> Self {
        Self {
            collect_sp_interval,
            collect_alloc_bytes,
        }
    }
}

impl GcTriggerDecision {
    #[inline(always)]
    pub(crate) fn reason_bits(self) -> u64 {
        self.reason_bits
    }

    #[inline(always)]
    pub(crate) fn triggered_by_safepoint(self) -> bool {
        (self.reason_bits & 1) != 0
    }

    #[inline(always)]
    pub(crate) fn triggered_by_alloc(self) -> bool {
        (self.reason_bits & 2) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_trigger_policy_none_when_thresholds_are_disabled() {
        let policy = GcTriggerPolicy::new_for_tests(None, None);
        assert_eq!(policy.decide(10, 4096), None);
    }

    #[test]
    fn gc_trigger_policy_triggers_on_safepoint_interval() {
        let policy = GcTriggerPolicy::new_for_tests(Some(3), None);
        let decision = policy.decide(3, 0).expect("safepoint trigger");
        assert!(decision.triggered_by_safepoint());
        assert!(!decision.triggered_by_alloc());
        assert_eq!(decision.reason_bits(), 1);
    }

    #[test]
    fn gc_trigger_policy_triggers_on_alloc_threshold() {
        let policy = GcTriggerPolicy::new_for_tests(None, Some(64));
        let decision = policy.decide(0, 64).expect("alloc trigger");
        assert!(!decision.triggered_by_safepoint());
        assert!(decision.triggered_by_alloc());
        assert_eq!(decision.reason_bits(), 2);
    }

    #[test]
    fn gc_trigger_policy_triggers_on_both_thresholds() {
        let policy = GcTriggerPolicy::new_for_tests(Some(2), Some(32));
        let decision = policy.decide(2, 32).expect("dual trigger");
        assert!(decision.triggered_by_safepoint());
        assert!(decision.triggered_by_alloc());
        assert_eq!(decision.reason_bits(), 3);
    }

    #[test]
    fn gc_trigger_policy_treats_zero_thresholds_as_disabled() {
        let policy = GcTriggerPolicy::new_for_tests(Some(0), Some(0));
        assert_eq!(policy.decide(100, 100), None);
    }
}
