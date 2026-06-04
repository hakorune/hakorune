"""Replacement-front shim report/emission raw C source only."""

from __future__ import annotations


REPORT_C = r"""
static void write_str(int fd, const char* s) {
  size_t len = 0;
  while (s[len]) {
    len++;
  }
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
#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
  flush_thread_counters();
#endif
  const char* path = getenv("HAKORUNE_REPLACEMENT_FRONT_REPORT");
  if (!path || !path[0]) {
    return;
  }
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) {
    return;
  }
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
  write_kv(fd, "replacement_front_thread_local_mode_enabled", thread_local_mode_enabled);
  write_kv(fd, "replacement_front_remote_free_push_count", remote_free_push_count);
  write_kv(fd, "replacement_front_remote_free_drain_count", remote_free_drain_count);
  write_kv(
      fd,
      "replacement_front_cross_thread_realloc_unsupported_count",
      cross_thread_realloc_unsupported_count);
  write_kv(fd, "replacement_front_arena_registry_overflow_count", arena_registry_overflow_count);
  write_kv(fd, "replacement_front_abandoned_arena_count", abandoned_arena_count);
  write_kv(fd, "replacement_front_abandoned_remote_free_count", abandoned_remote_free_count);
  write_kv(fd, "replacement_front_skip_hot_counters_enabled", skip_hot_counters_enabled);
  write_kv(fd, "replacement_front_tls_counter_mode_enabled", tls_counter_mode_enabled);
  write_kv(
      fd,
      "replacement_front_tls_initial_exec_model_enabled",
      tls_initial_exec_model_enabled);
  close(fd);
}

__attribute__((constructor))
static void install_report(void) {
#ifdef HAKO_REPLACEMENT_FRONT_LOCKED
  lock_mode_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_THREAD_LOCAL
  thread_local_mode_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_SKIP_HOT_COUNTERS
  skip_hot_counters_enabled = 1;
#endif
#ifdef HAKO_REPLACEMENT_FRONT_TLS_COUNTERS
  tls_counter_mode_enabled = 1;
#endif
#if defined(HAKO_REPLACEMENT_FRONT_THREAD_LOCAL) && defined(__GNUC__)
  tls_initial_exec_model_enabled = 1;
#endif
  atexit(write_report);
}
"""
