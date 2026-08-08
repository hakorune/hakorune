//! CHILDREN0 source-bound static helper work.
//!
//! This module validates one already projected locator by direct source
//! indexing.  It does not scan the AST, open a Builder session, or mutate any
//! physical owner.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::raw_source_projection::{OwnedRawSourceV1, RawSourceLocatorV1};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawRootStaticChildWorkErrorV1 {
    LocatorOutOfRange,
    NotStaticBox,
    BoxNameMismatch,
    MethodNameMismatch,
    MethodNotStatic,
    MethodOverride,
    ContractsPresent,
    ParameterMismatch,
    SymbolMismatch,
    ScheduleMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootStaticChildWorkV1 {
    ordinal: usize,
    locator: RawSourceLocatorV1,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawRootStaticChildLoweringPartsV1 {
    pub(super) function_name: String,
    pub(super) params: Vec<String>,
    pub(super) param_decls: Vec<ParamDecl>,
    pub(super) return_type_name: Option<String>,
    pub(super) body: Vec<ASTNode>,
    pub(super) uses: Vec<String>,
    pub(super) attrs: DeclarationAttrs,
}

impl RawRootStaticChildWorkV1 {
    pub(in crate::mir) fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub(in crate::mir) fn locator(&self) -> &RawSourceLocatorV1 {
        &self.locator
    }
    pub(in crate::mir) fn symbol(&self) -> &str {
        self.locator.symbol()
    }
    pub(in crate::mir) const fn arity(&self) -> usize {
        self.locator.arity()
    }

    pub(in crate::mir::builder) fn into_source_parts(
        self,
    ) -> (usize, RawSourceLocatorV1, RawRootStaticChildLoweringPartsV1) {
        let Self {
            ordinal,
            locator,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        } = self;
        let function_name = locator.symbol().to_owned();
        (
            ordinal,
            locator,
            RawRootStaticChildLoweringPartsV1 {
                function_name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            },
        )
    }

    pub(in crate::mir) fn into_callable_main(
        self,
    ) -> Result<RawCallableMainWorkV1, RawRootStaticChildWorkErrorV1> {
        if self.locator.box_name() != "Main" || self.locator.method_name() != "main" {
            return Err(RawRootStaticChildWorkErrorV1::MethodNameMismatch);
        }
        Ok(RawCallableMainWorkV1 { inner: self })
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawCallableMainWorkV1 {
    inner: RawRootStaticChildWorkV1,
}

impl RawCallableMainWorkV1 {
    pub(in crate::mir) fn symbol(&self) -> &str {
        self.inner.symbol()
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.inner.arity()
    }

    pub(in crate::mir::builder) fn into_inner(self) -> RawRootStaticChildWorkV1 {
        self.inner
    }
}

impl OwnedRawSourceV1 {
    pub(in crate::mir) fn prepare_static_child(
        &self,
        locator: RawSourceLocatorV1,
        ordinal: usize,
    ) -> Result<RawRootStaticChildWorkV1, RawRootStaticChildWorkErrorV1> {
        let ASTNode::Program { statements, .. } = self.ast() else {
            return Err(RawRootStaticChildWorkErrorV1::LocatorOutOfRange);
        };
        let Some(ASTNode::BoxDeclaration {
            name,
            methods,
            is_static,
            ..
        }) = statements.get(locator.top_level_statement())
        else {
            return Err(RawRootStaticChildWorkErrorV1::LocatorOutOfRange);
        };
        if !*is_static {
            return Err(RawRootStaticChildWorkErrorV1::NotStaticBox);
        }
        if name != locator.box_name() {
            return Err(RawRootStaticChildWorkErrorV1::BoxNameMismatch);
        }
        let Some(declaration) = methods.get_declaration(locator.method_name()) else {
            return Err(RawRootStaticChildWorkErrorV1::MethodNameMismatch);
        };
        let ASTNode::FunctionDeclaration {
            name: declared_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            is_static,
            is_override,
            contracts,
            ..
        } = declaration
        else {
            return Err(RawRootStaticChildWorkErrorV1::MethodNameMismatch);
        };
        if declared_name != locator.method_name() {
            return Err(RawRootStaticChildWorkErrorV1::MethodNameMismatch);
        }
        if !*is_static {
            return Err(RawRootStaticChildWorkErrorV1::MethodNotStatic);
        }
        if *is_override {
            return Err(RawRootStaticChildWorkErrorV1::MethodOverride);
        }
        if !contracts.is_empty() {
            return Err(RawRootStaticChildWorkErrorV1::ContractsPresent);
        }
        if params.len() != param_decls.len()
            || params
                .iter()
                .zip(param_decls)
                .any(|(param, decl)| param != &decl.name)
        {
            return Err(RawRootStaticChildWorkErrorV1::ParameterMismatch);
        }
        let symbol = crate::mir::naming::encode_static_method(name, declared_name, params.len());
        if symbol != locator.symbol() || params.len() != locator.arity() {
            return Err(RawRootStaticChildWorkErrorV1::SymbolMismatch);
        }
        Ok(RawRootStaticChildWorkV1 {
            ordinal,
            locator,
            params: params.clone(),
            param_decls: param_decls.clone(),
            return_type_name: return_type_name.clone(),
            body: body.clone(),
            uses: uses.clone(),
            attrs: attrs.clone(),
        })
    }
}
