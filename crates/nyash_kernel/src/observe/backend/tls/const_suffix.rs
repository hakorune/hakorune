macro_rules! tls_bump_unit_method {
    ($($name:ident => $field:ident,)+) => {
        $(
            #[inline(always)]
            fn $name(&self) {
                Self::bump(&self.$field);
            }
        )+
    };
}

impl ThreadCounters {
    tls_bump_unit_method! {
        const_suffix_enter => const_suffix_total,
        const_suffix_cached_handle_hit => const_suffix_cached_handle_hit,
        const_suffix_text_cache_reload => const_suffix_text_cache_reload,
        const_suffix_freeze_fallback => const_suffix_freeze_fallback,
        const_suffix_empty_return => const_suffix_empty_return,
        const_suffix_cached_fast_str_hit => const_suffix_cached_fast_str_hit,
        const_suffix_cached_span_hit => const_suffix_cached_span_hit,
    }
}
