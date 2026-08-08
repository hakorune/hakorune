//! One-shot source partition for explicit raw static-Main compatibility.
//!
//! This owner retains no AST after preparation. It preserves the historical
//! compatibility order: sorted non-main helpers lower first; only then does
//! the prepared root disposition yield the legacy missing/non-function error.

use super::decls::CallableMainCompatibilityLoweringErrorV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::{declaration_order::sorted_method_entries, MirBuilder, ValueId};
use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};

#[derive(Debug)]
pub(super) struct PreparedRawStaticMainBoxCompatibilityV1 {
    box_name: String,
    helpers: Vec<PreparedRawStaticMainHelperV1>,
    root: RawStaticMainRootDispositionV1,
}

#[derive(Debug)]
struct PreparedRawStaticMainHelperV1 {
    symbol: String,
    parts: OwnedRawStaticMainFunctionPartsV1,
}

#[derive(Debug)]
enum RawStaticMainRootDispositionV1 {
    Missing,
    NotFunction,
    Function(OwnedRawStaticMainFunctionPartsV1),
}

#[derive(Debug)]
struct OwnedRawStaticMainFunctionPartsV1 {
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl OwnedRawStaticMainFunctionPartsV1 {
    fn from_source(source: &ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            ..
        } = source
        else {
            return None;
        };
        Some(Self {
            params: params.clone(),
            param_decls: param_decls.clone(),
            return_type_name: return_type_name.clone(),
            body: body.clone(),
            uses: uses.clone(),
            attrs: attrs.clone(),
        })
    }

    fn into_parts(
        self,
    ) -> (
        Vec<String>,
        Vec<ParamDecl>,
        Option<String>,
        Vec<ASTNode>,
        Vec<String>,
        DeclarationAttrs,
    ) {
        (
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}

impl PreparedRawStaticMainBoxCompatibilityV1 {
    pub(super) fn prepare(box_name: String, methods: BoxMethodInventoryV1) -> Self {
        let mut helpers = Vec::new();
        let mut root = RawStaticMainRootDispositionV1::Missing;

        for (method_name, source) in sorted_method_entries(&methods) {
            if method_name == "main" {
                root = match OwnedRawStaticMainFunctionPartsV1::from_source(source) {
                    Some(parts) => RawStaticMainRootDispositionV1::Function(parts),
                    None => RawStaticMainRootDispositionV1::NotFunction,
                };
                continue;
            }
            let Some(parts) = OwnedRawStaticMainFunctionPartsV1::from_source(source) else {
                continue;
            };
            helpers.push(PreparedRawStaticMainHelperV1 {
                symbol: crate::mir::naming::encode_static_method(
                    &box_name,
                    method_name,
                    parts.params.len(),
                ),
                parts,
            });
        }

        Self {
            box_name,
            helpers,
            root,
        }
    }

    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1>
    where
        Port: RootCallableCapturePortV1,
    {
        let Self {
            box_name,
            helpers,
            root,
        } = self;
        for helper in helpers {
            let (params, param_decls, return_type_name, body, uses, attrs) =
                helper.parts.into_parts();
            port.lower_static_box_method(
                builder,
                helper.symbol,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )?;
        }

        match root {
            RawStaticMainRootDispositionV1::Missing => {
                Err("static box must contain a main() method".to_string().into())
            }
            RawStaticMainRootDispositionV1::NotFunction => {
                Err(CallableMainCompatibilityLoweringErrorV1::Lowering(
                    "main method in static box is not a FunctionDeclaration".to_string(),
                ))
            }
            RawStaticMainRootDispositionV1::Function(parts) => {
                let (params, param_decls, return_type_name, body, uses, attrs) = parts.into_parts();
                builder.lower_static_main_function_parts_with_port_v1(
                    port,
                    &box_name,
                    None,
                    None,
                    true,
                    super::decls::StaticMainScriptArgsSourceV1::LegacyEnvironment,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                )
            }
        }
    }
}
