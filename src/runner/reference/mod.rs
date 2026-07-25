pub(crate) mod raw_vm_reference;
pub mod vm_hako;
// PROFILE0 vocabulary is retained while CANARY0 consumes only the selected
// fields; remove or narrow this allowance when the profile vocabulary is
// retired after normal-entry cutover.
#[allow(dead_code)]
pub(crate) mod raw_vm_reference_request;
