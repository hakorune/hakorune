use std::path::PathBuf;

use super::Opts;

pub(super) fn resolve_backend_object_output(opts: &Opts) -> PathBuf {
    if let Some(p) = opts.out.clone() {
        p
    } else {
        std::env::temp_dir().join("hako_llvm_out.o")
    }
}
