//! Reserved `@rune Profile(...)` expansion targets.
//!
//! This module is parser/MIR-neutral on purpose: parsers validate reserved
//! names here, while MIR plan builders expand profiles into primitive facts.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuneProfileExpansion {
    pub name: &'static str,
    pub hints: &'static [&'static str],
    pub lowerings: &'static [&'static str],
    pub contracts: &'static [&'static str],
    pub capabilities: &'static [&'static str],
}

const ALLOCATOR_FAST: RuneProfileExpansion = RuneProfileExpansion {
    name: "allocator.fast",
    hints: &["hot"],
    lowerings: &["inline_required"],
    contracts: &["no_alloc", "no_safepoint"],
    capabilities: &["hako.mem", "hako.ptr", "hako.tls"],
};

const ALLOCATOR_SLOW: RuneProfileExpansion = RuneProfileExpansion {
    name: "allocator.slow",
    hints: &["cold", "noinline"],
    lowerings: &[],
    contracts: &[],
    capabilities: &["hako.mem", "hako.osvm", "hako.gc"],
};

const SUBSTRATE_LEAF: RuneProfileExpansion = RuneProfileExpansion {
    name: "substrate.leaf",
    hints: &["inline"],
    lowerings: &["inline_required"],
    contracts: &["no_alloc", "no_safepoint"],
    capabilities: &["hako.mem", "hako.buf", "hako.ptr"],
};

const INTRINSIC_LEAF: RuneProfileExpansion = RuneProfileExpansion {
    name: "intrinsic.leaf",
    hints: &["inline"],
    lowerings: &[],
    contracts: &["no_alloc", "no_safepoint"],
    capabilities: &[],
};

const RAW_LAYOUT: RuneProfileExpansion = RuneProfileExpansion {
    name: "raw.layout",
    hints: &[],
    lowerings: &[],
    contracts: &[],
    capabilities: &["hako.ptr"],
};

pub const SUPPORTED_PROFILE_NAMES_MSG: &str =
    "allocator.fast|allocator.slow|substrate.leaf|intrinsic.leaf|raw.layout";

pub fn expansion(name: &str) -> Option<&'static RuneProfileExpansion> {
    match name {
        "allocator.fast" => Some(&ALLOCATOR_FAST),
        "allocator.slow" => Some(&ALLOCATOR_SLOW),
        "substrate.leaf" => Some(&SUBSTRATE_LEAF),
        "intrinsic.leaf" => Some(&INTRINSIC_LEAF),
        "raw.layout" => Some(&RAW_LAYOUT),
        _ => None,
    }
}

pub fn supported_name(name: &str) -> bool {
    expansion(name).is_some()
}
