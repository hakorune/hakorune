mod cfg_gate;
mod declarations;
mod error;
mod model;
mod path_resolution;
mod traversal;

pub use error::ModuleTopologyErrorV1;
pub use model::{
    DeclaredModuleEdgeV1, DeclaredModuleInstanceV1, DeclaredModuleTopologyV1, ModuleEdgeKindV1,
    ModuleInstanceKindV1, ModuleSourceObservationV1,
};
pub use traversal::collect_declared_module_topology_v1;
