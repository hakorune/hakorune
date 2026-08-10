//! Parser-private Box-member source cursor shared by source-capable Box lanes.
//!
//! The cursor owns parser provenance, the exact Box declaration path, and the
//! next source-member ordinal. It owns no method inventory, parameter syntax,
//! resolver meaning, or semantic contract.

use super::source_authority::{
    ParserInvocationBrandV1, SourceBoxDeclarationSiteV1, SourceBoxMemberSiteV1,
};
use super::source_path::SourceBoxDeclarationPathV1;
use super::source_path::{SourceProgramCallablePathV1, SourceProgramDeclarationPathV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParserBoxMemberSourceCursorErrorV1 {
    MemberOrdinalOverflow,
}

#[derive(Debug)]
pub(super) struct ParserBoxMemberSourceCursorV1 {
    brand: ParserInvocationBrandV1,
    box_site: SourceBoxDeclarationSiteV1,
    next_member_ordinal: u32,
}

impl ParserBoxMemberSourceCursorV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1, statement_ordinal: u32) -> Self {
        Self::open_with_path(
            brand.clone(),
            SourceBoxDeclarationPathV1::root(brand, statement_ordinal),
        )
    }

    pub(super) fn open_with_path(
        brand: ParserInvocationBrandV1,
        path: SourceBoxDeclarationPathV1,
    ) -> Self {
        debug_assert!(path.brand().same_as(&brand));
        Self {
            brand,
            box_site: SourceBoxDeclarationSiteV1::from_path(path),
            next_member_ordinal: 0,
        }
    }

    pub(super) fn brand(&self) -> &ParserInvocationBrandV1 {
        &self.brand
    }

    pub(super) fn box_site(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.box_site
    }

    pub(super) fn current_member_site(&self) -> SourceBoxMemberSiteV1 {
        SourceBoxMemberSiteV1::new(self.box_site.clone(), self.next_member_ordinal)
    }

    pub(super) fn current_gate_site(&self) -> crate::ast::BoxMemberGateSiteV1 {
        crate::ast::BoxMemberGateSiteV1::from_box_member_ordinal(self.next_member_ordinal)
    }

    pub(super) fn current_member_ordinal(&self) -> u32 {
        self.next_member_ordinal
    }

    pub(super) fn current_program_callable_path(&self) -> SourceProgramCallablePathV1 {
        SourceProgramCallablePathV1::box_method(
            SourceProgramDeclarationPathV1::from_parser_path(self.box_site.path().clone()),
            Box::new([]),
            self.next_member_ordinal,
        )
    }

    pub(super) fn branch(&self) -> Self {
        Self {
            brand: self.brand.clone(),
            box_site: self.box_site.clone(),
            next_member_ordinal: 0,
        }
    }

    pub(super) fn finish_member(&mut self) -> Result<(), ParserBoxMemberSourceCursorErrorV1> {
        self.next_member_ordinal = self
            .next_member_ordinal
            .checked_add(1)
            .ok_or(ParserBoxMemberSourceCursorErrorV1::MemberOrdinalOverflow)?;
        Ok(())
    }

    pub(super) fn into_parts(self) -> (ParserInvocationBrandV1, SourceBoxDeclarationSiteV1) {
        (self.brand, self.box_site)
    }
}

#[cfg(test)]
#[path = "source_member_cursor_tests.rs"]
mod tests;
