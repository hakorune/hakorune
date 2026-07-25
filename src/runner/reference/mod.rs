pub(crate) mod raw_vm_reference;
pub(crate) mod cli_profile;
pub(crate) mod normal_file_vm_request;
#[allow(dead_code)]
pub(crate) mod normal_file_vm_frontdoor;
pub mod vm_hako;
// SUPPORT0 keeps the request vocabulary at the runner/MIR boundary while the
// supported opt-in lane remains separate from normal/default routing.
#[allow(dead_code)]
pub(crate) mod raw_vm_reference_request;
pub(crate) mod request;
