use super::types::{CarrierInit, CarrierRole, CarrierVar};
use crate::mir::ValueId;

impl CarrierVar {
    /// Create a new CarrierVar with default LoopState role.
    pub fn new(name: String, host_id: ValueId) -> Self {
        Self {
            name,
            host_id,
            join_id: None,
            role: CarrierRole::LoopState,
            init: CarrierInit::FromHost,
        }
    }

    /// Create a CarrierVar with explicit role.
    pub fn with_role(name: String, host_id: ValueId, role: CarrierRole) -> Self {
        Self {
            name,
            host_id,
            join_id: None,
            role,
            init: CarrierInit::FromHost,
        }
    }

    /// Create a CarrierVar with explicit role and init policy.
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
        }
    }
}
