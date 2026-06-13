//! Runtime scheduler route descriptors.
//!
//! These descriptors are report/check vocabulary only. They do not select a
//! scheduler route and do not widen `.hako` source-level concurrency semantics.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HakoSchedulerRoute {
    InlineResolvedFuture,
    CooperativeTask,
    WorkerPoolTask,
}

impl HakoSchedulerRoute {
    pub const fn key(self) -> &'static str {
        match self {
            Self::InlineResolvedFuture => "hako.scheduler.inline_resolved_future",
            Self::CooperativeTask => "hako.scheduler.cooperative_task",
            Self::WorkerPoolTask => "hako.scheduler.worker_pool_task",
        }
    }

    pub const fn report_field(self) -> &'static str {
        match self {
            Self::InlineResolvedFuture => {
                "scheduler_route_inline_resolved_future_descriptor_present"
            }
            Self::CooperativeTask => "scheduler_route_cooperative_task_descriptor_present",
            Self::WorkerPoolTask => "scheduler_route_worker_pool_task_descriptor_present",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::InlineResolvedFuture => {
                "Phase-0 future route; expression may run before FutureNew"
            }
            Self::CooperativeTask => "queued task route under cooperative scheduler polling",
            Self::WorkerPoolTask => {
                "runtime worker-pool route; source semantics remain unchanged"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerRouteDescriptor {
    pub route: HakoSchedulerRoute,
    pub key: &'static str,
    pub report_field: &'static str,
    pub description: &'static str,
}

impl SchedulerRouteDescriptor {
    pub const fn new(route: HakoSchedulerRoute) -> Self {
        Self {
            route,
            key: route.key(),
            report_field: route.report_field(),
            description: route.description(),
        }
    }
}

pub const SCHEDULER_ROUTE_DESCRIPTORS: &[SchedulerRouteDescriptor] = &[
    SchedulerRouteDescriptor::new(HakoSchedulerRoute::InlineResolvedFuture),
    SchedulerRouteDescriptor::new(HakoSchedulerRoute::CooperativeTask),
    SchedulerRouteDescriptor::new(HakoSchedulerRoute::WorkerPoolTask),
];

pub fn scheduler_route_descriptors() -> &'static [SchedulerRouteDescriptor] {
    SCHEDULER_ROUTE_DESCRIPTORS
}

pub fn scheduler_route_report_fields() -> Vec<(&'static str, &'static str)> {
    SCHEDULER_ROUTE_DESCRIPTORS
        .iter()
        .map(|descriptor| (descriptor.report_field, "1"))
        .collect()
}

pub fn scheduler_route_activation_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("scheduler_route_worker_pool_default_enabled", "0"),
        ("worker_pool_source_route_enabled", "0"),
        ("source_level_thread_syntax", "0"),
        ("nowait_os_thread_spawn", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_route_keys_are_stable() {
        let keys: Vec<_> = scheduler_route_descriptors()
            .iter()
            .map(|descriptor| descriptor.key)
            .collect();

        assert_eq!(
            keys,
            vec![
                "hako.scheduler.inline_resolved_future",
                "hako.scheduler.cooperative_task",
                "hako.scheduler.worker_pool_task",
            ]
        );
    }

    #[test]
    fn scheduler_route_activation_keeps_source_threads_closed() {
        assert_eq!(
            scheduler_route_activation_report_fields(),
            vec![
                ("scheduler_route_worker_pool_default_enabled", "0"),
                ("worker_pool_source_route_enabled", "0"),
                ("source_level_thread_syntax", "0"),
                ("nowait_os_thread_spawn", "0"),
            ]
        );
    }
}
