/*!
 * Nested executor contract for array/text observer routes.
 *
 * This is implementation support for `array_text_observer_routes`, not a new
 * metadata family. MIR owns the legality/proof; backends only consume the
 * nested contract.
 */

mod model;
mod types;

pub use model::{ArrayTextObserverExecutorContract, ArrayTextObserverStoreRegionMapping};
pub use types::{
    ArrayTextObserverExecutorCarrier, ArrayTextObserverExecutorConsumerCapability,
    ArrayTextObserverExecutorEffect, ArrayTextObserverExecutorExecutionMode,
    ArrayTextObserverExecutorMaterializationPolicy, ArrayTextObserverExecutorProofRegion,
};
