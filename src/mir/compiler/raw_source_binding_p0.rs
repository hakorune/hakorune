use super::raw_runtime_inputs::RawRuntimeInputCaptureErrorV1;
use super::raw_source_binding::{RawCallableMainSelectionV1, RawSourceBindingErrorV1};
use super::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use std::collections::HashMap;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvRestore {
    values: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

impl EnvRestore {
    fn capture(keys: &[&'static str]) -> Self {
        Self {
            values: keys
                .iter()
                .map(|key| (*key, std::env::var_os(key)))
                .collect(),
        }
    }
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        for (key, value) in &self.values {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

fn function(name: &str, arity: usize) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: (0..arity).map(|index| format!("p{index}")).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn script() -> ASTNode {
    ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    }
}

fn app() -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".into(), function("main", 0));
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
            is_static: true,
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

#[test]
fn raw_bind_mints_one_compiler_owned_raw_token_after_projection() {
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(script()),
            Some("script.hako"),
            "script",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap();
    assert_eq!(package.family(), ModuleInvocationFamilyV1::Raw);
    assert_eq!(
        package.continuation().callable_main(),
        super::super::builder::RawCallableMainCompatibilityDispositionV1::NotSelected
    );
    assert!(package.source().projection().is_script());
    assert_eq!(package.config().source_file(), Some("script.hako"));
    assert_eq!(package.module_name(), "script");
    assert!(package.source().ast().to_string().starts_with("Program"));
}

#[test]
fn raw_bind_selected_callable_main_requires_app_source() {
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(app()),
            None,
            "app",
            RawCallableMainSelectionV1::Required,
        )
        .unwrap();
    assert_eq!(package.family(), ModuleInvocationFamilyV1::Raw);
    assert_eq!(
        package.continuation().callable_main(),
        super::super::builder::RawCallableMainCompatibilityDispositionV1::Selected
    );
}

#[test]
fn raw_bind_retains_one_runtime_snapshot_on_the_source_package() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _restore = EnvRestore::capture(&[
        "NYASH_SCRIPT_ARGS_JSON",
        "HAKO_SCRIPT_ARGS_JSON",
        "NYASH_BUILDER_SAFEPOINT_ENTRY",
    ]);
    std::env::set_var("NYASH_SCRIPT_ARGS_JSON", r#"["alpha","beta"]"#);
    std::env::remove_var("HAKO_SCRIPT_ARGS_JSON");
    std::env::set_var("NYASH_BUILDER_SAFEPOINT_ENTRY", "true");

    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(script()),
            None,
            "runtime-snapshot",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap();
    let runtime = package.runtime_inputs();
    assert_eq!(
        runtime.script_args().values(),
        Some(&["alpha".to_string(), "beta".to_string()][..])
    );
    assert!(runtime.entry_safepoint().is_enabled());
}

#[test]
fn raw_bind_rejects_malformed_runtime_before_issuing_a_token() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _restore = EnvRestore::capture(&[
        "NYASH_SCRIPT_ARGS_JSON",
        "HAKO_SCRIPT_ARGS_JSON",
        "NYASH_BUILDER_SAFEPOINT_ENTRY",
    ]);
    std::env::set_var("NYASH_SCRIPT_ARGS_JSON", "not-json");
    std::env::remove_var("HAKO_SCRIPT_ARGS_JSON");
    std::env::remove_var("NYASH_BUILDER_SAFEPOINT_ENTRY");

    let mut compiler = MirCompiler::new();
    let rejected = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(script()),
            None,
            "malformed-runtime",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        RawSourceBindingErrorV1::RuntimeInputs(
            RawRuntimeInputCaptureErrorV1::MalformedScriptArgsJson { .. }
        )
    ));
    assert!(rejected.has_unpublished_source_owner());

    std::env::set_var("NYASH_SCRIPT_ARGS_JSON", "[]");
    let package = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(script()),
            None,
            "after-malformed-runtime",
            RawCallableMainSelectionV1::Omitted,
        )
        .unwrap();
    assert_eq!(package.brand().ordinal(), 1);
}

#[test]
fn raw_bind_rejects_required_callable_main_for_script() {
    let mut compiler = MirCompiler::new();
    let rejected = compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(script()),
            None,
            "script",
            RawCallableMainSelectionV1::Required,
        )
        .unwrap_err();
    assert!(matches!(
        rejected.error(),
        RawSourceBindingErrorV1::CallableMainRequiredForScript
    ));
}
