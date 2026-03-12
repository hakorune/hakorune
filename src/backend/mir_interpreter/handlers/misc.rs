use super::*;
use crate::config::env;
use crate::runtime::get_global_ring0;

impl MirInterpreter {
    pub(super) fn handle_debug(&mut self, message: &str, value: ValueId) -> Result<(), VMError> {
        let v = self.reg_load(value)?;
        if env::cli_verbose_enabled()
            || std::env::var("NYASH_MIR_DEBUG_LOG").ok().as_deref() == Some("1")
        {
            get_global_ring0()
                .log
                .debug(&format!("[mir-debug] {} => {:?}", message, v));
        }
        Ok(())
    }
}
