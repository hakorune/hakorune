use super::fastpath::LocalFastPathFallbackReason;
use super::ids::{ObjectBasicBlockId, ObjectInstructionIndex, ObjectValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectPublicationReason {
    PluginOrExternBoundary,
    HostHandleRequired,
    DynamicArrayOrMapStorage,
    DynamicNyashBoxApi,
    ReturnAsDynamicBox,
    TaskFutureChannelBoundary,
    UnknownFiniOrDrop,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicationState {
    Unpublished,
    Published,
    MaybePublished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectPublicationSite {
    pub value_id: ObjectValueId,
    pub reason: ObjectPublicationReason,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
}

impl PublicationState {
    #[inline]
    pub fn permits_local_fast_path(self) -> bool {
        matches!(self, Self::Unpublished)
    }

    #[inline]
    pub fn fallback_reason(self) -> Option<LocalFastPathFallbackReason> {
        match self {
            Self::Unpublished => None,
            Self::Published => Some(LocalFastPathFallbackReason::PublishedBeforeSite),
            Self::MaybePublished => Some(LocalFastPathFallbackReason::MaybePublishedBeforeSite),
        }
    }
}
