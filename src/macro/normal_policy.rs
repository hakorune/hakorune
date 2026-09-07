//! Invocation configuration, never source authority. Capture outside the parser.
use super::default_derive::{self, DefaultDeriveSelectionV1};
use crate::ast::{ASTNode, BoxMethodInventoryV1};

#[derive(Debug, Clone)]
pub(crate) struct NormalMacroPolicyV1 {
    enabled: bool,
    derive_all: bool,
    derive_set: String,
}

impl NormalMacroPolicyV1 {
    pub(crate) fn capture() -> Self {
        Self {
            enabled: super::enabled(),
            derive_all: crate::config::env::macro_derive_all(),
            derive_set: crate::config::env::macro_derive()
                .unwrap_or_else(|| "Equals,ToString".into()),
        }
    }
    pub(crate) fn settings(&self) -> (bool, &str) { (self.derive_all, &self.derive_set) }
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
    pub(crate) fn selection(
        &self,
        is_static: bool,
        methods: &BoxMethodInventoryV1,
    ) -> DefaultDeriveSelectionV1 {
        default_derive::select(is_static, methods, self.derive_all, &self.derive_set)
    }
    pub(crate) fn would_generate(&self, ast: &ASTNode) -> bool {
        let ASTNode::Program { statements, .. } = ast else {
            return false;
        };
        self.enabled
            && statements.iter().any(|node| match node {
                ASTNode::BoxDeclaration {
                    methods, is_static, ..
                } => {
                    let selected = self.selection(*is_static, methods);
                    selected.equals || selected.to_string
                }
                _ => false,
            })
    }
}
