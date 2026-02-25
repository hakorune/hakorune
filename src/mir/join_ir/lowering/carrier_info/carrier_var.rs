use super::types::{CarrierInit, CarrierRole, CarrierVar};
use crate::mir::ValueId;

impl CarrierVar {
    /// Create a new CarrierVar with default LoopState role
    ///
    /// This is the primary constructor for CarrierVar. Use this instead of
    /// struct literal syntax to ensure role defaults to LoopState.
    pub fn new(name: String, host_id: ValueId) -> Self {
        Self {
            name,
            host_id,
            join_id: None,
            role: CarrierRole::LoopState,
            init: CarrierInit::FromHost, // Phase 228: Default to FromHost
            #[cfg(feature = "normalized_dev")]
            binding_id: None, // Phase 78: No BindingId by default
        }
    }

    /// Create a CarrierVar with explicit role
    pub fn with_role(name: String, host_id: ValueId, role: CarrierRole) -> Self {
        Self {
            name,
            host_id,
            join_id: None,
            role,
            init: CarrierInit::FromHost, // Phase 228: Default to FromHost
            #[cfg(feature = "normalized_dev")]
            binding_id: None, // Phase 78: No BindingId by default
        }
    }

    /// Phase 228: Create a CarrierVar with explicit role and init policy
    pub fn with_role_and_init(
        name: String,
        host_id: ValueId,
        role: CarrierRole,
        init: CarrierInit,
    ) -> Self {
        Self {
            name,
            host_id,
            join_id: None,
            role,
            init,
            #[cfg(feature = "normalized_dev")]
            binding_id: None, // Phase 78: No BindingId by default
        }
    }
}
