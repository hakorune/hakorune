use super::*;

#[test]
fn test_compilation_context_creation() {
    let ctx = CompilationContext::new();
    assert!(ctx.current_static_box.is_none());
    assert!(ctx.user_defined_boxes.is_empty());
    assert!(ctx.reserved_value_ids.is_empty());
}

#[test]
fn test_user_defined_box() {
    let mut ctx = CompilationContext::new();
    assert!(!ctx.is_user_defined_box("MyBox"));

    ctx.register_user_box("MyBox".to_string());
    assert!(ctx.is_user_defined_box("MyBox"));
}

#[test]
fn test_reserved_value_ids() {
    let mut ctx = CompilationContext::new();
    let id = ValueId::new(42);

    assert!(!ctx.is_reserved_value_id(id));

    ctx.reserve_value_id(id);
    assert!(ctx.is_reserved_value_id(id));

    ctx.clear_reserved_value_ids();
    assert!(!ctx.is_reserved_value_id(id));
}

#[test]
fn test_static_box_mode() {
    let mut ctx = CompilationContext::new();
    assert!(!ctx.is_in_static_box());

    ctx.enter_static_box("Main".to_string());
    assert!(ctx.is_in_static_box());
    assert_eq!(ctx.current_static_box(), Some("Main"));

    ctx.exit_static_box();
    assert!(!ctx.is_in_static_box());
    assert_eq!(ctx.current_static_box(), None);
}

#[test]
fn test_weak_field_registry() {
    let mut ctx = CompilationContext::new();

    ctx.register_weak_field("MyBox".to_string(), "weakField".to_string());
    assert!(ctx.is_weak_field("MyBox", "weakField"));
    assert!(!ctx.is_weak_field("MyBox", "strongField"));
    assert!(!ctx.is_weak_field("OtherBox", "weakField"));
}

#[test]
fn test_property_getter_registry() {
    let mut ctx = CompilationContext::new();

    assert!(ctx.register_property_getter_method("MyBox".to_string(), "__get_computed"));
    assert!(ctx.register_property_getter_method("MyBox".to_string(), "__get_once_cached"));
    assert!(ctx.register_property_getter_method("MyBox".to_string(), "__get_birth_config"));
    assert!(!ctx.register_property_getter_method("MyBox".to_string(), "__compute_birth_config"));

    assert_eq!(
        ctx.property_getter_method_name("MyBox", "computed"),
        Some("__get_computed".to_string())
    );
    assert_eq!(
        ctx.property_getter_method_name("MyBox", "cached"),
        Some("__get_once_cached".to_string())
    );
    assert_eq!(
        ctx.property_getter_method_name("MyBox", "config"),
        Some("__get_birth_config".to_string())
    );
    assert_eq!(ctx.property_getter_method_name("MyBox", "other"), None);
}

#[test]
fn test_field_origin_tracking() {
    let mut ctx = CompilationContext::new();
    let base_id = ValueId::new(10);

    ctx.set_field_origin_class(base_id, "name".to_string(), "StringBox".to_string());
    assert_eq!(
        ctx.get_field_origin_class(base_id, "name"),
        Some("StringBox")
    );
    assert_eq!(ctx.get_field_origin_class(base_id, "other"), None);
}

#[test]
fn test_static_method_index() {
    let mut ctx = CompilationContext::new();

    ctx.register_static_method("parse".to_string(), "JsonBox".to_string(), 1);
    ctx.register_static_method("parse".to_string(), "XmlBox".to_string(), 1);

    let candidates = ctx.get_static_method_candidates("parse").unwrap();
    assert_eq!(candidates.len(), 2);
    assert!(candidates.contains(&("JsonBox".to_string(), 1)));
    assert!(candidates.contains(&("XmlBox".to_string(), 1)));
}

#[test]
fn test_method_tail_index() {
    let mut ctx = CompilationContext::new();

    ctx.add_method_tail_entry(".str/0".to_string(), "JsonNode.str/0".to_string());
    ctx.add_method_tail_entry(".str/0".to_string(), "XmlNode.str/0".to_string());

    let candidates = ctx.get_method_tail_candidates(".str/0").unwrap();
    assert_eq!(candidates.len(), 2);
    assert!(candidates.contains(&"JsonNode.str/0".to_string()));
    assert!(candidates.contains(&"XmlNode.str/0".to_string()));
}

#[test]
fn test_method_tail_index_rebuild() {
    let mut ctx = CompilationContext::new();

    assert!(ctx.maybe_rebuild_method_tail_index(100));
    assert!(!ctx.maybe_rebuild_method_tail_index(100));
    assert!(ctx.maybe_rebuild_method_tail_index(200));
}
