use crate::ast::ASTNode;

use super::product::{NormalMainMethodSiteV1, NormalTopLevelSiteV1, SealedNormalMainSourceV1};

pub(super) fn main_at_root(source: ASTNode) -> SealedNormalMainSourceV1 {
    SealedNormalMainSourceV1::seal(
        source,
        NormalTopLevelSiteV1::new(0),
        NormalMainMethodSiteV1::new(0, "main".into(), 0, true),
    )
}
