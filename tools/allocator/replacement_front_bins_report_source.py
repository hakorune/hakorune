"""Multi-bin replacement-front benchmark report/emission raw C source only."""

from __future__ import annotations


REPORT_C = r"""
static void write_str(int fd, const char* s) {
  size_t len = 0;
  while (s[len]) len++;
  ssize_t ignored = write(fd, s, len);
  (void)ignored;
}

static void write_u64(int fd, unsigned long long value) {
  char buf[32];
  int pos = 31;
  buf[pos--] = '\n';
  if (value == 0) {
    buf[pos--] = '0';
  } else {
    while (value > 0 && pos >= 0) {
      buf[pos--] = (char)('0' + (value % 10));
      value /= 10;
    }
  }
  ssize_t ignored = write(fd, buf + pos + 1, (size_t)(31 - pos));
  (void)ignored;
}

static void write_kv(int fd, const char* key, unsigned long long value) {
  write_str(fd, key);
  write_str(fd, "=");
  write_u64(fd, value);
}

static void write_report(void) {
  const char* path = getenv("HAKORUNE_REPLACEMENT_FRONT_REPORT");
  if (!path || !path[0]) return;
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) return;
  write_kv(fd, "replacement_front_alloc_count", alloc_count);
  write_kv(fd, "replacement_front_calloc_count", calloc_count);
  write_kv(fd, "replacement_front_realloc_count", realloc_count);
  write_kv(fd, "replacement_front_free_count", free_count);
  write_kv(fd, "replacement_front_host_passthrough_count", host_passthrough_count);
  write_kv(fd, "replacement_front_direct_core_call_count", direct_core_call_count);
  write_kv(fd, "replacement_front_realloc_copy_bytes", realloc_copy_bytes);
  write_kv(fd, "replacement_front_realloc_inplace_count", realloc_inplace_count);
  write_kv(fd, "replacement_front_calloc_zero_bytes", calloc_zero_bytes);
  write_kv(fd, "replacement_front_lock_mode_enabled", lock_mode_enabled);
  write_kv(fd, "replacement_front_lock_enter_count", lock_enter_count);
  write_kv(fd, "replacement_front_skip_hot_counters_enabled", skip_hot_counters_enabled);
  write_kv(fd, "replacement_front_thread_local_page_bins_mode_enabled", thread_local_page_bins_mode_enabled);
  write_kv(fd, "replacement_front_malloc_tls_fast_count", malloc_tls_fast_count);
  write_kv(fd, "replacement_front_malloc_tls_refill_slow_count", malloc_tls_refill_slow_count);
  write_kv(fd, "replacement_front_same_thread_alloc_local_count", same_thread_alloc_local_count);
  write_kv(fd, "replacement_front_same_thread_free_local_count", same_thread_free_local_count);
  write_kv(fd, "replacement_front_cross_thread_free_remote_push_count", cross_thread_free_remote_push_count);
  write_kv(fd, "replacement_front_remote_free_drain_count", remote_free_drain_count);
  write_kv(fd, "replacement_front_remote_free_cas_retry_count", remote_free_cas_retry_count);
  write_kv(fd, "replacement_front_global_lock_hot_path_count", global_lock_hot_path_count);
  write_kv(fd, "replacement_front_global_lock_refill_count", global_lock_refill_count);
  write_kv(fd, "replacement_front_global_lock_reclaim_count", global_lock_reclaim_count);
  write_kv(fd, "replacement_front_tls_arena_count", tls_arena_count);
  write_kv(fd, "replacement_front_tls_arena_peak_count", tls_arena_peak_count);
  write_kv(fd, "replacement_front_thread_exit_arena_flush_count", thread_exit_arena_flush_count);
  write_kv(fd, "replacement_front_abandoned_owner_count", abandoned_owner_count);
  write_kv(fd, "replacement_front_abandoned_remote_free_count", abandoned_remote_free_count);
  write_kv(fd, "replacement_front_page_index_insert_count", page_index_insert_count);
  write_kv(fd, "replacement_front_page_index_probe_count", page_index_probe_count);
  write_kv(fd, "replacement_front_page_index_collision_count", page_index_collision_count);
  write_kv(fd, "replacement_front_page_index_overflow_count", page_index_overflow_count);
  close(fd);
}

__attribute__((constructor)) static void replacement_front_init(void) {
  resolve_real();
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  lock_mode_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS
  skip_hot_counters_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_TLS_PAGE_ARENA
  thread_local_page_bins_mode_enabled = 1;
#endif
  atexit(write_report);
}
"""
