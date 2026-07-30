//! One source-only constructor batch shared by instance-Box lifecycles.
//!
//! Field registration and Box declaration lowering stay with each outer
//! lifecycle. This owner only projects the constructor map into deterministic,
//! owned inputs for the existing instance-method child terminal.

use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceBatchV1;
use super::recursive_child_lowering::RawBoxMethodChildPortV1;
use super::MirBuilder;

#[derive(Clone, Debug)]
pub(super) struct PreparedInstanceBoxConstructorBatchV1 {
    owner: String,
    constructors: Box<[PreparedInstanceBoxConstructorV1]>,
}

#[derive(Clone, Debug)]
struct PreparedInstanceBoxConstructorV1 {
    parser_constructor_key: String,
    function_name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
}

impl PreparedInstanceBoxConstructorBatchV1 {
    pub(super) fn prepare(owner: &str, constructors: &HashMap<String, ASTNode>) -> Self {
        let mut entries: Vec<(&str, &ASTNode)> = constructors
            .iter()
            .map(|(key, constructor)| (key.as_str(), constructor))
            .collect();
        entries.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
        let constructors = entries
            .into_iter()
            .filter_map(|(constructor_key, constructor)| {
                let ASTNode::FunctionDeclaration {
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    ..
                } = constructor
                else {
                    return None;
                };
                Some(PreparedInstanceBoxConstructorV1 {
                    parser_constructor_key: constructor_key.to_owned(),
                    function_name: format!("{}.{}", owner, constructor_key),
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
            constructors,
        }
    }

    /// Issues one source occurrence for each parser-normalized constructor row.
    /// The returned batch may be transported to Script runtime work, where a
    /// second physical demand still materializes a fresh collector admission.
    pub(super) fn normal_sources(
        &self,
        statement_index: usize,
    ) -> NormalInstanceConstructorSourceBatchV1 {
        NormalInstanceConstructorSourceBatchV1::new(
            statement_index,
            &self.owner,
            self.constructors
                .iter()
                .map(|constructor| constructor.parser_constructor_key.clone())
                .collect(),
        )
    }

    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        for constructor in self.constructors {
            port.lower_instance_box_method(
                builder,
                constructor.function_name,
                self.owner.clone(),
                constructor.params,
                constructor.param_decls,
                constructor.return_type_name,
                constructor.body,
                constructor.uses,
                constructor.attrs,
            )?;
        }
        Ok(())
    }

    pub(super) fn lower_normal_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
        sources: &NormalInstanceConstructorSourceBatchV1,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        let Self {
            owner,
            constructors,
        } = self;
        if constructors.len() != sources.sources().len() {
            return Err("[freeze:contract][mir/instance-constructor-admission/count]".to_owned());
        }
        for (constructor, source_key) in constructors.into_vec().into_iter().zip(sources.sources())
        {
            if constructor.parser_constructor_key != source_key.parser_constructor_key()
                || owner != source_key.box_name()
            {
                return Err(
                    "[freeze:contract][mir/instance-constructor-admission/source-drift]".to_owned(),
                );
            }
            port.lower_normal_instance_constructor(
                builder,
                source_key,
                constructor.params,
                constructor.param_decls,
                constructor.return_type_name,
                constructor.body,
                constructor.uses,
                constructor.attrs,
            )?;
        }
        Ok(())
    }
}
