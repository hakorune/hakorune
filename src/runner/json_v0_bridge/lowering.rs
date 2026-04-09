use super::ast::{EnumDeclV0, ProgramV0, StmtV0, UserBoxDeclV0};
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
};
// Phase 25.1: BTreeMap → BTreeMap（決定性確保）
use std::collections::BTreeMap;

// Split out merge/new_block helpers for readability (no behavior change)
mod merge;
use merge::{merge_var_maps, new_block};
// Feature splits (gradual extraction)
pub(super) mod dump;
pub(super) mod expr;
pub(super) mod globals;
pub(super) mod if_else;
pub(super) mod if_legacy;
pub(super) mod lambda_legacy;
pub(super) mod loop_;
pub(super) mod loop_runtime;
pub(super) mod match_expr; // placeholder (not wired)
pub(super) mod program;
pub(super) mod scope_exit;
pub(super) mod stmts;
pub(super) mod ternary; // placeholder (not wired)
pub(super) mod throw_ctx;
pub(super) mod throw_lower;
pub(super) mod try_catch; // thread-local ctx for Result-mode throw routing
pub(super) mod while_legacy;

pub(super) fn normalize_scope_exit_registrations(stmts: &[StmtV0]) -> Result<Vec<StmtV0>, String> {
    scope_exit::normalize_scope_exit_registrations(stmts)
}

#[derive(Clone, Copy)]
pub(super) struct LoopContext {
    /// ループ条件を評価する header ブロック
    pub(super) cond_bb: BasicBlockId,
    /// break がジャンプする exit ブロック
    pub(super) exit_bb: BasicBlockId,
    /// canonical continue merge ブロック（存在する場合）
    /// - Some(continue_merge_bb): continue は一度ここに集約してから header へ戻る
    /// - None: 旧来どおり header へ直接 continue
    pub(super) continue_merge_bb: Option<BasicBlockId>,
}

#[derive(Clone)]
pub(super) struct BridgeEnv {
    pub(super) throw_enabled: bool,
    // フェーズM.2: mir_no_phiフィールド削除（PHI統一で不要）
    pub(super) allow_me_dummy: bool,
    pub(super) me_class: String,
    pub(super) try_result_mode: bool,
    // Phase 21.8: using imports map (alias -> box_type)
    pub(super) imports: BTreeMap<String, String>,
    /// Static-box method call resolution table (JSON v0 bridge).
    /// Key format: "{BoxName}.{method}/{arity}" (e.g. "RewriteKnownMini.run/0")
    pub(super) static_methods: BTreeMap<String, ()>,
    /// Declared enum inventory for canonical sum lowering.
    pub(super) enum_decls: BTreeMap<String, EnumDeclV0>,
    /// User-defined box declarations needed for source-route NewBox/FieldGet lowering.
    pub(super) user_box_decls: BTreeMap<String, UserBoxDeclV0>,
}

impl BridgeEnv {
    #[allow(dead_code)]
    pub(super) fn load() -> Self {
        Self::with_imports(BTreeMap::new())
    }

    pub(super) fn with_imports(imports: BTreeMap<String, String>) -> Self {
        let trm = crate::config::env::try_result_mode();
        // フェーズM.2: no_phi変数削除
        if crate::config::env::cli_verbose() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[Bridge] load: try_result_mode={}", trm));
        }
        Self {
            throw_enabled: std::env::var("NYASH_BRIDGE_THROW_ENABLE").ok().as_deref() == Some("1"),
            // フェーズM.2: mir_no_phiフィールド削除
            allow_me_dummy: std::env::var("NYASH_BRIDGE_ME_DUMMY").ok().as_deref() == Some("1"),
            me_class: std::env::var("NYASH_BRIDGE_ME_CLASS").unwrap_or_else(|_| "Main".to_string()),
            try_result_mode: trm,
            imports,
            static_methods: BTreeMap::new(),
            enum_decls: BTreeMap::new(),
            user_box_decls: BTreeMap::new(),
        }
    }
}

/// Phase 25.1p: FunctionDefBuilder — 関数定義から MIR 関数への変換を箱化
/// SSOT for JSON v0 function signatures/var-map initialization
pub(super) struct FunctionDefBuilder {
    def: super::ast::FuncDefV0,
}

impl FunctionDefBuilder {
    pub(super) fn new(def: super::ast::FuncDefV0) -> Self {
        Self { def }
    }

