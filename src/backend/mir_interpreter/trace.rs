use super::*;

impl MirInterpreter {
    pub(super) fn box_trace_enabled() -> bool {
        std::env::var("NYASH_BOX_TRACE").ok().as_deref() == Some("1")
    }

    fn box_trace_filter_match(class_name: &str) -> bool {
        match std::env::var("NYASH_BOX_TRACE_FILTER").ok() {
            Some(pattern) if !pattern.is_empty() => class_name.contains(pattern.as_str()),
            _ => true,
        }
    }

    fn json_escape(s: &str) -> String {
        let mut out = String::new();
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out
    }

    pub(super) fn box_trace_emit_new(&self, class_name: &str, argc: usize) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"new\",\"class\":\"{}\",\"argc\":{}}}",
            Self::json_escape(class_name),
            argc
        ));
    }

    pub(super) fn box_trace_emit_call(&self, class_name: &str, method: &str, argc: usize) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"call\",\"class\":\"{}\",\"method\":\"{}\",\"argc\":{}}}",
            Self::json_escape(class_name),
            Self::json_escape(method),
            argc
        ));
    }

    pub(super) fn box_trace_emit_get(&self, class_name: &str, field: &str, val_kind: &str) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"get\",\"class\":\"{}\",\"field\":\"{}\",\"val\":\"{}\"}}",
            Self::json_escape(class_name),
            Self::json_escape(field),
            Self::json_escape(val_kind)
        ));
    }

    pub(super) fn box_trace_emit_set(&self, class_name: &str, field: &str, val_kind: &str) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"set\",\"class\":\"{}\",\"field\":\"{}\",\"val\":\"{}\"}}",
            Self::json_escape(class_name),
            Self::json_escape(field),
            Self::json_escape(val_kind)
        ));
    }

    pub(super) fn print_trace_enabled() -> bool {
        std::env::var("NYASH_PRINT_TRACE").ok().as_deref() == Some("1")
    }

    pub(super) fn print_trace_emit(&self, val: &VMValue) {
        if !Self::print_trace_enabled() {
            return;
        }
        let (kind, class, nullish) = match val {
            VMValue::Integer(_) => ("Integer", "".to_string(), None),
            VMValue::ExactNumeric(value) => ("ExactNumeric", value.source_name.clone(), None),
            VMValue::Float(_) => ("Float", "".to_string(), None),
            VMValue::Bool(_) => ("Bool", "".to_string(), None),
            VMValue::String(_) => ("String", "".to_string(), None),
            VMValue::Void => ("Void", "".to_string(), None),
            VMValue::Future(_) => ("Future", "".to_string(), None),
            VMValue::BoxRef(b) => {
                // Prefer InstanceBox.class_name when available
                if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
                    let tag = if crate::config::env::null_missing_box_enabled() {
                        if b.as_any()
                            .downcast_ref::<crate::boxes::null_box::NullBox>()
                            .is_some()
                        {
                            Some("null")
                        } else if b
                            .as_any()
                            .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                            .is_some()
                        {
                            Some("missing")
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    ("BoxRef", inst.class_name.clone(), tag)
                } else {
                    let tag = if crate::config::env::null_missing_box_enabled() {
                        if b.as_any()
                            .downcast_ref::<crate::boxes::null_box::NullBox>()
                            .is_some()
                        {
                            Some("null")
                        } else if b
                            .as_any()
                            .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                            .is_some()
                        {
                            Some("missing")
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    ("BoxRef", b.type_name().to_string(), tag)
                }
            }
            VMValue::WeakBox(_) => ("WeakRef", "".to_string(), None), // Phase 285A0
        };
        if let Some(tag) = nullish {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "{{\"ev\":\"print\",\"kind\":\"{}\",\"class\":\"{}\",\"nullish\":\"{}\"}}",
                kind,
                Self::json_escape(&class),
                tag
            ));
        } else {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "{{\"ev\":\"print\",\"kind\":\"{}\",\"class\":\"{}\"}}",
                kind,
                Self::json_escape(&class)
            ));
        }
    }
}
