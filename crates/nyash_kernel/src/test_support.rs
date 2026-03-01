pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) fn with_env_vars<F: FnOnce()>(pairs: &[(&str, &str)], f: F) {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let prev: Vec<(String, Option<String>)> = pairs
        .iter()
        .map(|(k, _)| ((*k).to_string(), std::env::var(k).ok()))
        .collect();
    for (k, v) in pairs {
        std::env::set_var(k, v);
    }
    f();
    for (k, prev_v) in prev {
        if let Some(v) = prev_v {
            std::env::set_var(&k, v);
        } else {
            std::env::remove_var(&k);
        }
    }
}

pub(crate) fn with_env_var<F: FnOnce()>(key: &str, value: &str, f: F) {
    with_env_vars(&[(key, value)], f);
}
