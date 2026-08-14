//! Selected-normal source witness for one cataloged Box-method draft admission.
//!
//! This receipt is intentionally narrower than a resolved function owner.  It
//! preserves the catalog's source identity while deriving the existing physical
//! draft symbol and arity used by the normal collector compatibility boundary.

use super::calls::{LegacyFunctionPendingSessionV1, PendingFunctionSessionCloseV1};
use super::module_draft_collector::FunctionDraftKeyV1;
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use crate::ast::{DeclarationAttrs, ParamDecl};

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationV1;
use super::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalCatalogedBoxMethodAdmissionErrorV1 {
    PhysicalArityOverflow,
}

impl std::fmt::Display for NormalCatalogedBoxMethodAdmissionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][mir/normal-cataloged-box-method-admission] {self:?}"
        )
    }
}

impl std::error::Error for NormalCatalogedBoxMethodAdmissionErrorV1 {}

/// One catalog-backed source identity paired with the existing physical draft
/// contract.  It owns neither a body snapshot nor a collector borrow.
#[derive(Debug)]
pub(in crate::mir) struct NormalCatalogedBoxMethodDraftAdmissionV1 {
    source_key: CanonicalSameModuleCallableKeyV1,
    physical_symbol: Box<str>,
    physical_arity: usize,
    _seal: NormalCatalogedBoxMethodDraftAdmissionSealV1,
}

/// Owned physical header projection issued from the installed catalog row.
/// It carries storage-facing declaration data only; it is not a second
/// semantic declaration authority and is consumed by the selected A-prime
/// handoff.
#[derive(Debug)]
pub(in crate::mir) struct CatalogedBoxMethodPhysicalHeaderProjectionV1 {
    key: CanonicalSameModuleCallableKeyV1,
    params: Box<[String]>,
    param_decls: Box<[ParamDecl]>,
    return_type_name: Option<Box<str>>,
    uses: Box<[String]>,
    attrs: DeclarationAttrs,
}

impl CatalogedBoxMethodPhysicalHeaderProjectionV1 {
    pub(in crate::mir) fn from_catalog_declaration(
        declaration: &VerifiedSameModuleCallableDeclarationV1,
    ) -> Self {
        Self {
            key: declaration.key().clone(),
            params: declaration.params().to_vec().into_boxed_slice(),
            param_decls: declaration.param_decls().to_vec().into_boxed_slice(),
            return_type_name: declaration
                .return_type_name()
                .map(str::to_owned)
                .map(Into::into),
            uses: declaration.uses().to_vec().into_boxed_slice(),
            attrs: declaration.attrs().clone(),
        }
    }

    pub(in crate::mir) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }

    pub(in crate::mir) fn params(&self) -> &[String] {
        &self.params
    }

    pub(in crate::mir) fn param_decls(&self) -> &[ParamDecl] {
        &self.param_decls
    }

    pub(in crate::mir) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(in crate::mir) fn uses(&self) -> &[String] {
        &self.uses
    }

    pub(in crate::mir) fn attrs(&self) -> &DeclarationAttrs {
        &self.attrs
    }
}

#[derive(Debug)]
struct NormalCatalogedBoxMethodDraftAdmissionSealV1;

impl NormalCatalogedBoxMethodDraftAdmissionV1 {
    pub(in crate::mir) fn seal(
        source_key: CanonicalSameModuleCallableKeyV1,
    ) -> Result<Self, NormalCatalogedBoxMethodAdmissionErrorV1> {
        let source_arity = usize::try_from(source_key.arity())
            .map_err(|_| NormalCatalogedBoxMethodAdmissionErrorV1::PhysicalArityOverflow)?;
        let physical_arity = match source_key.namespace() {
            SameModuleCallableNamespaceV1::StaticBoxMethod => source_arity,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => source_arity
                .checked_add(1)
                .ok_or(NormalCatalogedBoxMethodAdmissionErrorV1::PhysicalArityOverflow)?,
        };
        let physical_symbol = source_key.mir_symbol_projection().into_boxed_str();

        Ok(Self {
            source_key,
            physical_symbol,
            physical_arity,
            _seal: NormalCatalogedBoxMethodDraftAdmissionSealV1,
        })
    }

    pub(in crate::mir) fn source_key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.source_key
    }

    pub(in crate::mir) fn physical_symbol(&self) -> &str {
        &self.physical_symbol
    }

    pub(in crate::mir) const fn physical_arity(&self) -> usize {
        self.physical_arity
    }

    pub(in crate::mir::builder) fn into_legacy_collector_parts(
        self,
    ) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            source_key: _,
            physical_symbol,
            physical_arity,
            _seal: _,
        } = self;
        let symbol = physical_symbol.into_string();
        (
            FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
            symbol,
            physical_arity,
        )
    }
}

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn commit_normal_cataloged_box_method_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.commit_legacy_symbol_pending(pending, admission.into_legacy_collector_parts())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ASTNode;
    use crate::mir::builder::callable_declaration_catalog::{
        SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    };
    use crate::parser::NyashParser;

    use super::*;

    fn catalog_key(
        source: &str,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        name: &str,
        arity: usize,
    ) -> CanonicalSameModuleCallableKeyV1 {
        let root = NyashParser::parse_from_string(source).expect("catalog source");
        let ASTNode::Program { .. } = root else {
            panic!("fixture must parse as Program");
        };
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root)
            .expect("catalog")
            .declaration_for(namespace, owner, name, arity)
            .expect("exact catalog row")
            .key()
            .clone()
    }

    #[test]
    fn seal_derives_static_and_instance_physical_contracts() {
        let static_admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(catalog_key(
            "static box Tools { add(left, right) { return left } }",
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Tools",
            "add",
            2,
        ))
        .expect("static receipt");
        assert_eq!(static_admission.physical_symbol(), "Tools.add/2");
        assert_eq!(static_admission.physical_arity(), 2);

        let instance_admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(catalog_key(
            "box Page { render(value) { return value } }",
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "Page",
            "render",
            1,
        ))
        .expect("instance receipt");
        assert_eq!(instance_admission.physical_symbol(), "Page.render/1");
        assert_eq!(instance_admission.physical_arity(), 2);
        assert_eq!(
            instance_admission.source_key().namespace(),
            SameModuleCallableNamespaceV1::InstanceBoxMethod
        );
    }
}
