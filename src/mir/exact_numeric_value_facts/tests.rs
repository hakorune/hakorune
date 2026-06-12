use super::*;
use crate::mir::function::MirParamDecl;
#[allow(unused_imports)]
use crate::mir::{
    BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature, MirModule, UserBoxFieldDecl,
};

fn module_with_fields(function: MirFunction) -> MirModule {
    let mut module = MirModule::new("exact_numeric_value_facts_test".to_string());
    module.metadata.user_box_field_decls.insert(
        "Page".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "capacity".to_string(),
                declared_type_name: Some("usize".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "count".to_string(),
                declared_type_name: Some("u64".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "delta".to_string(),
                declared_type_name: Some("i64".to_string()),
                is_weak: false,
            },
        ],
    );
    module.add_function(function);
    module
}

fn page_function() -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn numeric_param_function() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "takes_size".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "size".to_string(),
        declared_type_name: Some("usize".to_string()),
    }];
    function.metadata.declared_return_type_name = Some("u64".to_string());
    function
}

#[cfg(test)]
#[path = "tests/acceptance.rs"]
mod acceptance;

#[cfg(test)]
#[path = "tests/rejections.rs"]
mod rejections;
