/*!
 * CompilationContext - Compilation state management for MirBuilder
 *
 * Phase 136 follow-up (Step 7/7): Extract compilation-related fields from MirBuilder
 * to consolidate box compilation state, type information, and analysis metadata.
 *
 * Consolidates:
 * - compilation_context: Box compilation context (BoxCompilationContext)
 * - current_static_box: Current static box being compiled
 * - user_defined_boxes: User-defined box names registry
 * - reserved_value_ids: Reserved ValueIds for PHI instructions
 * - fn_body_ast: Function body AST for capture analysis
 * - weak_fields_by_box: Weak field registry
 * - property_getters_by_box: Property getter registry
 * - field_origin_class: Field origin tracking
 * - field_origin_by_box: Class-level field origin
 * - static_method_index: Static method index
 * - method_tail_index: Method tail index
 * - method_tail_index_source_len: Source length snapshot
 * - type_registry: Type registry box
 * - current_slot_registry: Function scope slot registry
 * - plugin_method_sigs: Plugin method signatures
 */

use crate::ast::ASTNode;
use crate::ast::FieldDecl;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{MirType, ValueId};
use std::collections::{HashMap, HashSet};

use super::type_registry::TypeRegistry;
use super::PropertyKind;
use hakorune_mir_builder::BoxCompilationContext;

/// Compilation state context for MIR builder
///
/// Consolidates all compilation-related state including box compilation context,
/// type information, analysis metadata, and method resolution indices.
#[derive(Debug)]
pub(crate) struct CompilationContext {
    /// Box compilation context (for static box compilation isolation)
    /// Some(ctx) during static box compilation, None for traditional mode
    pub compilation_context: Option<BoxCompilationContext>,

    /// Current static box name when lowering a static box body (e.g., "Main")
    pub current_static_box: Option<String>,

    /// Names of user-defined boxes declared in the current module
    /// Phase 285LLVM-1.1: Extended to track fields (box name → field names)
    /// For static boxes: empty Vec (no fields)
    /// For instance boxes: Vec of field names
    pub user_defined_boxes: HashMap<String, Vec<String>>,

    /// Typed field declarations keyed by user box name.
    pub user_box_field_decls: HashMap<String, Vec<FieldDecl>>,

    /// Phase 201-A: Reserved ValueIds that must not be allocated
    /// These are PHI dst ValueIds created by LoopHeaderPhiBuilder.
    /// When next_value_id() encounters a reserved ID, it skips to the next.
    /// Cleared after JoinIR merge completes.
    pub reserved_value_ids: HashSet<ValueId>,

    /// Phase 200-C: Original function body AST for capture analysis
    /// Stored temporarily during function lowering to support FunctionScopeCaptureAnalyzer.
    /// None when not lowering a function, or when fn_body is not available.
    pub fn_body_ast: Option<Vec<ASTNode>>,

    /// Weak field registry: BoxName -> {weak field names}
    pub weak_fields_by_box: HashMap<String, HashSet<String>>,

    /// Unified members: BoxName -> {propName -> Kind}
    pub property_getters_by_box: HashMap<String, HashMap<String, PropertyKind>>,

    /// Remember class of object fields after assignments: (base_id, field) -> class_name
    pub field_origin_class: HashMap<(ValueId, String), String>,

    /// Class-level field origin (cross-function heuristic): (BaseBoxName, field) -> FieldBoxName
    pub field_origin_by_box: HashMap<(String, String), String>,

    /// Index of static methods seen during lowering: name -> [(BoxName, arity)]
    pub static_method_index: HashMap<String, Vec<(String, usize)>>,

    /// Explicit imported static-box bindings: alias -> concrete static box name.
    ///
    /// This is Layer 3 of the alias split:
    /// - Layer 1: manifest alias/module ownership in hako.toml
    /// - Layer 2: runner strip/text-merge binds imported aliases
    /// - Layer 3: builder consumes that table so `Alias.method(...)` lowers as
    ///   a static call even after `using` lines were stripped from source.
    pub using_import_boxes: HashMap<String, String>,

    /// Fast lookup: method+arity tail → candidate function names (e.g., ".str/0" → ["JsonNode.str/0", ...])
    pub method_tail_index: HashMap<String, Vec<String>>,

    /// Source size snapshot to detect when to rebuild the tail index
    pub method_tail_index_source_len: usize,

    /// 🎯 箱理論: 型情報管理の一元化（TypeRegistryBox）
    /// NYASH_USE_TYPE_REGISTRY=1 で有効化（段階的移行用）
    pub type_registry: TypeRegistry,

    /// 関数スコープの SlotRegistry（観測専用）
    /// - current_function と同じライフサイクルを持つよ。
    /// - 既存の variable_map/SSA には影響しない（メタデータのみ）。
    pub current_slot_registry: Option<FunctionSlotRegistry>,

