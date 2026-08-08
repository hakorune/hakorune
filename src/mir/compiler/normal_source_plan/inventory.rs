use crate::ast::ASTNode;

use super::product::{NormalTopLevelSiteV1, PreparedNormalSourcePlanInputV1};
use super::rejection::{
    NormalSourcePlanErrorV1, NormalUnsupportedTopLevelKindV1, RejectedNormalSourcePlanV1,
};

#[derive(Debug)]
pub(super) struct NormalMethodSurfaceV1 {
    pub(super) method_key: Box<str>,
    pub(super) declaration_name: Option<Box<str>>,
    pub(super) arity: Option<usize>,
    pub(super) is_static: Option<bool>,
}

#[derive(Debug)]
pub(super) struct NormalMainBoxSurfaceV1 {
    pub(super) site: NormalTopLevelSiteV1,
    pub(super) is_static: bool,
    pub(super) methods: Box<[NormalMethodSurfaceV1]>,
}

#[derive(Debug)]
pub(super) struct NormalUnsupportedTopLevelSiteV1 {
    pub(super) statement_index: usize,
    pub(super) kind: NormalUnsupportedTopLevelKindV1,
}

#[derive(Debug)]
pub(in crate::mir) struct NormalSourceSurfaceInventoryV1 {
    pub(super) input: PreparedNormalSourcePlanInputV1,
    pub(super) script_sites: Box<[NormalTopLevelSiteV1]>,
    pub(super) top_level_callables: Box<[NormalTopLevelSiteV1]>,
    pub(super) main_boxes: Box<[NormalMainBoxSurfaceV1]>,
    pub(super) non_main_box_sites: Box<[NormalTopLevelSiteV1]>,
    pub(super) unsupported: Box<[NormalUnsupportedTopLevelSiteV1]>,
}

impl NormalSourceSurfaceInventoryV1 {
    pub(super) fn collect(
        input: PreparedNormalSourcePlanInputV1,
    ) -> Result<Self, RejectedNormalSourcePlanV1> {
        let ASTNode::Program { statements, .. } = input.source() else {
            return Err(RejectedNormalSourcePlanV1::new(
                input,
                NormalSourcePlanErrorV1::RootNotProgram,
            ));
        };

        let mut script_sites = Vec::new();
        let mut top_level_callables = Vec::new();
        let mut main_boxes = Vec::new();
        let mut non_main_box_sites = Vec::new();
        let mut unsupported = Vec::new();

        for (statement_index, statement) in statements.iter().enumerate() {
            match statement {
                ASTNode::FunctionDeclaration { .. } => {
                    top_level_callables.push(NormalTopLevelSiteV1::new(statement_index));
                }
                ASTNode::BoxDeclaration {
                    name,
                    methods,
                    is_static,
                    ..
                } if name == "Main" => {
                    let methods = methods
                        .iter_compat_name_order()
                        .map(|entry| match entry.declaration() {
                            ASTNode::FunctionDeclaration {
                                name,
                                params,
                                is_static,
                                ..
                            } => NormalMethodSurfaceV1 {
                                method_key: entry.name().into(),
                                declaration_name: Some(name.as_str().into()),
                                arity: Some(params.len()),
                                is_static: Some(*is_static),
                            },
                            _ => NormalMethodSurfaceV1 {
                                method_key: entry.name().into(),
                                declaration_name: None,
                                arity: None,
                                is_static: None,
                            },
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice();
                    main_boxes.push(NormalMainBoxSurfaceV1 {
                        site: NormalTopLevelSiteV1::new(statement_index),
                        is_static: *is_static,
                        methods,
                    });
                }
                ASTNode::Program { .. } => unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                    statement_index,
                    kind: NormalUnsupportedTopLevelKindV1::NestedProgram,
                }),
                ASTNode::UsingStatement { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::Using,
                    })
                }
                ASTNode::ImportStatement { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::Import,
                    })
                }
                ASTNode::BuildGate { .. } => unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                    statement_index,
                    kind: NormalUnsupportedTopLevelKindV1::BuildGate,
                }),
                ASTNode::BoxDeclaration { .. } => {
                    non_main_box_sites.push(NormalTopLevelSiteV1::new(statement_index));
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::Box,
                    })
                }
                ASTNode::EnumDeclaration { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::Enum,
                    })
                }
                ASTNode::BrandDeclaration { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::Brand,
                    })
                }
                ASTNode::TypeAliasDeclaration { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::TypeAlias,
                    })
                }
                ASTNode::GlobalVar { .. } => unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                    statement_index,
                    kind: NormalUnsupportedTopLevelKindV1::Global,
                }),
                ASTNode::StaticConstTable { .. } => {
                    unsupported.push(NormalUnsupportedTopLevelSiteV1 {
                        statement_index,
                        kind: NormalUnsupportedTopLevelKindV1::StaticConstTable,
                    })
                }
                _ => script_sites.push(NormalTopLevelSiteV1::new(statement_index)),
            }
        }

        Ok(Self {
            input,
            script_sites: script_sites.into_boxed_slice(),
            top_level_callables: top_level_callables.into_boxed_slice(),
            main_boxes: main_boxes.into_boxed_slice(),
            non_main_box_sites: non_main_box_sites.into_boxed_slice(),
            unsupported: unsupported.into_boxed_slice(),
        })
    }
}
