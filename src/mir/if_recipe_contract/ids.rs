//! Recipe-local logical identities for the first portable If shell.

use serde::{Deserialize, Serialize};

macro_rules! if_key {
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

if_key!(IfBlockKeyV1);
if_key!(IfItemKeyV1);
if_key!(IfBindingKeyV1);
if_key!(IfValueKeyV1);
