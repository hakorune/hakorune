use super::backing::{SourceLifetimeKeep, TextKeep};
use crate::observe;
use crate::plugin::value_demand::{
    DemandSet, BORROWED_ALIAS_ENCODE, BORROWED_ALIAS_FALLBACK_PUBLISH,
};
use nyash_rust::{
    box_trait::{next_box_id, BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{
    any::Any,
    sync::{
        atomic::{AtomicI64, AtomicU64, Ordering},
        Arc,
    },
};

#[derive(Clone, Copy)]
pub(crate) enum BorrowedAliasEncodeCaller {
    #[cfg(test)]
    Generic,
    ArrayGetIndexEncoded,
    MapRuntimeDataGetAnyKey,
}

impl BorrowedAliasEncodeCaller {
    #[inline(always)]
    pub(crate) const fn demand(self) -> DemandSet {
        match self {
            #[cfg(test)]
            Self::Generic => BORROWED_ALIAS_ENCODE,
            Self::ArrayGetIndexEncoded | Self::MapRuntimeDataGetAnyKey => BORROWED_ALIAS_ENCODE,
        }
    }

    #[inline(always)]
    fn record(self) {
        self.record_metric(
            observe::record_borrowed_alias_encode_to_handle_arc_array_get_index,
            observe::record_borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any,
        );
    }

    #[inline(always)]
    fn record_live_source_hit(self) {
        self.record_metric(
            observe::record_borrowed_alias_encode_live_source_hit_array_get_index,
            observe::record_borrowed_alias_encode_live_source_hit_map_runtime_data_get_any,
        );
    }

    #[inline(always)]
    fn record_cached_handle_hit(self) {
        self.record_metric(
            observe::record_borrowed_alias_encode_cached_handle_hit_array_get_index,
            observe::record_borrowed_alias_encode_cached_handle_hit_map_runtime_data_get_any,
        );
    }

    #[inline(always)]
    fn record_metric(self, array: fn(), map: fn()) {
        match self {
            #[cfg(test)]
            Self::Generic => {}
            Self::ArrayGetIndexEncoded => array(),
            Self::MapRuntimeDataGetAnyKey => map(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct AliasSourceMeta {
    pub(super) source_handle: i64,
    pub(super) source_drop_epoch: u64,
}

impl AliasSourceMeta {
    #[inline(always)]
    fn new(source_handle: i64, source_drop_epoch: u64) -> Self {
        Self {
            source_handle,
            source_drop_epoch,
        }
    }

    #[inline(always)]
    fn borrowed_handle_source_fast(self) -> Option<(i64, u64)> {
        if self.source_handle > 0 {
            Some((self.source_handle, self.source_drop_epoch))
        } else {
            None
        }
    }
}

#[derive(Debug)]
/// Runtime-private borrowed-alias encode wrapper around stable object text.
/// It feeds the read-only `TextRef` path, but does not own that view itself.
pub(crate) struct BorrowedHandleBox {
    pub(super) text_keep: TextKeep,
    pub(super) source_meta: AliasSourceMeta,
    cached_runtime_handle: Arc<BorrowedHandleRuntimeCache>,
    base: BoxBase,
}

#[derive(Debug)]
struct BorrowedHandleRuntimeCache {
    handle: AtomicI64,
    epoch: AtomicU64,
    live_source_handle: AtomicI64,
    live_source_epoch: AtomicU64,
}

impl BorrowedHandleRuntimeCache {
    fn new() -> Self {
        Self {
            handle: AtomicI64::new(0),
            epoch: AtomicU64::new(0),
            live_source_handle: AtomicI64::new(0),
            live_source_epoch: AtomicU64::new(0),
        }
    }
}

impl Clone for BorrowedHandleBox {
    fn clone(&self) -> Self {
        Self {
            text_keep: self.text_keep.clone(),
            source_meta: self.source_meta,
            cached_runtime_handle: Arc::clone(&self.cached_runtime_handle),
            base: self.base.clone(),
        }
    }
}

impl BorrowedHandleBox {
    pub(super) fn new(
        keep: SourceLifetimeKeep,
        source_handle: i64,
        source_drop_epoch: u64,
    ) -> Self {
        let stable_id = if source_handle > 0 {
            source_handle as u64
        } else {
            next_box_id()
        };
        Self {
            text_keep: TextKeep::new(keep),
            source_meta: AliasSourceMeta::new(source_handle, source_drop_epoch),
            cached_runtime_handle: Arc::new(BorrowedHandleRuntimeCache::new()),
            base: BoxBase {
                id: stable_id,
                parent_type_id: None,
            },
        }
    }

    #[inline(always)]
    fn cached_runtime_handle_at_epoch(&self, current_epoch: u64) -> Option<i64> {
        let handle = self.cached_runtime_handle.handle.load(Ordering::Relaxed);
        if handle > 0 && self.cached_runtime_handle.epoch.load(Ordering::Relaxed) == current_epoch {
            Some(handle)
        } else {
            None
        }
    }

    #[inline(always)]
    fn validated_live_source_handle_at_epoch(
        &self,
        current_epoch: u64,
        source_handle: i64,
    ) -> Option<i64> {
        let handle = self
            .cached_runtime_handle
            .live_source_handle
            .load(Ordering::Relaxed);
        if handle == source_handle
            && handle > 0
            && self
                .cached_runtime_handle
                .live_source_epoch
                .load(Ordering::Relaxed)
                == current_epoch
        {
            Some(handle)
        } else {
            None
        }
    }

    #[inline(always)]
    fn cache_runtime_handle(&self, handle: i64) {
        self.cached_runtime_handle
            .handle
            .store(handle, Ordering::Relaxed);
        self.cached_runtime_handle
            .epoch
            .store(handles::drop_epoch(), Ordering::Relaxed);
    }

    #[inline(always)]
    fn cache_validated_live_source_handle(&self, handle: i64, current_epoch: u64) {
        self.cached_runtime_handle
            .live_source_handle
            .store(handle, Ordering::Relaxed);
        self.cached_runtime_handle
            .live_source_epoch
            .store(current_epoch, Ordering::Relaxed);
    }

    #[inline(always)]
    #[cfg(test)]
    pub(super) fn invalidate_cached_runtime_handle(&self) {
        self.cached_runtime_handle
            .handle
            .store(0, Ordering::Relaxed);
        self.cached_runtime_handle.epoch.store(0, Ordering::Relaxed);
        self.cached_runtime_handle
            .live_source_handle
            .store(0, Ordering::Relaxed);
        self.cached_runtime_handle
            .live_source_epoch
            .store(0, Ordering::Relaxed);
    }

    #[inline(always)]
    fn source_is_latest_fresh(&self) -> bool {
        self.source_meta.source_handle > 0
            && observe::len_route_matches_latest_fresh_handle(self.source_meta.source_handle)
    }

    #[cold]
    #[inline(never)]
    fn equals_cold_promoted_object(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self
                .text_keep
                .source_lifetime
                .backing
                .stable_box
                .as_ref()
                .equals(
                    other_alias
                        .text_keep
                        .source_lifetime
                        .backing
                        .stable_box
                        .as_ref(),
                );
        }
        self.text_keep
            .source_lifetime
            .backing
            .stable_box
            .as_ref()
            .equals(other)
    }
}

impl BoxCore for BorrowedHandleBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(text) = self
            .text_keep
            .source_lifetime
            .backing
            .stable_box
            .as_ref()
            .as_str_fast()
        {
            return write!(f, "\"{}\"", text);
        }
        write!(
            f,
            "\"{}\"",
            self.text_keep
                .source_lifetime
                .backing
                .stable_box
                .as_ref()
                .to_string_box()
                .value
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for BorrowedHandleBox {
    fn to_string_box(&self) -> StringBox {
        observe::record_borrowed_alias_to_string_box();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_to_string_box_latest_fresh();
        }
        StringBox::new(
            self.text_keep
                .source_lifetime
                .backing
                .stable_box
                .as_ref()
                .to_string_box()
                .value,
        )
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        observe::record_borrowed_alias_equals();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_equals_latest_fresh();
        }
        self.equals_cold_promoted_object(other)
    }

    fn type_name(&self) -> &'static str {
        match self.text_keep.source_lifetime.class {
            super::backing::TextKeepClass::StringBox => "StringBox",
            super::backing::TextKeepClass::StringView => "StringViewBox",
        }
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        observe::record_borrowed_alias_clone_box();
        if self.source_is_latest_fresh() {
            observe::record_borrowed_alias_clone_box_latest_fresh();
        }
        Box::new(self.clone())
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn is_identity(&self) -> bool {
        false
    }

    fn borrowed_handle_source_fast(&self) -> Option<(i64, u64)> {
        observe::record_borrowed_alias_borrowed_source_fast();
        self.source_meta.borrowed_handle_source_fast()
    }

    fn as_str_fast(&self) -> Option<&str> {
        observe::record_borrowed_alias_as_str_fast();
        if observe::enabled() {
            if self.source_meta.source_handle > 0 {
                if self.source_meta.source_drop_epoch == handles::drop_epoch() {
                    observe::record_borrowed_alias_as_str_fast_live_source();
                } else {
                    observe::record_borrowed_alias_as_str_fast_stale_source();
                }
            }
        }
        // Object-boundary NyashBox still exposes `&str`, but the underlying read path
        // stays in the runtime-private borrowed-alias encode path until this final projection.
        self.text_keep
            .source_lifetime
            .backing
            .stable_box
            .as_ref()
            .as_str_fast()
    }
}

#[derive(Clone, Copy)]
enum BorrowedAliasEncodePlan {
    LiveSourceHandle(i64),
    CachedRuntimeHandle(i64),
    EncodeFallback,
}

impl BorrowedAliasEncodePlan {
    #[inline(always)]
    const fn demand(self) -> DemandSet {
        match self {
            Self::LiveSourceHandle(_) | Self::CachedRuntimeHandle(_) => BORROWED_ALIAS_ENCODE,
            Self::EncodeFallback => BORROWED_ALIAS_FALLBACK_PUBLISH,
        }
    }
}

#[inline(always)]
pub(crate) fn runtime_i64_from_borrowed_alias(
    alias: &BorrowedHandleBox,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    let _caller_demand = caller.demand();
    let plan = plan_borrowed_alias_runtime_i64(alias);
    let _plan_demand = plan.demand();
    match plan {
        BorrowedAliasEncodePlan::LiveSourceHandle(handle) => {
            observe::record_borrowed_alias_encode_live_source_hit();
            caller.record_live_source_hit();
            handle
        }
        BorrowedAliasEncodePlan::CachedRuntimeHandle(handle) => {
            observe::record_borrowed_alias_encode_cached_handle_hit();
            caller.record_cached_handle_hit();
            handle
        }
        BorrowedAliasEncodePlan::EncodeFallback => {
            observe::record_borrowed_alias_encode_to_handle_arc();
            caller.record();
            let handle =
                handles::to_handle_arc(alias.text_keep.source_lifetime.backing.stable_box.clone())
                    as i64;
            alias.cache_runtime_handle(handle);
            handle
        }
    }
}

#[inline(always)]
fn plan_borrowed_alias_runtime_i64(alias: &BorrowedHandleBox) -> BorrowedAliasEncodePlan {
    let current_epoch = handles::drop_epoch();
    let source_handle = alias.source_meta.source_handle;
    if source_handle > 0 {
        if alias.source_meta.source_drop_epoch == current_epoch {
            observe::record_borrowed_alias_encode_epoch_hit();
            alias.cache_validated_live_source_handle(source_handle, current_epoch);
            return BorrowedAliasEncodePlan::LiveSourceHandle(source_handle);
        }
        if let Some(handle) =
            alias.validated_live_source_handle_at_epoch(current_epoch, source_handle)
        {
            return BorrowedAliasEncodePlan::LiveSourceHandle(handle);
        }
    }
    if let Some(cached) = alias.cached_runtime_handle_at_epoch(current_epoch) {
        return BorrowedAliasEncodePlan::CachedRuntimeHandle(cached);
    }
    if source_handle > 0 {
        if let Some(source_obj) = handles::get(source_handle as u64) {
            if Arc::ptr_eq(
                &alias.text_keep.source_lifetime.backing.stable_box,
                &source_obj,
            ) {
                observe::record_borrowed_alias_encode_ptr_eq_hit();
                alias.cache_validated_live_source_handle(source_handle, current_epoch);
                return BorrowedAliasEncodePlan::LiveSourceHandle(source_handle);
            }
        }
    }
    BorrowedAliasEncodePlan::EncodeFallback
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::handle_registry_test_lock;

    #[test]
    fn ptr_eq_validated_live_source_is_cached_for_current_epoch() {
        let _guard = handle_registry_test_lock();
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("live-source-cache".to_string()));
        let source_handle = handles::to_handle_arc(source.clone()) as i64;
        let stale_epoch = handles::drop_epoch();

        let epoch_bump: Arc<dyn NyashBox> = Arc::new(StringBox::new("epoch-bump".to_string()));
        let epoch_bump_handle = handles::to_handle_arc(epoch_bump) as i64;
        handles::drop_handle(epoch_bump_handle as u64);
        let current_epoch = handles::drop_epoch();
        assert!(current_epoch > stale_epoch);

        let alias = BorrowedHandleBox::new(
            SourceLifetimeKeep::string_box(source),
            source_handle,
            stale_epoch,
        );
        assert_eq!(
            alias.validated_live_source_handle_at_epoch(current_epoch, source_handle),
            None
        );

        assert!(matches!(
            plan_borrowed_alias_runtime_i64(&alias),
            BorrowedAliasEncodePlan::LiveSourceHandle(handle) if handle == source_handle
        ));
        assert_eq!(
            alias.validated_live_source_handle_at_epoch(current_epoch, source_handle),
            Some(source_handle)
        );

        assert!(matches!(
            plan_borrowed_alias_runtime_i64(&alias),
            BorrowedAliasEncodePlan::LiveSourceHandle(handle) if handle == source_handle
        ));

        handles::drop_handle(source_handle as u64);
    }
}
