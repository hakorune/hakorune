//! BORROW-P0-ROOT-P0c: atomic declaration-fact publication into one shell.
//!
//! Preparation consumes the function-empty shell and the already sealed four
//! declaration lanes. It checks the complete destination surface before any
//! mutation. Commit then moves every lane together and cannot fail.

use super::ModuleLoweringShellV1;
use crate::mir::builder::module_declaration_facts::SealedModuleDeclarationFactsV1;
use crate::mir::function::{MirEnumDecl, RecordDecl};
use crate::mir::UserBoxFieldDecl;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleDeclarationFactShellPrepareErrorV1 {
    DestinationNotEmpty {
        user_box_decls: usize,
        user_box_field_decls: usize,
        record_decls: usize,
        enum_decls: usize,
    },
}

impl std::fmt::Display for ModuleDeclarationFactShellPrepareErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][module_shell/declaration_facts] {self:?}"
        )
    }
}

impl std::error::Error for ModuleDeclarationFactShellPrepareErrorV1 {}

/// A failed preparation returns both owners unchanged. No caller needs to
/// reconstruct facts from Builder or CompilationContext state.
#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedModuleDeclarationFactShellCommitV1 {
    shell: ModuleLoweringShellV1,
    facts: SealedModuleDeclarationFactsV1,
    error: ModuleDeclarationFactShellPrepareErrorV1,
    _seal: RejectedModuleDeclarationFactShellCommitSealV1,
}

#[derive(Debug)]
struct RejectedModuleDeclarationFactShellCommitSealV1;

impl RejectedModuleDeclarationFactShellCommitV1 {
    pub(in crate::mir::builder) fn error(&self) -> &ModuleDeclarationFactShellPrepareErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ModuleLoweringShellV1,
        SealedModuleDeclarationFactsV1,
        ModuleDeclarationFactShellPrepareErrorV1,
    ) {
        (self.shell, self.facts, self.error)
    }
}

#[derive(Debug)]
struct PreparedModuleDeclarationMetadataV1 {
    user_box_decls: HashMap<String, Vec<String>>,
    user_box_field_decls: HashMap<String, Vec<UserBoxFieldDecl>>,
    record_decls: BTreeMap<String, RecordDecl>,
    enum_decls: BTreeMap<String, MirEnumDecl>,
}

/// Non-Clone one-shot publication owner. All checks and representation
/// conversion are complete before this product exists.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedModuleDeclarationFactShellCommitV1 {
    shell: ModuleLoweringShellV1,
    metadata: PreparedModuleDeclarationMetadataV1,
    _seal: PreparedModuleDeclarationFactShellCommitSealV1,
}

#[derive(Debug)]
struct PreparedModuleDeclarationFactShellCommitSealV1;

impl ModuleLoweringShellV1 {
    /// Builder-private variant used only after the aggregate environment
    /// installer has checked every destination lane. It cannot fail or
    /// perform a lookup.
    pub(in crate::mir::builder) fn prepare_declaration_fact_commit_preflighted(
        self,
        facts: SealedModuleDeclarationFactsV1,
    ) -> PreparedModuleDeclarationFactShellCommitV1 {
        debug_assert!(self.module.metadata.user_box_decls.is_empty());
        debug_assert!(self.module.metadata.user_box_field_decls.is_empty());
        debug_assert!(self.module.metadata.record_decls.is_empty());
        debug_assert!(self.module.metadata.enum_decls.is_empty());
        let (user_box_decls, user_box_field_decls, record_decls, enum_decls) = facts.into_parts();
        PreparedModuleDeclarationFactShellCommitV1 {
            shell: self,
            metadata: PreparedModuleDeclarationMetadataV1 {
                user_box_decls: user_box_decls.into_iter().collect(),
                user_box_field_decls: user_box_field_decls.into_iter().collect(),
                record_decls,
                enum_decls,
            },
            _seal: PreparedModuleDeclarationFactShellCommitSealV1,
        }
    }

    pub(in crate::mir::builder) fn prepare_declaration_fact_commit(
        self,
        facts: SealedModuleDeclarationFactsV1,
    ) -> Result<
        PreparedModuleDeclarationFactShellCommitV1,
        RejectedModuleDeclarationFactShellCommitV1,
    > {
        let metadata = &self.module.metadata;
        let counts = (
            metadata.user_box_decls.len(),
            metadata.user_box_field_decls.len(),
            metadata.record_decls.len(),
            metadata.enum_decls.len(),
        );
        if counts != (0, 0, 0, 0) {
            return Err(RejectedModuleDeclarationFactShellCommitV1 {
                shell: self,
                facts,
                error: ModuleDeclarationFactShellPrepareErrorV1::DestinationNotEmpty {
                    user_box_decls: counts.0,
                    user_box_field_decls: counts.1,
                    record_decls: counts.2,
                    enum_decls: counts.3,
                },
                _seal: RejectedModuleDeclarationFactShellCommitSealV1,
            });
        }

        let (user_box_decls, user_box_field_decls, record_decls, enum_decls) = facts.into_parts();
        Ok(PreparedModuleDeclarationFactShellCommitV1 {
            shell: self,
            metadata: PreparedModuleDeclarationMetadataV1 {
                user_box_decls: user_box_decls.into_iter().collect(),
                user_box_field_decls: user_box_field_decls.into_iter().collect(),
                record_decls,
                enum_decls,
            },
            _seal: PreparedModuleDeclarationFactShellCommitSealV1,
        })
    }
}

impl PreparedModuleDeclarationFactShellCommitV1 {
    pub(in crate::mir::builder) fn commit(mut self) -> ModuleLoweringShellV1 {
        let target = &mut self.shell.module.metadata;
        target.user_box_decls = self.metadata.user_box_decls;
        target.user_box_field_decls = self.metadata.user_box_field_decls;
        target.record_decls = self.metadata.record_decls;
        target.enum_decls = self.metadata.enum_decls;
        self.shell
    }

    pub(in crate::mir::builder) fn commit_with_source_file(
        mut self,
        source_file: Option<Box<str>>,
    ) -> ModuleLoweringShellV1 {
        self.shell.module.metadata.source_file = source_file.map(Into::into);
        self.commit()
    }
}
