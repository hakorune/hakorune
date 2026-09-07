//! Process entry for the selected normalized-status lifecycle artifact.
//! Runtime state and startup/flush remain owned by nyash_kernel.

/// Fixed entry ABI: magic, little-endian revision 1, normalized-i64 kind 1.
#[used]
#[link_section = ".nyash.entry_abi.v1"]
#[export_name = "nyash_lifecycle_entry_abi_v1"]
pub static ENTRY_ABI: [u8; 16] = *b"NYENTRY1\x01\x00\x00\x00\x01\x00\x00\x00";

#[no_mangle]
pub extern "C" fn main() -> i32 {
    fn invoke() -> i64 {
        extern "C" {
            fn ny_main() -> i64;
        }
        // SAFETY: the selected generated object implements the normalized ABI.
        unsafe { ny_main() }
    }
    nyash_kernel::run_normalized_entry(invoke)
}
