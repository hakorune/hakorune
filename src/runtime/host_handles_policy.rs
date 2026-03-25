use crate::config::env::HostHandleAllocPolicyMode;

trait HostHandleReusePolicy {
    fn take_reusable_handle(free: &mut Vec<u64>) -> Option<u64>;
    fn issue_fresh_handle(next: &mut u64) -> u64;
    fn recycle_handle(free: &mut Vec<u64>, handle: u64);
}

struct LifoHostHandleReusePolicy;

impl HostHandleReusePolicy for LifoHostHandleReusePolicy {
    #[inline(always)]
    fn take_reusable_handle(free: &mut Vec<u64>) -> Option<u64> {
        free.pop()
    }

    #[inline(always)]
    fn issue_fresh_handle(next: &mut u64) -> u64 {
        let handle = *next;
        *next = next
            .checked_add(1)
            .expect("[host_handles] fresh handle counter overflow");
        handle
    }

    #[inline(always)]
    fn recycle_handle(free: &mut Vec<u64>, handle: u64) {
        free.push(handle);
    }
}

struct NoReuseHostHandleReusePolicy;

impl HostHandleReusePolicy for NoReuseHostHandleReusePolicy {
    #[inline(always)]
    fn take_reusable_handle(_free: &mut Vec<u64>) -> Option<u64> {
        None
    }

    #[inline(always)]
    fn issue_fresh_handle(next: &mut u64) -> u64 {
        LifoHostHandleReusePolicy::issue_fresh_handle(next)
    }

    #[inline(always)]
    fn recycle_handle(_free: &mut Vec<u64>, _handle: u64) {}
}

#[inline(always)]
pub(crate) fn active_host_handle_alloc_policy_mode() -> HostHandleAllocPolicyMode {
    crate::config::env::host_handle_alloc_policy_mode()
}

#[inline(always)]
pub(crate) fn take_reusable_handle(
    mode: HostHandleAllocPolicyMode,
    free: &mut Vec<u64>,
) -> Option<u64> {
    match mode {
        HostHandleAllocPolicyMode::Lifo => LifoHostHandleReusePolicy::take_reusable_handle(free),
        HostHandleAllocPolicyMode::None => NoReuseHostHandleReusePolicy::take_reusable_handle(free),
    }
}

#[inline(always)]
pub(crate) fn issue_fresh_handle(mode: HostHandleAllocPolicyMode, next: &mut u64) -> u64 {
    match mode {
        HostHandleAllocPolicyMode::Lifo => LifoHostHandleReusePolicy::issue_fresh_handle(next),
        HostHandleAllocPolicyMode::None => NoReuseHostHandleReusePolicy::issue_fresh_handle(next),
    }
}

#[inline(always)]
pub(crate) fn recycle_handle(mode: HostHandleAllocPolicyMode, free: &mut Vec<u64>, handle: u64) {
    match mode {
        HostHandleAllocPolicyMode::Lifo => LifoHostHandleReusePolicy::recycle_handle(free, handle),
        HostHandleAllocPolicyMode::None => {
            NoReuseHostHandleReusePolicy::recycle_handle(free, handle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_handles_policy_lifo_reuses_last_dropped_handle() {
        let mut free = vec![1, 3, 7];
        assert_eq!(
            take_reusable_handle(HostHandleAllocPolicyMode::Lifo, &mut free),
            Some(7)
        );
        assert_eq!(free, vec![1, 3]);

        recycle_handle(HostHandleAllocPolicyMode::Lifo, &mut free, 9);
        assert_eq!(free, vec![1, 3, 9]);
    }

    #[test]
    fn host_handles_policy_none_disables_reuse() {
        let mut free = vec![2, 4];
        assert_eq!(
            take_reusable_handle(HostHandleAllocPolicyMode::None, &mut free),
            None
        );
        assert_eq!(free, vec![2, 4]);

        recycle_handle(HostHandleAllocPolicyMode::None, &mut free, 6);
        assert_eq!(free, vec![2, 4]);
    }

    #[test]
    fn host_handles_policy_fresh_issue_is_monotonic() {
        let mut next = 11;
        assert_eq!(
            issue_fresh_handle(HostHandleAllocPolicyMode::Lifo, &mut next),
            11
        );
        assert_eq!(
            issue_fresh_handle(HostHandleAllocPolicyMode::None, &mut next),
            12
        );
        assert_eq!(next, 13);
    }
}
