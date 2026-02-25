use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Default)]
pub struct MacroCaps {
    pub io: bool,
    pub net: bool,
    pub env: bool,
}

#[derive(Debug, Clone)]
pub struct MacroCtx {
    pub caps: MacroCaps,
}

impl MacroCtx {
    pub fn from_env() -> Self {
        MacroCtx {
            caps: MacroCaps {
                io: crate::config::env::env_flag("NYASH_MACRO_CAP_IO").unwrap_or(false),
                net: crate::config::env::env_flag("NYASH_MACRO_CAP_NET").unwrap_or(false),
                env: crate::config::env::env_flag("NYASH_MACRO_CAP_ENV").unwrap_or(false),
            },
        }
    }

    pub fn gensym(&self, prefix: &str) -> String {
        gensym(prefix)
    }

    pub fn report(&self, level: &str, message: &str) {
        crate::macro_log!("[macro][{}] {}", level, message);
    }

    pub fn get_env(&self, key: &str) -> Option<String> {
        if !self.caps.env {
            return None;
        }
        std::env::var(key).ok()
    }
}

static GENSYM_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gensym(prefix: &str) -> String {
    let n = GENSYM_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}_{}", prefix, n)
}
