#![doc = "ring0 layer guard: defines responsibilities and import boundaries"]
#[allow(dead_code)]
pub const LAYER_NAME: &str = "ring0";
#[allow(dead_code)]
pub const ALLOWED_IMPORTS: &[&str] = &[
    "runner", "config", "backend", "hostbridge", "runtime",
];
#[allow(dead_code)]
pub const FORBIDDEN_IMPORTS: &[&str] = &[
    // Do not depend on plugins directly from ring0
    "plugins", "providers::ring1",
];

