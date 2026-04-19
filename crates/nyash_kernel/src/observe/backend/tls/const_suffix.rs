impl ThreadCounters {
    #[inline(always)]
    fn const_suffix_enter(&self) {
        Self::bump(&self.const_suffix_total);
    }

    #[inline(always)]
    fn const_suffix_cached_handle_hit(&self) {
        Self::bump(&self.const_suffix_cached_handle_hit);
    }

    #[inline(always)]
    fn const_suffix_text_cache_reload(&self) {
        Self::bump(&self.const_suffix_text_cache_reload);
    }

    #[inline(always)]
    fn const_suffix_freeze_fallback(&self) {
        Self::bump(&self.const_suffix_freeze_fallback);
    }

    #[inline(always)]
    fn const_suffix_empty_return(&self) {
        Self::bump(&self.const_suffix_empty_return);
    }

    #[inline(always)]
    fn const_suffix_cached_fast_str_hit(&self) {
        Self::bump(&self.const_suffix_cached_fast_str_hit);
    }

    #[inline(always)]
    fn const_suffix_cached_span_hit(&self) {
        Self::bump(&self.const_suffix_cached_span_hit);
    }

}
