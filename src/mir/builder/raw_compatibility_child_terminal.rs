//! RawCompatibility child terminal for the phase2160 root lane.
//!
//! This capability only connects an already-owned compatibility source shape to
//! the existing capture/collector seams.  It does not resolve a target, inspect
//! an AST for a symbol, or enter the SelectedNormal adapter.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::entry_materialization::RawEntryMaterializationSourceReceiptV1;
use super::main_expansion::VerifiedMainStaticChildV1;
use super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use super::MirBuilder;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawCompatibilityCallableShapeV1 {
    symbol: Box<str>,
    physical_arity: usize,
}

impl RawCompatibilityCallableShapeV1 {
    pub(in crate::mir::builder) fn issue(
        symbol: impl Into<Box<str>>,
        physical_arity: usize,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            physical_arity,
        }
    }

    pub(in crate::mir::builder) fn from_main_child(child: &VerifiedMainStaticChildV1<'_>) -> Self {
        Self::issue(child.symbol(), child.arity())
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn physical_arity(&self) -> usize {
        self.physical_arity
    }

    fn take(self) -> (String, usize) {
        (self.symbol.into_string(), self.physical_arity)
    }
}

pub(in crate::mir::builder) trait RawCompatibilityChildTerminalPortV1 {
    fn lower_raw_compat_static_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String>;

    fn lower_raw_compat_instance_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String>;

    fn lower_raw_compat_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String>;

    fn lower_raw_compat_main_materialization(
        &mut self,
        builder: &mut MirBuilder,
        receipt: RawEntryMaterializationSourceReceiptV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String>;
}

impl RawCompatibilityChildTerminalPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn lower_raw_compat_static_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        require_script_root_source(self)?;
        let (symbol, arity) = shape.take();
        if arity != params.len() {
            return Err(format!(
                "[freeze:contract][raw-compat-child/shape] symbol={symbol} shape_arity={arity} source_arity={}",
                params.len()
            ));
        }
        let pending = self
            .capture_static_box_method_pending_v1(
                builder,
                symbol.clone(),
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
            .map_err(|error: ModuleLoweringPortChildErrorV1| error.to_string())?;
        self.commit_legacy_nested_box_method_symbol_pending_v1(pending, symbol, arity)
            .map_err(|error| error.to_string())
    }

    fn lower_raw_compat_instance_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        require_script_root_source(self)?;
        let (symbol, arity) = shape.take();
        if arity != params.len() + 1 {
            return Err(format!(
                "[freeze:contract][raw-compat-child/shape] symbol={symbol} shape_arity={arity} source_arity={}",
                params.len() + 1
            ));
        }
        let pending = self
            .capture_normalized_instance_box_method_pending_v1(
                builder,
                symbol.clone(),
                box_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
            .map_err(|error: ModuleLoweringPortChildErrorV1| error.to_string())?;
        self.commit_legacy_nested_box_method_symbol_pending_v1(pending, symbol, arity)
            .map_err(|error| error.to_string())
    }

    fn lower_raw_compat_app_main_static_child(
        &mut self,
        builder: &mut MirBuilder,
        shape: RawCompatibilityCallableShapeV1,
        child: &VerifiedMainStaticChildV1<'_>,
    ) -> Result<(), String> {
        require_script_root_source(self)?;
        if shape.symbol() != child.symbol() || shape.physical_arity() != child.arity() {
            return Err(format!(
                "[freeze:contract][raw-compat-child/app-main-shape] shape={}/{} child={}/{}",
                shape.symbol(),
                shape.physical_arity(),
                child.symbol(),
                child.arity(),
            ));
        }
        let (symbol, params, param_decls, return_type_name, body, uses, attrs) =
            child.to_owned_lowering().into_parts();
        self.lower_raw_compat_static_child(
            builder,
            shape,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
        .map_err(|error| format!("{error} (child={symbol})"))
    }

    fn lower_raw_compat_main_materialization(
        &mut self,
        builder: &mut MirBuilder,
        receipt: RawEntryMaterializationSourceReceiptV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        require_script_root_source(self)?;
        if !receipt.policy().is_required() {
            return Ok(());
        }
        let target = receipt.target().ok_or_else(|| {
            "[freeze:contract][raw-compat-child/materialization-target-missing]".to_owned()
        })?;
        let shape = RawCompatibilityCallableShapeV1::issue(target.symbol(), target.arity());
        self.lower_raw_compat_static_child(
            builder,
            shape,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }
}

fn require_script_root_source(port: &RawInvocationChildPortV1<'_, '_>) -> Result<(), String> {
    validate_raw_compat_source_context_v1(port.current_source_context_v1().as_ref())
}

pub(in crate::mir::builder) fn validate_raw_compat_source_context_v1(
    context: Option<&RawInvocationSourceContextV1>,
) -> Result<(), String> {
    match context {
        Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            site: _,
            ..
        }) => Ok(()),
        Some(RawInvocationSourceContextV1::Located { root, .. }) => Err(format!(
            "[freeze:contract][raw-compat-child/source-lineage] unexpected root {root:?}"
        )),
        Some(RawInvocationSourceContextV1::UnlocatedCompatibility { .. }) => {
            Err("[freeze:contract][raw-compat-child/source-unlocated]".to_owned())
        }
        None => Err("[freeze:contract][raw-compat-child/source-missing]".to_owned()),
    }
}
