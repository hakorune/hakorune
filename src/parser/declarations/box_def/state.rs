use crate::ast::{
    ASTNode, ContractClause, DeclarationAttrs, DelegateDecl, FieldDecl, ParamDecl, TransitionDecl,
};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default)]
pub(crate) struct BoxMemberState {
    pub(crate) fields: Vec<String>,
    pub(crate) field_decls: Vec<FieldDecl>,
    pub(crate) field_initializers: Vec<(String, ASTNode)>,
    pub(crate) methods: HashMap<String, ASTNode>,
    pub(crate) public_fields: Vec<String>,
    pub(crate) private_fields: Vec<String>,
    pub(crate) constructors: HashMap<String, ASTNode>,
    pub(crate) init_fields: Vec<String>,
    pub(crate) weak_fields: Vec<String>,
    pub(crate) delegates: Vec<DelegateDecl>,
    pub(crate) invariants: Vec<ASTNode>,
    pub(crate) transitions: Vec<TransitionDecl>,
    pub(crate) birth_once_props: Vec<String>,
    pub(crate) last_method_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct MethodSignature {
    name: String,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    uses: Vec<String>,
    contracts: Vec<ContractClause>,
    is_static: bool,
    is_override: bool,
    attrs: DeclarationAttrs,
}

impl MethodSignature {
    fn from_node(node: &ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = node
        else {
            return None;
        };
        Some(Self {
            name: name.clone(),
            params: params.clone(),
            param_decls: param_decls.clone(),
            return_type_name: return_type_name.clone(),
            uses: uses.clone(),
            contracts: contracts.clone(),
            is_static: *is_static,
            is_override: *is_override,
            attrs: attrs.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoxMemberSignature {
    fields: Vec<String>,
    field_decls: Vec<FieldDecl>,
    field_initializers: Vec<(String, ASTNode)>,
    public_fields: Vec<String>,
    private_fields: Vec<String>,
    methods: BTreeMap<String, MethodSignature>,
    constructors: BTreeMap<String, MethodSignature>,
    init_fields: Vec<String>,
    weak_fields: Vec<String>,
    delegates: Vec<DelegateDecl>,
    invariants: Vec<ASTNode>,
    transitions: Vec<TransitionDecl>,
    birth_once_props: Vec<String>,
}

impl BoxMemberSignature {
    pub(crate) fn is_empty(&self) -> bool {
        self.fields.is_empty()
            && self.field_decls.is_empty()
            && self.field_initializers.is_empty()
            && self.public_fields.is_empty()
            && self.private_fields.is_empty()
            && self.methods.is_empty()
            && self.constructors.is_empty()
            && self.init_fields.is_empty()
            && self.weak_fields.is_empty()
            && self.delegates.is_empty()
            && self.invariants.is_empty()
            && self.transitions.is_empty()
            && self.birth_once_props.is_empty()
    }
}

impl BoxMemberState {
    pub(crate) fn merge_from(&mut self, mut other: BoxMemberState) {
        self.fields.extend(other.fields.drain(..));
        self.field_decls.extend(other.field_decls.drain(..));
        self.field_initializers
            .extend(other.field_initializers.drain(..));
        self.methods.extend(other.methods.drain());
        self.public_fields.extend(other.public_fields.drain(..));
        self.private_fields.extend(other.private_fields.drain(..));
        self.constructors.extend(other.constructors.drain());
        self.init_fields.extend(other.init_fields.drain(..));
        self.weak_fields.extend(other.weak_fields.drain(..));
        self.delegates.extend(other.delegates.drain(..));
        self.invariants.extend(other.invariants.drain(..));
        self.transitions.extend(other.transitions.drain(..));
        self.birth_once_props
            .extend(other.birth_once_props.drain(..));
    }

    pub(crate) fn signature(&self) -> BoxMemberSignature {
        let mut methods = BTreeMap::new();
        for (name, node) in &self.methods {
            if let Some(sig) = MethodSignature::from_node(node) {
                methods.insert(name.clone(), sig);
            }
        }

        let mut constructors = BTreeMap::new();
        for (name, node) in &self.constructors {
            if let Some(sig) = MethodSignature::from_node(node) {
                constructors.insert(name.clone(), sig);
            }
        }

        BoxMemberSignature {
            fields: self.fields.clone(),
            field_decls: self.field_decls.clone(),
            field_initializers: self.field_initializers.clone(),
            public_fields: self.public_fields.clone(),
            private_fields: self.private_fields.clone(),
            methods,
            constructors,
            init_fields: self.init_fields.clone(),
            weak_fields: self.weak_fields.clone(),
            delegates: self.delegates.clone(),
            invariants: self.invariants.clone(),
            transitions: self.transitions.clone(),
            birth_once_props: self.birth_once_props.clone(),
        }
    }
}
