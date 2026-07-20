mod adapter;
mod error;
mod model;

pub use adapter::seal_declared_cargo_unit_v1;
pub use error::CargoEvidenceErrorV1;
pub use model::{
    CargoDeclaredCompileUnitEvidenceV1, CargoMetadataSnapshotV1, CargoPackageDeclarationEvidenceV1,
    CargoPackageSnapshotV1, CargoResolveNodeSnapshotV1, CargoTargetDeclarationEvidenceV1,
    CargoTargetSnapshotV1,
};
