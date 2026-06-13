//! Thread capability descriptors.
//!
//! These names are descriptor/report vocabulary only. They do not authorize
//! moving or sharing Box values across worker threads.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HakoThreadCapability {
    Send,
    Share,
    ThreadRoot,
}

impl HakoThreadCapability {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Send => "hako.thread.send",
            Self::Share => "hako.thread.share",
            Self::ThreadRoot => "hako.thread.root",
        }
    }

    pub const fn report_field(self) -> &'static str {
        match self {
            Self::Send => "hako_send_capability_descriptor_present",
            Self::Share => "hako_share_capability_descriptor_present",
            Self::ThreadRoot => "hako_thread_root_descriptor_present",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Send => "Box value may be moved to another runtime worker when enforcement lands",
            Self::Share => {
                "Box value may be shared by multiple runtime workers when enforcement lands"
            }
            Self::ThreadRoot => "thread is registered as a future runtime root owner",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadCapabilityDescriptor {
    pub capability: HakoThreadCapability,
    pub key: &'static str,
    pub report_field: &'static str,
    pub description: &'static str,
    pub enforcement_enabled: bool,
}

impl ThreadCapabilityDescriptor {
    pub const fn new(capability: HakoThreadCapability) -> Self {
        Self {
            capability,
            key: capability.key(),
            report_field: capability.report_field(),
            description: capability.description(),
            enforcement_enabled: false,
        }
    }
}

pub const THREAD_CAPABILITY_DESCRIPTORS: &[ThreadCapabilityDescriptor] = &[
    ThreadCapabilityDescriptor::new(HakoThreadCapability::Send),
    ThreadCapabilityDescriptor::new(HakoThreadCapability::Share),
    ThreadCapabilityDescriptor::new(HakoThreadCapability::ThreadRoot),
];

pub fn thread_capability_descriptors() -> &'static [ThreadCapabilityDescriptor] {
    THREAD_CAPABILITY_DESCRIPTORS
}

pub fn thread_capability_report_fields() -> Vec<(&'static str, &'static str)> {
    THREAD_CAPABILITY_DESCRIPTORS
        .iter()
        .map(|descriptor| (descriptor.report_field, "1"))
        .collect()
}

pub fn thread_capability_enforcement_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("hako_send_share_enforced", "0"),
        ("thread_registry_gc_roots_enabled", "0"),
        ("worker_pool_source_route_enabled", "0"),
        ("source_syntax_exposure", "0"),
        ("nowait_os_thread_spawn", "0"),
    ]
}

pub fn thread_capability_inventory_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("hako_send_candidate_count", "0"),
        ("hako_share_candidate_count", "0"),
        ("hako_thread_root_candidate_count", "0"),
        ("rejected_non_send_count", "0"),
        ("rejected_non_share_count", "0"),
        ("thread_root_required_count", "0"),
        ("cross_worker_value_move_enabled", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_capability_keys_are_stable() {
        let keys: Vec<_> = thread_capability_descriptors()
            .iter()
            .map(|descriptor| descriptor.key)
            .collect();

        assert_eq!(
            keys,
            vec!["hako.thread.send", "hako.thread.share", "hako.thread.root"]
        );
    }

    #[test]
    fn thread_capability_descriptors_do_not_enable_enforcement() {
        assert!(thread_capability_descriptors()
            .iter()
            .all(|descriptor| !descriptor.enforcement_enabled));
        assert_eq!(
            thread_capability_enforcement_report_fields(),
            vec![
                ("hako_send_share_enforced", "0"),
                ("thread_registry_gc_roots_enabled", "0"),
                ("worker_pool_source_route_enabled", "0"),
                ("source_syntax_exposure", "0"),
                ("nowait_os_thread_spawn", "0"),
            ]
        );
    }

    #[test]
    fn thread_capability_inventory_starts_closed() {
        assert_eq!(
            thread_capability_inventory_report_fields(),
            vec![
                ("hako_send_candidate_count", "0"),
                ("hako_share_candidate_count", "0"),
                ("hako_thread_root_candidate_count", "0"),
                ("rejected_non_send_count", "0"),
                ("rejected_non_share_count", "0"),
                ("thread_root_required_count", "0"),
                ("cross_worker_value_move_enabled", "0"),
            ]
        );
    }
}
