//! Parser-private placement receipts for source-aware top-level projection.
//!
//! These rows remember where one exact parser source item landed after
//! BuildGate flattening.  They are placement only: callable identity remains
//! owned by the opaque declaration anchor.

use crate::parser::source_authority::{ParserInvocationBrandV1, SourceProgramDeclarationPathV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ProjectedProgramItemSlotV1 {
    source_path: SourceProgramDeclarationPathV1,
    final_statement_slot: u32,
}

impl ProjectedProgramItemSlotV1 {
    pub(in crate::parser) fn source_path(&self) -> &SourceProgramDeclarationPathV1 {
        &self.source_path
    }

    pub(in crate::parser) fn final_statement_slot(&self) -> u32 {
        self.final_statement_slot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) enum ProjectedProgramItemSlotErrorV1 {
    ForeignParser,
    DuplicateSourcePath,
    DuplicateFinalSlot,
    FinalSlotOverflow,
}

/// Complete selected top-level placement set for one parser invocation.
///
/// The constructor and mutation surface stay inside `build_cfg`; later parser
/// owners may only borrow or consume the already-complete set.
#[derive(Debug)]
pub(in crate::parser) struct ProjectedProgramItemSlotSetV1 {
    brand: ParserInvocationBrandV1,
    rows: Box<[ProjectedProgramItemSlotV1]>,
}

impl ProjectedProgramItemSlotSetV1 {
    pub(in crate::parser) fn rows(&self) -> &[ProjectedProgramItemSlotV1] {
        &self.rows
    }

    pub(in crate::parser) fn brand_matches(&self, brand: &ParserInvocationBrandV1) -> bool {
        self.brand.same_as(brand)
    }

    pub(in crate::parser) fn exact_final_slot(
        &self,
        source_path: &SourceProgramDeclarationPathV1,
    ) -> Result<Option<u32>, ProjectedProgramItemSlotErrorV1> {
        if !source_path.brand().same_as(&self.brand) {
            return Err(ProjectedProgramItemSlotErrorV1::ForeignParser);
        }
        Ok(self
            .rows
            .iter()
            .find(|row| row.source_path == *source_path)
            .map(|row| row.final_statement_slot))
    }
}

pub(super) struct OpenProjectedProgramItemSlotsV1 {
    brand: ParserInvocationBrandV1,
    rows: Vec<ProjectedProgramItemSlotV1>,
}

impl OpenProjectedProgramItemSlotsV1 {
    pub(super) fn open(brand: ParserInvocationBrandV1) -> Self {
        Self {
            brand,
            rows: Vec::new(),
        }
    }

    pub(super) fn brand(&self) -> &ParserInvocationBrandV1 {
        &self.brand
    }

    pub(super) fn record(
        &mut self,
        source_path: SourceProgramDeclarationPathV1,
        final_statement_slot: usize,
    ) -> Result<(), ProjectedProgramItemSlotErrorV1> {
        if !source_path.brand().same_as(&self.brand) {
            return Err(ProjectedProgramItemSlotErrorV1::ForeignParser);
        }
        if self.rows.iter().any(|row| row.source_path == source_path) {
            return Err(ProjectedProgramItemSlotErrorV1::DuplicateSourcePath);
        }
        let final_statement_slot = u32::try_from(final_statement_slot)
            .map_err(|_| ProjectedProgramItemSlotErrorV1::FinalSlotOverflow)?;
        if self
            .rows
            .iter()
            .any(|row| row.final_statement_slot == final_statement_slot)
        {
            return Err(ProjectedProgramItemSlotErrorV1::DuplicateFinalSlot);
        }
        self.rows.push(ProjectedProgramItemSlotV1 {
            source_path,
            final_statement_slot,
        });
        Ok(())
    }

    pub(super) fn finish(self) -> ProjectedProgramItemSlotSetV1 {
        ProjectedProgramItemSlotSetV1 {
            brand: self.brand,
            rows: self.rows.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::source_authority::SourceBoxDeclarationPathV1;

    #[test]
    fn open_set_rejects_foreign_duplicate_path_and_duplicate_slot() {
        let brand = ParserInvocationBrandV1::issue();
        let mut open = OpenProjectedProgramItemSlotsV1::open(brand.clone());
        let path = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(brand.clone(), 0),
        );
        open.record(path.clone(), 0).unwrap();
        assert_eq!(
            open.record(path, 1).unwrap_err(),
            ProjectedProgramItemSlotErrorV1::DuplicateSourcePath
        );
        let second = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(brand, 1),
        );
        assert_eq!(
            open.record(second, 0).unwrap_err(),
            ProjectedProgramItemSlotErrorV1::DuplicateFinalSlot
        );

        let foreign = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(ParserInvocationBrandV1::issue(), 0),
        );
        assert_eq!(
            open.record(foreign, 2).unwrap_err(),
            ProjectedProgramItemSlotErrorV1::ForeignParser
        );
    }

    #[test]
    fn sealed_set_distinguishes_missing_and_foreign_paths() {
        let brand = ParserInvocationBrandV1::issue();
        let mut open = OpenProjectedProgramItemSlotsV1::open(brand.clone());
        open.record(
            SourceProgramDeclarationPathV1::from_parser_path(SourceBoxDeclarationPathV1::root(
                brand.clone(),
                0,
            )),
            0,
        )
        .unwrap();
        let set = open.finish();
        let missing = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(brand, 1),
        );
        assert_eq!(set.exact_final_slot(&missing).unwrap(), None);
        let foreign = SourceProgramDeclarationPathV1::from_parser_path(
            SourceBoxDeclarationPathV1::root(ParserInvocationBrandV1::issue(), 0),
        );
        assert_eq!(
            set.exact_final_slot(&foreign).unwrap_err(),
            ProjectedProgramItemSlotErrorV1::ForeignParser
        );
    }
}
