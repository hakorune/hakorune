//! External function implementations for plugin loader v2
//!
//! This module contains all `env.*` external function implementations
//! that were previously in a large switch statement in loader.rs

use crate::bid::{BidError, BidResult};
use crate::box_trait::IntegerBox;
use crate::box_trait::{BoolBox, NyashBox, StringBox, VoidBox};
use crate::boxes::array::ArrayBox;
use crate::boxes::future::FutureBox;
use crate::boxes::map_box::MapBox;
use crate::boxes::null_box::NullBox;
use crate::boxes::result::NyashResultBox;
use crate::boxes::token_box::TokenBox;
use crate::runtime::get_global_ring0;
use crate::runtime::global_hooks;
use crate::runtime::modules_registry;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Handle external function calls from the runtime
pub fn extern_call(
    iface_name: &str,
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    if std::env::var("HAKO_CALL_TRACE").ok().as_deref() == Some("1") {
        if should_trace_call_extern(iface_name, method_name) {
            get_global_ring0()
                .log
                .debug(&format!("[call:{}.{}]", iface_name, method_name));
        }
    }
    match iface_name {
        "env.console" => handle_console(method_name, args),
        "env.file" => handle_file(method_name, args),
        "env.result" => handle_result(method_name, args),
        "env.modules" => handle_modules(method_name, args),
        "env.task" => handle_task(method_name, args),
        "env.debug" => handle_debug(method_name, args),
        "env.runtime" => handle_runtime(method_name, args),
        "env.future" => handle_future(method_name, args),
        "env.mirbuilder" => handle_mirbuilder(method_name, args),
        "env.codegen" => super::compat_codegen_receiver::handle_codegen(method_name, args),
        "env.box_introspect" => handle_box_introspect(method_name, args),
        _ => reject_unknown(iface_name, method_name),
    }
}

#[inline]
fn reject_unknown(_iface_name: &str, _method_name: &str) -> BidResult<Option<Box<dyn NyashBox>>> {
    Err(BidError::PluginError)
}

fn should_trace_call_extern(target: &str, method: &str) -> bool {
    if let Ok(flt) = std::env::var("HAKO_CALL_TRACE_FILTER") {
        let key = format!("{}.{}", target, method);
        for pat in flt.split(',') {
            let p = pat.trim();
            if p.is_empty() {
                continue;
            }
            if p == method || p == key {
                return true;
            }
        }
        return false;
    }
    true
}