    /// Plugin method return type signatures loaded from nyash_box.toml
    pub plugin_method_sigs: HashMap<(String, String), MirType>,

    /// Phase 288: REPL mode での内部ログ抑制フラグ
    /// REPL mode でのみ true、file mode では常に false
    pub quiet_internal_logs: bool,
}

#[allow(dead_code)]
impl CompilationContext {
    /// Create a new CompilationContext with default-initialized state
    pub fn new() -> Self {
        Self {
            compilation_context: None,
            current_static_box: None,
            user_defined_boxes: HashMap::new(), // Phase 285LLVM-1.1: HashMap for fields
            user_box_field_decls: HashMap::new(),
            reserved_value_ids: HashSet::new(),
            fn_body_ast: None,
            weak_fields_by_box: HashMap::new(),
            property_getters_by_box: HashMap::new(),
            field_origin_class: HashMap::new(),
            field_origin_by_box: HashMap::new(),
            static_method_index: HashMap::new(),
            using_import_boxes: HashMap::new(),
            method_tail_index: HashMap::new(),
            method_tail_index_source_len: 0,
            type_registry: TypeRegistry::new(),
            current_slot_registry: None,
            plugin_method_sigs: HashMap::new(),
            quiet_internal_logs: false, // File mode: 常に false
        }
    }

    /// Create a new CompilationContext with plugin method signatures
    pub fn with_plugin_sigs(plugin_method_sigs: HashMap<(String, String), MirType>) -> Self {
        Self {
            plugin_method_sigs,
            ..Self::new()
        }
    }

    /// Check if a box is user-defined
    pub fn is_user_defined_box(&self, name: &str) -> bool {
        self.user_defined_boxes.contains_key(name) // Phase 285LLVM-1.1: HashMap check
    }

    /// Register a user-defined box (backward compatibility - no fields)
    pub fn register_user_box(&mut self, name: String) {
        self.user_defined_boxes.insert(name.clone(), Vec::new()); // Phase 285LLVM-1.1: Empty fields
        self.user_box_field_decls.insert(name, Vec::new());
    }

    /// Phase 285LLVM-1.1: Register a user-defined box with field information
    pub fn register_user_box_with_fields(&mut self, name: String, fields: Vec<String>) {
        self.user_defined_boxes.insert(name.clone(), fields);
        self.user_box_field_decls.insert(name, Vec::new());
    }

    pub fn register_user_box_with_field_decls(
        &mut self,
        name: String,
        field_decls: Vec<FieldDecl>,
    ) {
        let fields = field_decls.iter().map(|decl| decl.name.clone()).collect();
        self.user_defined_boxes.insert(name.clone(), fields);
        self.user_box_field_decls.insert(name, field_decls);
    }

    pub fn declared_field_type_name(&self, box_name: &str, field_name: &str) -> Option<&str> {
        self.user_box_field_decls
            .get(box_name)
            .and_then(|decls| decls.iter().find(|decl| decl.name == field_name))
            .and_then(|decl| decl.declared_type_name.as_deref())
    }

    /// Check if a ValueId is reserved
    pub fn is_reserved_value_id(&self, id: ValueId) -> bool {
        self.reserved_value_ids.contains(&id)
    }

    /// Reserve a ValueId (for PHI instructions)
    pub fn reserve_value_id(&mut self, id: ValueId) {
        self.reserved_value_ids.insert(id);
    }

    /// Clear all reserved ValueIds (after JoinIR merge)
    pub fn clear_reserved_value_ids(&mut self) {
        self.reserved_value_ids.clear();
    }

    /// Enter static box compilation mode
    pub fn enter_static_box(&mut self, name: String) {
        self.current_static_box = Some(name);
    }

    /// Exit static box compilation mode
    pub fn exit_static_box(&mut self) {
        self.current_static_box = None;
    }

    /// Get current static box name
    pub fn current_static_box(&self) -> Option<&str> {
        self.current_static_box.as_deref()
    }

    /// Check if currently compiling a static box
    pub fn is_in_static_box(&self) -> bool {
        self.current_static_box.is_some()
    }

    /// Store function body AST for capture analysis
    pub fn set_fn_body_ast(&mut self, ast: Vec<ASTNode>) {
        self.fn_body_ast = Some(ast);
    }

    /// Take function body AST (consumes it)
    pub fn take_fn_body_ast(&mut self) -> Option<Vec<ASTNode>> {
        self.fn_body_ast.take()
    }

    /// Clear function body AST
    pub fn clear_fn_body_ast(&mut self) {
        self.fn_body_ast = None;
    }

    /// Check if a field is weak for a box
    pub fn is_weak_field(&self, box_name: &str, field_name: &str) -> bool {
        self.weak_fields_by_box
            .get(box_name)
            .map_or(false, |fields| fields.contains(field_name))
    }

