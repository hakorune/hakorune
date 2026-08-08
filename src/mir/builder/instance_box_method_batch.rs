//! One source-only ordinary-method batch shared by instance-Box lifecycles.
//!
//! The Program-root and raw lifecycles keep distinct terminal authority:
//! Program lowering resolves each exact callable-catalog key, while raw
//! lowering directly uses the existing instance-method child terminal.

use crate::ast::{ASTNode, BoxMethodInventoryV1, DeclarationAttrs, ParamDecl};

use super::declaration_order::sorted_method_entries;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::nested_box_method_source::NestedBoxMethodLoweringInputV1;
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::recursive_child_lowering::RawBoxMethodChildPortV1;
use super::{MirBuilder, SameModuleCallableNamespaceV1};

pub(super) struct PreparedInstanceBoxMethodBatchV1 {
    owner: String,
    methods: Box<[PreparedInstanceBoxMethodV1]>,
}

struct PreparedInstanceBoxMethodV1 {
    method_name: String,
    function_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl PreparedInstanceBoxMethodBatchV1 {
    pub(super) fn prepare(owner: &str, methods: &BoxMethodInventoryV1) -> Self {
        let methods = sorted_method_entries(methods)
            .into_iter()
            .filter_map(|(method_name, method)| {
                let ASTNode::FunctionDeclaration {
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    is_static,
                    uses,
                    attrs,
                    ..
                } = method
                else {
                    return None;
                };
                if *is_static {
                    return None;
                }
                Some(PreparedInstanceBoxMethodV1 {
                    method_name: method_name.to_owned(),
                    function_name: format!("{}.{}/{}", owner, method_name, params.len()),
                    params: params.clone(),
                    param_decls: param_decls.clone(),
                    return_type_name: return_type_name.clone(),
                    body: body.clone(),
                    uses: uses.clone(),
                    attrs: attrs.clone(),
                })
            })
            .collect();
        Self {
            owner: owner.to_owned(),
            methods,
        }
    }

    pub(super) fn lower_raw_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        for method in self.methods {
            port.lower_nested_box_method(
                builder,
                NestedBoxMethodLoweringInputV1::instance_method(
                    method.method_name,
                    method.function_name,
                    self.owner.clone(),
                    method.params,
                    method.param_decls,
                    method.return_type_name,
                    method.body,
                    method.uses,
                    method.attrs,
                ),
            )?;
        }
        Ok(())
    }

    pub(super) fn lower_root_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        for method in self.methods {
            let canonical_key = builder
                .comp_ctx
                .callable_declaration_catalog()
                .map_err(|error| error.to_string())?
                .declaration_for(
                    SameModuleCallableNamespaceV1::InstanceBoxMethod,
                    &self.owner,
                    &method.method_name,
                    method.params.len(),
                )
                .ok_or_else(|| {
                    format!(
                        "[freeze:contract][mir/instance-capture/catalog] \
                         missing exact declaration for {}.{}/{}",
                        self.owner,
                        method.method_name,
                        method.params.len()
                    )
                })?
                .key()
                .clone();
            let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(canonical_key)
                .map_err(|error| error.to_string())?;
            port.lower_cataloged_instance_box_method(
                builder,
                admission,
                method.params,
                method.param_decls,
                method.return_type_name,
                method.body,
                method.uses,
                method.attrs,
            )?;
        }
        Ok(())
    }
}