fn handle_file(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "read" => {
            let path = args.get(0).ok_or(BidError::PluginError)?.to_string_box().value;
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(Box::new(StringBox::new(&text)))),
                Err(_) => Ok(Some(Box::new(VoidBox::new()))),
            }
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.console.* methods
fn handle_console(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "log" => {
            let trace = std::env::var("NYASH_CONSOLE_TRACE").ok().as_deref() == Some("1");
            for a in args {
                let s = a.to_string_box().value;
                if trace {
                    get_global_ring0().log.debug(&format!(
                        "[console.trace] len={} text=<{:.64}>",
                        s.len(),
                        s
                    ));
                }
                println!("{}", s);
            }
            Ok(None)
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.result.* methods
fn handle_result(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "ok" => {
            // Wrap the first argument as Result.Ok; if missing, use Void
            let v = args
                .get(0)
                .map(|b| b.clone_box())
                .unwrap_or_else(|| Box::new(VoidBox::new()));
            Ok(Some(Box::new(NyashResultBox::new_ok(v))))
        }
        "err" => {
            // Wrap the first argument as Result.Err; if missing, synthesize a StringBox("Error")
            let e: Box<dyn NyashBox> = args
                .get(0)
                .map(|b| b.clone_box())
                .unwrap_or_else(|| Box::new(StringBox::new("Error")));
            Ok(Some(Box::new(NyashResultBox::new_err(e))))
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.modules.* methods
fn handle_modules(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "set" => {
            if args.len() >= 2 {
                let key = args[0].to_string_box().value;
                let val = args[1].clone_box();
                modules_registry::set(key, val);
            }
            Ok(None)
        }
        "get" => {
            if let Some(k) = args.get(0) {
                let key = k.to_string_box().value;
                if let Some(v) = modules_registry::get(&key) {
                    return Ok(Some(v));
                }
            }
            Ok(Some(Box::new(VoidBox::new())))
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.task.* methods
fn handle_task(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "cancelCurrent" => {
            global_hooks::cancel_current_group_with_reason("scope-cancelled");
            Ok(None)
        }
        "currentToken" => {
            let tok = global_hooks::current_group_token();
            let tb = TokenBox::from_token(tok);
            Ok(Some(Box::new(tb)))
        }
        "spawn" => handle_task_spawn(args),
        "wait" => handle_task_wait(args),
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.task.spawn method
fn handle_task_spawn(args: &[Box<dyn NyashBox>]) -> BidResult<Option<Box<dyn NyashBox>>> {
    if let Some(b) = args.get(0) {
        // The plugin loader originally included additional spawn logic,
        // but we keep the simplified version here for now
        // TODO: Implement full task spawning logic
        Ok(Some(b.clone_box()))
    } else {
        Ok(None)
    }
}

/// Handle env.task.wait method
fn handle_task_wait(_args: &[Box<dyn NyashBox>]) -> BidResult<Option<Box<dyn NyashBox>>> {
    // Task wait is not yet implemented in the extracted module
    // This functionality will be added when properly integrating with future system
    Err(BidError::PluginError)
}

/// Handle env.debug.* methods
fn handle_debug(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "trace" => {
            if std::env::var("NYASH_DEBUG_TRACE").ok().as_deref() == Some("1") {
                for a in args {
                    get_global_ring0()
                        .log
                        .debug(&format!("[debug.trace] {}", a.to_string_box().value));
                }
            }
            Ok(None)
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.runtime.* methods
fn handle_runtime(
    method_name: &str,
    _args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "checkpoint" => {
            if crate::config::env::runtime_checkpoint_trace() {
                get_global_ring0().log.debug("[runtime.checkpoint] reached");
            }
            global_hooks::safepoint_and_poll();
            Ok(None)
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.future.* methods
fn handle_future(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "new" | "birth" => {
            let fut = FutureBox::new();
            if let Some(v) = args.get(0) {
                fut.set_result(v.clone_box());
            }
            global_hooks::register_future_to_current_group(&fut);
            Ok(Some(Box::new(fut)))
        }
        "set" => {
            if args.len() >= 2 {
                if let Some(fut) = args[0].as_any().downcast_ref::<FutureBox>() {
                    fut.set_result(args[1].clone_box());
                }
            }
            Ok(None)
        }
        "await" => handle_future_await(args),
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.box_introspect.* methods
pub fn handle_box_introspect(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "kind" => {
            let value = args.get(0).ok_or(BidError::PluginError)?;
            let info = build_box_info(value.as_ref());
            if std::env::var("NYASH_BOX_INTROSPECT_TRACE").ok().as_deref() == Some("1") {
                get_global_ring0().log.debug(&format!(
                    "[box_introspect:plugin] kind={} type_name={} is_map={} is_array={}",
                    info.get(Box::new(StringBox::new("kind")))
                        .to_string_box()
                        .value,
                    info.get(Box::new(StringBox::new("type_name")))
                        .to_string_box()
                        .value,
                    info.get(Box::new(StringBox::new("is_map")))
                        .to_string_box()
                        .value,
                    info.get(Box::new(StringBox::new("is_array")))
                        .to_string_box()
                        .value,
                ));
            }
            Ok(Some(Box::new(info)))
        }
        _ => Err(BidError::PluginError),
    }
}

fn build_box_info(value: &dyn NyashBox) -> MapBox {
    let info = MapBox::new();
    insert_string(&info, "kind", &classify_kind(value));
    insert_string(&info, "type_name", value.type_name());
    insert_string(&info, "type_id", &format!("{:016x}", type_id_hash(value)));
    insert_bool(
        &info,
        "is_map",
        value.as_any().downcast_ref::<MapBox>().is_some(),
    );
    insert_bool(
        &info,
        "is_array",
        value.as_any().downcast_ref::<ArrayBox>().is_some(),
    );
    insert_bool(
        &info,
        "is_null",
        value.as_any().downcast_ref::<NullBox>().is_some(),
    );
    info
}

fn insert_string(target: &MapBox, key: &str, value: &str) {
    let _ = target.set(
        Box::new(StringBox::new(key)),
        Box::new(StringBox::new(value)),
    );
}

fn insert_bool(target: &MapBox, key: &str, value: bool) {
    let _ = target.set(Box::new(StringBox::new(key)), Box::new(BoolBox::new(value)));
}

fn classify_kind(value: &dyn NyashBox) -> String {
    if value.as_any().downcast_ref::<MapBox>().is_some() {
        return "MapBox".to_string();
    }
    if value.as_any().downcast_ref::<ArrayBox>().is_some() {
        return "ArrayBox".to_string();
    }
    if value.as_any().downcast_ref::<StringBox>().is_some() {
        return "StringBox".to_string();
    }
    if value.as_any().downcast_ref::<IntegerBox>().is_some() {
        return "IntegerBox".to_string();
    }
    if value.as_any().downcast_ref::<BoolBox>().is_some() {
        return "BoolBox".to_string();
    }
    if value.as_any().downcast_ref::<NullBox>().is_some() {
        return "NullBox".to_string();
    }
    simplify_type_name(value.type_name())
}

fn simplify_type_name(full: &str) -> String {
    full.rsplit("::").next().unwrap_or(full).to_string()
}

fn type_id_hash(value: &dyn NyashBox) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.as_any().type_id().hash(&mut hasher);
    hasher.finish()
}

/// Handle env.mirbuilder.* methods (Program(JSON v0) → MIR(JSON v0))
fn handle_mirbuilder(
    method_name: &str,
    args: &[Box<dyn NyashBox>],
) -> BidResult<Option<Box<dyn NyashBox>>> {
    match method_name {
        "emit" => {
            if crate::config::env::mirbuilder_delegate_forbidden() {
                crate::runtime::get_global_ring0().log.error(
                    &crate::config::env::mirbuilder_delegate_forbidden_message(
                        "env.mirbuilder.emit",
                    ),
                );
                return Err(BidError::PluginError);
            }
            let program_json = args
                .get(0)
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            match crate::runtime::mirbuilder_emit::emit_program_json_to_mir_json_with_env_imports(
                &program_json,
            ) {
                Ok(s) => Ok(Some(Box::new(StringBox::new(&s)) as Box<dyn NyashBox>)),
                Err(_e) => Ok(None),
            }
        }
        _ => Err(BidError::PluginError),
    }
}

/// Handle env.future.await method
fn handle_future_await(args: &[Box<dyn NyashBox>]) -> BidResult<Option<Box<dyn NyashBox>>> {
    if let Some(arg) = args.get(0) {
        if let Some(fut) = arg
            .as_any()
            .downcast_ref::<crate::boxes::future::FutureBox>()
        {
            let max_ms: u64 = crate::config::env::await_max_ms();
            let start = std::time::Instant::now();
            let mut spins = 0usize;

            while !fut.ready() {
                global_hooks::safepoint_and_poll();
                std::thread::yield_now();
                spins += 1;

                if spins % 1024 == 0 {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }

                if start.elapsed() >= std::time::Duration::from_millis(max_ms) {
                    let err = StringBox::new("Timeout");
                    return Ok(Some(Box::new(NyashResultBox::new_err(Box::new(err)))));
                }
            }

            return match fut.wait_terminal() {
                crate::boxes::future::FutureTerminal::Ready(v) => {
                    Ok(Some(Box::new(NyashResultBox::new_ok(v))))
                }
                crate::boxes::future::FutureTerminal::Failed(error)
                | crate::boxes::future::FutureTerminal::Cancelled(error) => {
                    Ok(Some(Box::new(NyashResultBox::new_err(error))))
                }
            };
        } else {
            return Ok(Some(Box::new(NyashResultBox::new_ok(arg.clone_box()))));
        }
    }

    Ok(Some(Box::new(NyashResultBox::new_err(Box::new(
        StringBox::new("InvalidArgs"),
    )))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_log() {
        let args = vec![Box::new(StringBox::new("test")) as Box<dyn NyashBox>];
        let result = handle_console("log", &args);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_result_ok() {
        let args = vec![Box::new(StringBox::new("success")) as Box<dyn NyashBox>];
        let result = handle_result("ok", &args);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_result_err() {
        let args = vec![];
        let result = handle_result("err", &args);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_unknown_interface() {
        let args = vec![];
        let result = extern_call("unknown.interface", "method", &args);
        assert!(matches!(result, Err(BidError::PluginError)));
    }

    #[test]
    fn test_unknown_method() {
        let args = vec![];
        let result = extern_call("env.console", "unknown_method", &args);
        assert!(matches!(result, Err(BidError::PluginError)));
    }

    #[test]
    fn test_future_await_failed_returns_result_err() {
        let fut = crate::boxes::future::FutureBox::new();
        fut.set_failed(Box::new(crate::boxes::basic::ErrorBox::new(
            "TaskError",
            "boom",
        )));

        let args = vec![Box::new(fut) as Box<dyn NyashBox>];
        let out = handle_future_await(&args)
            .expect("future await bridge must succeed")
            .expect("future await bridge must return a result box");

        let result = out
            .as_any()
            .downcast_ref::<crate::boxes::result::NyashResultBox>()
            .expect("await bridge must return ResultBox");
        assert!(result.is_err());
        assert_eq!(result.get_error().to_string_box().value, "TaskError: boom");
    }

    #[test]
    fn test_future_await_cancelled_returns_result_err() {
        let fut = crate::boxes::future::FutureBox::new();
        fut.cancel_with_reason("scope-cancelled");

        let args = vec![Box::new(fut) as Box<dyn NyashBox>];
        let out = handle_future_await(&args)
            .expect("future await bridge must succeed")
            .expect("future await bridge must return a result box");

        let result = out
            .as_any()
            .downcast_ref::<crate::boxes::result::NyashResultBox>()
            .expect("await bridge must return ResultBox");
        assert!(result.is_err());
        assert_eq!(
            result.get_error().to_string_box().value,
            "Cancelled: scope-cancelled"
        );
    }

    #[test]
    fn test_future_await_sibling_failed_returns_result_err() {
        let group = crate::boxes::task_group_box::TaskGroupBox::new();
        let failed = crate::boxes::future::FutureBox::new();
        let sibling = crate::boxes::future::FutureBox::new();
        group.add_future(&failed);
        group.add_future(&sibling);
        failed.set_failed(Box::new(crate::boxes::basic::ErrorBox::new(
            "TaskError",
            "boom",
        )));

        let args = vec![Box::new(sibling) as Box<dyn NyashBox>];
        let out = handle_future_await(&args)
            .expect("future await bridge must succeed")
            .expect("future await bridge must return a result box");

        let result = out
            .as_any()
            .downcast_ref::<crate::boxes::result::NyashResultBox>()
            .expect("await bridge must return ResultBox");
        assert!(result.is_err());
        assert_eq!(
            result.get_error().to_string_box().value,
            "Cancelled: sibling-failed"
        );
    }
}
