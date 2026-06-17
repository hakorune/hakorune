use super::fastpath::LocalFastPathFallbackReason;
use super::ids::{ObjectBasicBlockId, ObjectInstructionIndex, ObjectSiteLocation, ObjectValueId};

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
    pub location: ObjectSiteLocation,
}

impl ObjectPublicationSite {
    #[inline]
    pub const fn location(&self) -> ObjectSiteLocation {
        self.location
    }

    #[inline]
    pub const fn block_id(&self) -> ObjectBasicBlockId {
        self.location.block_id
    }

    #[inline]
    pub const fn instruction_index(&self) -> ObjectInstructionIndex {
        self.location.instruction_index
    }
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