    /// 変数マップの初期化（params を SSOT としてそのまま使う）
    ///
    /// Note: `MirFunction::new()` already reserves parameter ValueIds (0..N-1) based on
    /// `signature.params.len()`. JSON v0 bridge must keep that convention to avoid ValueId
    /// collisions (SSA violations) when emitting Const/Copy instructions.
    pub(super) fn build_var_map(&self, param_ids: &[ValueId]) -> BTreeMap<String, ValueId> {
        let mut map = BTreeMap::new();

        for (i, param_name) in self.def.params.iter().enumerate() {
            if let Some(&vid) = param_ids.get(i) {
                map.insert(param_name.clone(), vid);
            }
        }

        map
    }

    /// 関数シグネチャの構築
    pub(super) fn build_signature(&self) -> FunctionSignature {
        let func_name = format!(
            "{}.{}/{}",
            self.def.box_name,
            self.def.name,
            self.def.params.len()
        );

        let param_types: Vec<MirType> = (0..self.def.params.len())
            .map(|_| MirType::Unknown)
            .collect();

        FunctionSignature {
            name: func_name,
            params: param_types,
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        }
    }

    // `next_value_id` setup is handled by `MirFunction::new()`.
}

/// Strip Phi instructions by inserting edge copies on each predecessor.
/// This normalizes MIR to PHI-off form for downstream harnesses that synthesize PHIs.
// フェーズM.2: strip_phi_functions()削除 - PHI統一により不要

pub(super) fn lower_stmt_with_vars(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    s: &StmtV0,
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    stmts::lower_stmt_with_vars(f, cur_bb, s, vars, loop_stack, env)
}

pub(super) fn lower_stmt_list_with_vars(
    f: &mut MirFunction,
    start_bb: BasicBlockId,
    stmts: &[StmtV0],
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    stmts::lower_stmt_list_with_vars(f, start_bb, stmts, vars, loop_stack, env)
}

pub(super) fn lower_program(
    prog: ProgramV0,
    imports: std::collections::BTreeMap<String, String>,
) -> Result<MirModule, String> {
    if prog.body.is_empty() {
        return Err("empty body".into());
    }
    let mut env = BridgeEnv::with_imports(imports);
    env.user_box_decls = prog
        .user_box_decls
        .iter()
        .map(|decl| (decl.name.clone(), decl.clone()))
        .collect();
    env.enum_decls = prog
        .enum_decls
        .iter()
        .map(|decl| (decl.name.clone(), decl.clone()))
        .collect();
    // Precompute static-box method table from defs, so Expr lowering can resolve `BoxName.method()`
    // even when `BoxName` isn't a runtime variable in JSON v0.
    for def in &prog.defs {
        let q = format!(
            "{}.{}{}",
            def.box_name,
            def.name,
            format!("/{}", def.params.len())
        );
        env.static_methods.insert(q, ());
    }
    let mut module = MirModule::new("ny_json_v0".into());
    module.metadata.user_box_decls = env
        .user_box_decls
        .iter()
        .map(|(name, decl)| (name.clone(), decl.fields.clone()))
        .collect();
    module.metadata.user_box_field_decls = env
        .user_box_decls
        .iter()
        .map(|(name, decl)| {
            (
                name.clone(),
                decl.field_decls
                    .iter()
                    .map(|field| crate::mir::UserBoxFieldDecl {
                        name: field.name.clone(),
                        declared_type_name: field.declared_type.clone(),
                        is_weak: field.is_weak,
                    })
                    .collect(),
            )
        })
        .collect();
    module.metadata.enum_decls = env
        .enum_decls
        .iter()
        .map(|(name, decl)| {
            (
                name.clone(),
                crate::mir::MirEnumDecl {
                    type_parameters: decl.type_parameters.clone(),
                    variants: decl
                        .variants
                        .iter()
                        .map(|variant| crate::mir::MirEnumVariantDecl {
                            name: variant.name.clone(),
                            payload_type_name: variant.payload_type.clone(),
                        })
                        .collect(),
                },
            )
        })
        .collect();
    program::lower_main_body(&mut module, &prog.body, &env)?;
    if let Some(entry_def) = prog
        .defs
        .iter()
        .find(|def| program::is_stageb_entry_def(def))
    {
        if let Some(main_fn) = module.functions.get_mut("main") {
            main_fn.metadata.runes = program::rune_attrs_from_json_v0(&entry_def.attrs);
        }
    }
    let func_map = program::lower_defs_into_module(&mut module, prog.defs, &env)?;
    program::maybe_resolve_calls(&mut module, &func_map);

    Ok(module)
}

pub(super) fn maybe_dump_mir(module: &MirModule) {
    dump::maybe_dump_mir(module);
}