    /// Register a weak field for a box
    pub fn register_weak_field(&mut self, box_name: String, field_name: String) {
        self.weak_fields_by_box
            .entry(box_name)
            .or_insert_with(HashSet::new)
            .insert(field_name);
    }

    /// Get property kind for a box member
    pub fn get_property_kind(&self, box_name: &str, prop_name: &str) -> Option<&PropertyKind> {
        self.property_getters_by_box
            .get(box_name)
            .and_then(|props| props.get(prop_name))
    }

    /// Register a property getter for a box
    pub fn register_property_getter(
        &mut self,
        box_name: String,
        prop_name: String,
        kind: PropertyKind,
    ) {
        self.property_getters_by_box
            .entry(box_name)
            .or_insert_with(HashMap::new)
            .insert(prop_name, kind);
    }

    /// Get field origin class for a value's field
    pub fn get_field_origin_class(&self, base_id: ValueId, field: &str) -> Option<&str> {
        self.field_origin_class
            .get(&(base_id, field.to_string()))
            .map(|s| s.as_str())
    }

    /// Set field origin class for a value's field
    pub fn set_field_origin_class(&mut self, base_id: ValueId, field: String, class: String) {
        self.field_origin_class.insert((base_id, field), class);
    }

    /// Get field origin by box (class-level)
    pub fn get_field_origin_by_box(&self, base_box: &str, field: &str) -> Option<&str> {
        self.field_origin_by_box
            .get(&(base_box.to_string(), field.to_string()))
            .map(|s| s.as_str())
    }

    /// Set field origin by box (class-level)
    pub fn set_field_origin_by_box(&mut self, base_box: String, field: String, origin: String) {
        self.field_origin_by_box.insert((base_box, field), origin);
    }

    /// Register a static method
    pub fn register_static_method(&mut self, method_name: String, box_name: String, arity: usize) {
        self.static_method_index
            .entry(method_name)
            .or_insert_with(Vec::new)
            .push((box_name, arity));
    }

    /// Get static method candidates
    pub fn get_static_method_candidates(&self, method_name: &str) -> Option<&[(String, usize)]> {
        self.static_method_index
            .get(method_name)
            .map(|v| v.as_slice())
    }

    /// Replace imported static-box alias bindings for the next compilation.
    pub fn set_using_import_boxes(&mut self, imports: HashMap<String, String>) {
        self.using_import_boxes = imports;
    }

    /// Clear imported static-box alias bindings.
    pub fn clear_using_import_boxes(&mut self) {
        self.using_import_boxes.clear();
    }

    /// Resolve an imported static-box alias to a concrete box name.
    pub fn resolve_imported_static_box(&self, alias: &str) -> Option<&str> {
        self.using_import_boxes.get(alias).map(|name| name.as_str())
    }

    /// Get method tail index candidates
    pub fn get_method_tail_candidates(&self, tail: &str) -> Option<&[String]> {
        self.method_tail_index.get(tail).map(|v| v.as_slice())
    }

    /// Rebuild method tail index if needed
    pub fn maybe_rebuild_method_tail_index(&mut self, current_source_len: usize) -> bool {
        if self.method_tail_index_source_len != current_source_len {
            self.method_tail_index_source_len = current_source_len;
            true
        } else {
            false
        }
    }

    /// Add method tail index entry
    pub fn add_method_tail_entry(&mut self, tail: String, full_name: String) {
        self.method_tail_index
            .entry(tail)
            .or_insert_with(Vec::new)
            .push(full_name);
    }

    /// Clear method tail index
    pub fn clear_method_tail_index(&mut self) {
        self.method_tail_index.clear();
    }

    /// Get plugin method signature
    pub fn get_plugin_method_sig(&self, box_name: &str, method_name: &str) -> Option<&MirType> {
        self.plugin_method_sigs
            .get(&(box_name.to_string(), method_name.to_string()))
    }

    /// Set current slot registry
    pub fn set_slot_registry(&mut self, registry: FunctionSlotRegistry) {
        self.current_slot_registry = Some(registry);
    }

    /// Take current slot registry (consumes it)
    pub fn take_slot_registry(&mut self) -> Option<FunctionSlotRegistry> {
        self.current_slot_registry.take()
    }

    /// Clear current slot registry
    pub fn clear_slot_registry(&mut self) {
        self.current_slot_registry = None;
    }
}

impl Default for CompilationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
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

        ctx.register_property_getter(
            "MyBox".to_string(),
            "computed".to_string(),
            PropertyKind::Computed,
        );

        assert_eq!(
            ctx.get_property_kind("MyBox", "computed"),
            Some(&PropertyKind::Computed)
        );
        assert_eq!(ctx.get_property_kind("MyBox", "other"), None);
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
}
