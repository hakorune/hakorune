use crate::ast::{
    ASTNode, BoxMemberGateSiteV1, BoxMethodInventoryV1, ContractClause, DeclarationAttrs,
    DelegateDecl, FieldDecl, ParamDecl, TransitionDecl,
};
use crate::parser::source_authority::{OpenBoxMethodSourceTransactionV1, ParserInvocationBrandV1};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub(crate) struct BoxMemberState {
    pub(crate) fields: Vec<String>,
    pub(crate) field_decls: Vec<FieldDecl>,
    pub(crate) field_initializers: Vec<(String, ASTNode)>,
    pub(crate) source_tx: OpenBoxMethodSourceTransactionV1,
    pub(crate) public_fields: Vec<String>,
    pub(crate) private_fields: Vec<String>,
    pub(crate) constructors: HashMap<String, ASTNode>,
    pub(crate) init_fields: Vec<String>,
    pub(crate) weak_fields: Vec<String>,
    pub(crate) delegates: Vec<DelegateDecl>,
    pub(crate) invariants: Vec<ASTNode>,
    pub(crate) transitions: Vec<TransitionDecl>,
    pub(crate) birth_once_props: Vec<String>,
}

impl Default for BoxMemberState {
    fn default() -> Self {
        Self::with_source_transaction(OpenBoxMethodSourceTransactionV1::open(
            ParserInvocationBrandV1::issue(),
            0,
        ))
    }
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
    pub(crate) fn with_source_transaction(source_tx: OpenBoxMethodSourceTransactionV1) -> Self {
        Self {
            source_tx,
            fields: Vec::new(),
            field_decls: Vec::new(),
            field_initializers: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            birth_once_props: Vec::new(),
        }
    }

    pub(crate) fn methods(&self) -> &BoxMethodInventoryV1 {
        self.source_tx.inventory()
    }

    pub(crate) fn current_gate_site(&self) -> crate::ast::BoxMemberGateSiteV1 {
        self.source_tx.current_gate_site()
    }

    pub(crate) fn current_source_member_ordinal(&self) -> u32 {
        self.source_tx.current_member_ordinal()
    }

    pub(crate) fn finish_source_member(&mut self) -> Result<(), crate::parser::ParseError> {
        self.source_tx
            .finish_member()
            .map_err(|error| crate::parser::ParseError::BuildCfg {
                message: format!("Box member source transaction failed: {error:?}"),
                line: 0,
            })
    }

    pub(crate) fn try_merge_selected_gate(
        &mut self,
        mut other: BoxMemberState,
        gate_site: BoxMemberGateSiteV1,
    ) -> Result<(), crate::parser::ParseError> {
        for delegate in &mut other.delegates {
            delegate.prepend_selected_gate(gate_site).map_err(|error| {
                crate::parser::ParseError::BuildCfg {
                    message: format!(
                        "selected delegate source provenance cannot be sealed: {error:?}"
                    ),
                    line: 0,
                }
            })?;
        }
        self.source_tx
            .try_merge_selected_gate(other.source_tx, gate_site)?;
        self.fields.extend(other.fields.drain(..));
        self.field_decls.extend(other.field_decls.drain(..));
        self.field_initializers
            .extend(other.field_initializers.drain(..));
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
        Ok(())
    }

    pub(crate) fn signature(&self) -> BoxMemberSignature {
        let mut methods = BTreeMap::new();
        for entry in self.methods().iter_selected_declaration_order() {
            let name = entry.name();
            let node = entry.declaration();
            if let Some(sig) = MethodSignature::from_node(node) {
                methods.insert(name.to_owned(), sig);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DeclarationAttrs, Span};

    fn function(name: &str, line: usize) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.to_owned(),
            params: vec![],
            param_decls: vec![],
            return_type_name: None,
            body: vec![],
            uses: vec![],
            contracts: vec![],
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::new(0, 0, line, 1),
        }
    }

    #[test]
    fn selected_collision_leaves_whole_destination_state_unchanged() {
        let mut destination = BoxMemberState::default();
        destination
            .source_tx
            .commit_explicit_at_current("run".to_owned(), function("run", 1), Span::new(0, 0, 1, 1))
            .unwrap();
        destination.fields.push("before".to_owned());
        let methods_before = destination.methods().clone();
        let fields_before = destination.fields.clone();
        let birth_once_before = destination.birth_once_props.clone();

        let mut selected = BoxMemberState::default();
        selected
            .source_tx
            .commit_explicit_at_current("run".to_owned(), function("run", 7), Span::new(0, 0, 7, 1))
            .unwrap();
        selected.fields.push("must_not_publish".to_owned());
        selected
            .birth_once_props
            .push("must_not_publish".to_owned());

        assert!(destination
            .try_merge_selected_gate(selected, BoxMemberGateSiteV1::from_box_member_ordinal(3),)
            .is_err());
        assert_eq!(destination.methods(), &methods_before);
        assert_eq!(destination.fields, fields_before);
        assert_eq!(destination.birth_once_props, birth_once_before);
    }
}
