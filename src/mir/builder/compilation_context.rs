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
 * - property_registry: Property getter registry
 * - field_origin_class: Field origin tracking
 * - field_origin_by_box: Class-level field origin
 * - callable_declaration_catalog: Complete same-module callable declarations
 * - method_tail_index: Method tail index
 * - method_tail_index_source_len: Source length snapshot
 * - type_registry: Type registry box
 * - current_slot_registry: Function scope slot registry
 * - plugin_method_sigs: Plugin method signatures
 */

use crate::ast::FieldDecl;
use crate::ast::{ASTNode, EnumVariantDecl};
use crate::mir::function::{MirEnumDecl, MirEnumVariantDecl, RecordDecl};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{MirType, UserBoxFieldDecl, ValueId};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub(crate) struct RecordLocalFieldValue {
    pub name: String,
    pub declared_type_name: Option<String>,
    pub value: ValueId,
}

#[derive(Debug, Clone)]
pub(crate) struct RecordLocalValue {
    pub record_name: String,
    pub fields: Vec<RecordLocalFieldValue>,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumDeclLocal {
    pub type_parameters: Vec<String>,
    pub variants: Vec<EnumVariantDecl>,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedEnumVariant<'a> {
    pub tag: u32,
    pub decl: &'a EnumVariantDecl,
}

use super::callable_declaration_catalog::{
    SameModuleCallableDeclarationCatalogSessionErrorV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
};
use super::properties::PropertyRegistry;
use super::static_scalar_facts::{infer_static_scalar_method_fact, StaticScalarMethodFact};
use super::type_registry::TypeRegistry;
use hakorune_mir_builder::BoxCompilationContext;

mod declarations;

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

    /// Brand declarations visible to direct MIR lowering.
    ///
    /// Stage1 owns brand mismatch checking. The MIR builder only needs this
    /// declaration inventory so `BrandName(value)` can lower as a transparent
    /// single-value constructor instead of an unresolved function call.
    pub brand_decls: HashMap<String, String>,

    /// Typed field declarations keyed by user box name.
    pub user_box_field_decls: HashMap<String, Vec<FieldDecl>>,

    /// Record declarations keyed by record name. Records are not ordinary
    /// user boxes and must not enter `user_defined_boxes`.
    pub record_decls: HashMap<String, RecordDecl>,

    /// Record field defaults keyed by record name then field name.
    ///
    /// Defaults are builder-local source expressions used to complete record
    /// literals such as `ReportFields {}`. They are not exported as MIR record
    /// layout truth and must not imply runtime record materialization.
    pub record_field_defaults: HashMap<String, HashMap<String, ASTNode>>,

    /// Enum declarations visible to direct MIR lowering.
    ///
    /// The parser preloads `Option` / `Result` for source validation, but direct
    /// MIR lowering also needs a local inventory so variant constructors and
    /// guard-let enum probes lower to canonical sum instructions instead of
    /// falling through to helper calls.
    pub enum_decls: HashMap<String, EnumDeclLocal>,

    /// Complete immutable same-module callable declarations for this root.
    ///
    /// The slot is cleared at module preparation and installed exactly once
    /// before declaration-index effects. Missing or duplicate installation is
    /// a typed session error, never an empty-candidate fallback.
    callable_declaration_catalog: Option<VerifiedSameModuleCallableDeclarationCatalogV1>,

    /// Verified static scalar method facts keyed by lowered function name.
    ///
    /// This is not a generic purity registry. Entries are produced only by the
    /// narrow body-shape verifier, and row 296x-136 records facts without
    /// lowering calls to constants.
    pub static_scalar_method_facts: HashMap<String, StaticScalarMethodFact>,

    /// Weak field registry: BoxName -> {weak field names}
    pub weak_fields_by_box: HashMap<String, HashSet<String>>,

    /// Unified member property getter registry.
    property_registry: PropertyRegistry,

    /// Remember class of object fields after assignments: (base_id, field) -> class_name
    pub field_origin_class: HashMap<(ValueId, String), String>,

