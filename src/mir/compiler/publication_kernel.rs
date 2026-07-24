//! PUBLICATION0: the sole live-Builder replacement kernel.

use crate::mir::builder::{
    BuilderPublicationReceiptV1, MirBuilder, PreparedBuilderExternalCommitV1,
    RawExternalCommitModuleV1, RawPublishedModuleV1,
};

pub(in crate::mir) enum PublishedModuleTransferV1 {
    None,
    Raw(RawPublishedModuleV1),
}

pub(in crate::mir) trait SealedPublicationPayloadV1 {
    type Published;

    fn finish(
        self,
        receipt: BuilderPublicationReceiptV1,
        module: PublishedModuleTransferV1,
    ) -> Self::Published;
}

pub(in crate::mir) fn publish_once<P: SealedPublicationPayloadV1>(
    target: &mut MirBuilder,
    builder: PreparedBuilderExternalCommitV1,
    payload: P,
    raw_module: Option<RawExternalCommitModuleV1>,
) -> P::Published {
    let (receipt, module) = match raw_module {
        Some(raw_module) => {
            let (receipt, published) = builder.commit_raw_direct(target, raw_module);
            (receipt, PublishedModuleTransferV1::Raw(published))
        }
        None => (builder.commit(target), PublishedModuleTransferV1::None),
    };
    payload.finish(receipt, module)
}
