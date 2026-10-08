//! Reduce only the same sealed handoff's exact Fresh descriptors, once per node.
use super::super::return_leaf::ObjectReturnTeardownDescriptorV1;
use super::{freeze, VerifiedObjectReturnAlternativeV1, VerifiedObjectReturnLeafV1};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) enum ObjectReturnTeardownAvailabilityV1
{
    Verified {
        descriptor: ObjectReturnTeardownDescriptorV1,
        nullable: bool,
    },
    Unavailable {
        descriptor: Option<ObjectReturnTeardownDescriptorV1>,
        nullable: bool,
        unsupported: bool,
    },
}
impl ObjectReturnTeardownAvailabilityV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn descriptor(
        &self,
    ) -> Option<(&ObjectReturnTeardownDescriptorV1, bool)> {
        match self {
            Self::Verified {
                descriptor,
                nullable,
            } => Some((descriptor, *nullable)),
            Self::Unavailable { .. } => None,
        }
    }
}

pub(super) fn reduce(
    alternatives: &[VerifiedObjectReturnAlternativeV1],
) -> Result<ObjectReturnTeardownAvailabilityV1, String> {
    let mut descriptor = None;
    let mut nullable = false;
    let mut unavailable = false;
    for alternative in alternatives {
        let candidate = match alternative {
            VerifiedObjectReturnAlternativeV1::Leaf(VerifiedObjectReturnLeafV1::Null) => {
                nullable = true;
                continue;
            }
            VerifiedObjectReturnAlternativeV1::Leaf(VerifiedObjectReturnLeafV1::Fresh {
                teardown,
                ..
            }) => teardown,
            VerifiedObjectReturnAlternativeV1::Call(child) => match child.teardown() {
                ObjectReturnTeardownAvailabilityV1::Verified {
                    descriptor,
                    nullable: child_nullable,
                } => {
                    nullable |= child_nullable;
                    descriptor
                }
                ObjectReturnTeardownAvailabilityV1::Unavailable {
                    descriptor,
                    nullable: child_nullable,
                    unsupported,
                } => {
                    unavailable |= unsupported;
                    nullable |= child_nullable;
                    let Some(descriptor) = descriptor else {
                        continue;
                    };
                    descriptor
                }
            },
        };
        if descriptor.is_some_and(|previous| previous != candidate) {
            return Err(freeze("teardown-descriptor-disagreement"));
        }
        descriptor = Some(candidate);
    }
    let Some(descriptor) = descriptor else {
        return Ok(ObjectReturnTeardownAvailabilityV1::Unavailable {
            descriptor: None,
            nullable,
            unsupported: unavailable,
        });
    };
    if unavailable || !descriptor.supports_direct_teardown(nullable) {
        return Ok(ObjectReturnTeardownAvailabilityV1::Unavailable {
            descriptor: Some(descriptor.clone()),
            nullable,
            unsupported: true,
        });
    }
    Ok(ObjectReturnTeardownAvailabilityV1::Verified {
        descriptor: descriptor.clone(),
        nullable,
    })
}
