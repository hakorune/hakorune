#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorExecutionMode {
    SingleRegionExecutor,
}

impl std::fmt::Display for ArrayTextObserverExecutorExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorProofRegion {
    LoopBackedgeSingleBody,
}

impl std::fmt::Display for ArrayTextObserverExecutorProofRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorCarrier {
    ArrayLaneTextCell,
}

impl std::fmt::Display for ArrayTextObserverExecutorCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorEffect {
    ObserveIndexOf,
    StoreCell,
}

impl std::fmt::Display for ArrayTextObserverExecutorEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorConsumerCapability {
    CompareOnly,
    SinkStore,
}

impl std::fmt::Display for ArrayTextObserverExecutorConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverExecutorMaterializationPolicy {
    TextResidentOrStringlikeSlot,
}

impl std::fmt::Display for ArrayTextObserverExecutorMaterializationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverExecutorExecutionMode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SingleRegionExecutor => "single_region_executor",
        }
    }
}

impl ArrayTextObserverExecutorProofRegion {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::LoopBackedgeSingleBody => "loop_backedge_single_body",
        }
    }
}

impl ArrayTextObserverExecutorCarrier {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ArrayLaneTextCell => "array_lane_text_cell",
        }
    }
}

impl ArrayTextObserverExecutorEffect {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ObserveIndexOf => "observe.indexof",
            Self::StoreCell => "store.cell",
        }
    }
}

impl ArrayTextObserverExecutorConsumerCapability {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::CompareOnly => "compare_only",
            Self::SinkStore => "sink_store",
        }
    }
}

impl ArrayTextObserverExecutorMaterializationPolicy {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::TextResidentOrStringlikeSlot => "text_resident_or_stringlike_slot",
        }
    }
}
