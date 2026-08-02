//! Recipe-local logical identities.

use serde::{Deserialize, Serialize};

macro_rules! logical_key {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub(crate) struct $name(u32);

        impl $name {
            pub(crate) const fn new(raw: u32) -> Self {
                Self(raw)
            }

            pub(crate) const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

logical_key!(LoopNodeKeyV1);
logical_key!(LoopBlockKeyV1);
logical_key!(LoopItemKeyV1);
logical_key!(LoopBindingKeyV1);
logical_key!(LoopValueKeyV1);
logical_key!(LoopCarrierKeyV1);
logical_key!(LoopExitKeyV1);
