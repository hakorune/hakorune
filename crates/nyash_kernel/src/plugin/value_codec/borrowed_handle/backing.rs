use nyash_rust::box_trait::NyashBox;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextKeepClass {
    StringBox,
    StringView,
}

#[derive(Debug, Clone)]
pub(crate) struct TextKeepBacking {
    pub(crate) stable_box: Arc<dyn NyashBox>,
}

impl TextKeepBacking {
    #[inline(always)]
    fn new(stable_box: Arc<dyn NyashBox>) -> Self {
        Self { stable_box }
    }
}

#[derive(Debug, Clone)]
/// Stable source proof + cached object reference for the current text read path.
/// This is runtime-private read-state, not the read-only `TextRef` view itself.
pub(crate) struct SourceLifetimeKeep {
    pub(crate) class: TextKeepClass,
    pub(crate) backing: TextKeepBacking,
}

impl SourceLifetimeKeep {
    #[inline(always)]
    pub(crate) fn string_box(obj: Arc<dyn NyashBox>) -> Self {
        Self {
            class: TextKeepClass::StringBox,
            backing: TextKeepBacking::new(obj),
        }
    }

    #[inline(always)]
    pub(crate) fn string_view(obj: Arc<dyn NyashBox>) -> Self {
        Self {
            class: TextKeepClass::StringView,
            backing: TextKeepBacking::new(obj),
        }
    }
}

#[derive(Debug, Clone)]
/// Internal lifetime state that keeps text reads anchored to a validated source
/// object. It supports `TextRef`, but is not the `TextRef` view itself.
pub(crate) struct TextKeep {
    pub(crate) source_lifetime: SourceLifetimeKeep,
}

impl TextKeep {
    #[inline(always)]
    pub(crate) fn new(source_lifetime: SourceLifetimeKeep) -> Self {
        Self { source_lifetime }
    }
}