    /// Class-level field origin (cross-function heuristic): (BaseBoxName, field) -> FieldBoxName
    pub field_origin_by_box: HashMap<(String, String), String>,

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
        let enum_decls = prelude_enum_decls();
        Self {
            compilation_context: None,
            current_static_box: None,
            user_defined_boxes: HashMap::new(), // Phase 285LLVM-1.1: HashMap for fields
            brand_decls: HashMap::new(),
            user_box_field_decls: HashMap::new(),
            record_decls: HashMap::new(),
            record_field_defaults: HashMap::new(),
            enum_decls,
            callable_declaration_catalog: None,
            static_scalar_method_facts: HashMap::new(),
            weak_fields_by_box: HashMap::new(),
            property_registry: PropertyRegistry::new(),
            field_origin_class: HashMap::new(),
            field_origin_by_box: HashMap::new(),
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

    pub(crate) fn clear_callable_declaration_catalog(&mut self) {
        self.callable_declaration_catalog = None;
    }

    pub(crate) fn install_callable_declaration_catalog(
        &mut self,
        catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Result<(), SameModuleCallableDeclarationCatalogSessionErrorV1> {
        if self.callable_declaration_catalog.is_some() {
            return Err(SameModuleCallableDeclarationCatalogSessionErrorV1::DuplicateInstall);
        }
        self.callable_declaration_catalog = Some(catalog);
        Ok(())
    }

    pub(crate) fn callable_declaration_catalog(
        &self,
    ) -> Result<
        &VerifiedSameModuleCallableDeclarationCatalogV1,
        SameModuleCallableDeclarationCatalogSessionErrorV1,
    > {
        self.callable_declaration_catalog
            .as_ref()
            .ok_or(SameModuleCallableDeclarationCatalogSessionErrorV1::QueryBeforeInstall)
    }

    pub(crate) fn callable_declaration(
        &self,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        method: &str,
        arity: usize,
    ) -> Result<
        Option<&VerifiedSameModuleCallableDeclarationV1>,
        SameModuleCallableDeclarationCatalogSessionErrorV1,
    > {
        Ok(self
            .callable_declaration_catalog()?
            .declaration_for(namespace, owner, method, arity))
    }

    pub fn register_static_scalar_method_fact_if_verified(
        &mut self,
        func_name: &str,
        params: &[String],
        body: &[ASTNode],
    ) -> bool {
        let Some(fact) = infer_static_scalar_method_fact(func_name, params, body) else {
            self.static_scalar_method_facts.remove(func_name);
            return false;
        };
        self.static_scalar_method_facts
            .insert(func_name.to_string(), fact);
        true
    }

    pub fn static_scalar_method_fact(&self, func_name: &str) -> Option<&StaticScalarMethodFact> {
        self.static_scalar_method_facts.get(func_name)
    }

    pub fn static_scalar_method_fact_count(&self) -> usize {
        self.static_scalar_method_facts.len()
    }

    pub fn declared_field_type_name(&self, box_name: &str, field_name: &str) -> Option<&str> {
        self.user_box_field_decls
            .get(box_name)
            .and_then(|decls| decls.iter().find(|decl| decl.name == field_name))
            .and_then(|decl| decl.declared_type_name.as_deref())
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

    /// Get synthetic getter method name for a property read.
    pub fn property_getter_method_name(&self, box_name: &str, prop_name: &str) -> Option<String> {
        self.property_registry
            .getter_method_name(box_name, prop_name)
    }

    /// Register a synthetic property getter method if `method_name` uses a known getter prefix.
    pub fn register_property_getter_method(&mut self, box_name: String, method_name: &str) -> bool {
        self.property_registry
            .register_getter_method(box_name, method_name)
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
        self.method_tail_index_source_len = 0;
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

fn prelude_enum_decls() -> HashMap<String, EnumDeclLocal> {
    crate::semantics::result_option_prelude::result_option_prelude_enum_decls()
        .into_iter()
        .map(|(name, variants)| {
            let type_parameters = match name.as_str() {
                "Option" => vec!["T".to_string()],
                "Result" => vec!["T".to_string(), "E".to_string()],
                _ => Vec::new(),
            };
            (
                name,
                EnumDeclLocal {
                    type_parameters,
                    variants,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests;
