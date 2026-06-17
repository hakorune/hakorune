/*!
 * Nested executor contract for array/text observer routes.
 *
 * This is implementation support for `array_text_observer_routes`, not a new
 * metadata family. MIR owns the legality/proof; backends only consume the
 * nested contract.
 */

mod matcher;
mod model;
mod types;

pub(crate) use matcher::derive_observer_store_region_contract;
pub use model::{ArrayTextObserverExecutorContract, ArrayTextObserverStoreRegionMapping};
pub use types::{
    ArrayTextObserverExecutorCarrier, ArrayTextObserverExecutorConsumerCapability,
    ArrayTextObserverExecutorEffect, ArrayTextObserverExecutorExecutionMode,
    ArrayTextObserverExecutorMaterializationPolicy, ArrayTextObserverExecutorProofRegion,
};
