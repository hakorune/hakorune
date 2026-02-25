//! PathBox - path operations via provider-lock PathService
//!
//! PathBox is intentionally thin: path semantics live in ring1 providers.

use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox};
use crate::runtime::provider_lock;
use crate::runtime::provider_lock::PathService;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub struct PathBox {
    provider: Arc<dyn PathService>,
    base: BoxBase,
}

impl PathBox {
    /// Create new PathBox (panic-on-missing-provider helper).
    pub fn new() -> Self {
        Self::try_new().expect("PathBox provider is not initialized. Call Runtime::initialize() first.")
    }

    /// Result-based constructor used by factories.
    pub fn try_new() -> Result<Self, String> {
        let provider = provider_lock::get_pathbox_provider_instance()?;
        Ok(Self {
            provider,
            base: BoxBase::new(),
        })
    }

    pub fn join(&self, base: &str, rest: &str) -> String {
        self.provider.join(base, rest)
    }

    pub fn dirname(&self, path: &str) -> String {
        self.provider.dirname(path)
    }

    pub fn basename(&self, path: &str) -> String {
        self.provider.basename(path)
    }

    pub fn extname(&self, path: &str) -> String {
        self.provider.extname(path)
    }

    pub fn is_abs(&self, path: &str) -> bool {
        self.provider.is_abs(path)
    }

    pub fn normalize(&self, path: &str) -> String {
        self.provider.normalize(path)
    }
}

impl Clone for PathBox {
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            base: BoxBase::new(),
        }
    }
}

impl Debug for PathBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathBox").finish()
    }
}

impl BoxCore for PathBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PathBox")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for PathBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new("PathBox")
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        BoolBox::new(other.as_any().downcast_ref::<PathBox>().is_some())
    }

    fn type_name(&self) -> &'static str {
        "PathBox"
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(self.clone())
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }
}

impl Display for PathBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::ring1::path::Ring1PathService;

    fn init_provider_once() {
        let _ = provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()));
    }

    #[test]
    fn pathbox_basics() {
        init_provider_once();
        let p = PathBox::new();
        assert_eq!(p.join("apps", "tests"), "apps/tests");
        assert_eq!(p.dirname("apps/tests/main.hako"), "apps/tests");
        assert_eq!(p.basename("apps/tests/main.hako"), "main.hako");
        assert_eq!(p.extname("apps/tests/main.hako"), ".hako");
        assert!(!p.is_abs("apps/tests/main.hako"));
        assert_eq!(p.normalize("./apps/./tests/../tests/main.hako"), "apps/tests/main.hako");
    }
}
