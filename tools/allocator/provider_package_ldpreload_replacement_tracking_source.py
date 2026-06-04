"""Pointer tracking and report raw C chunk for provider-backed LD_PRELOAD smoke probes."""

from __future__ import annotations


TRACKING_C = r"""
static void hako_track_ptr(void* ptr, size_t size) {
  if (!ptr) {
    return;
  }
  unsigned int hash = hako_ptr_hash(ptr);
  for (unsigned int probe = 0; probe < HAKO_POINTER_TABLE_CAP; probe++) {
    unsigned int index = (unsigned int)((hash + probe) % HAKO_POINTER_TABLE_CAP);
    unsigned int probe_count = probe + 1u;
    if (tracked[index].ptr == ptr) {
      track_probe_total += probe_count;
      if (probe_count > track_probe_max) {
        track_probe_max = probe_count;
      }
      tracked[index].size = size;
      return;
    }
    if (!tracked[index].ptr) {
      track_probe_total += probe_count;
      if (probe_count > track_probe_max) {
        track_probe_max = probe_count;
      }
      tracked[index].ptr = ptr;
      tracked[index].size = size;
      return;
    }
  }
  track_probe_total += HAKO_POINTER_TABLE_CAP;
  if (HAKO_POINTER_TABLE_CAP > track_probe_max) {
    track_probe_max = HAKO_POINTER_TABLE_CAP;
  }
  pointer_table_overflow++;
}

static void hako_untrack_index(int index) {
  if (index < 0) {
    return;
  }
  unsigned int hole = (unsigned int)index;
  for (;;) {
    unsigned int next = (unsigned int)((hole + 1u) % HAKO_POINTER_TABLE_CAP);
    if (!tracked[next].ptr) {
      tracked[hole].ptr = 0;
      tracked[hole].size = 0;
      return;
    }
    unsigned int home = hako_ptr_hash(tracked[next].ptr);
    unsigned int distance = (unsigned int)((next + HAKO_POINTER_TABLE_CAP - home) % HAKO_POINTER_TABLE_CAP);
    if (distance == 0) {
      tracked[hole].ptr = 0;
      tracked[hole].size = 0;
      return;
    }
    tracked[hole] = tracked[next];
    hole = next;
  }
}

static void hako_write_str(int fd, const char* s) {
  size_t len = 0;
  while (s[len]) {
    len++;
  }
  ssize_t ignored = write(fd, s, len);
  (void)ignored;
}

static void hako_write_u64(int fd, unsigned long long value) {
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

static void hako_write_kv(int fd, const char* key, unsigned long long value) {
  hako_write_str(fd, key);
  hako_write_str(fd, "=");
  hako_write_u64(fd, value);
}

static void hako_write_report(void) {
  const char* path = getenv("HAKORUNE_PROVIDER_LDPRELOAD_REPORT");
  if (!path || !path[0]) {
    return;
  }
  int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd < 0) {
    return;
  }
  hako_write_kv(fd, "shim_provider_bind_success", provider_bind_success);
  hako_write_kv(fd, "shim_provider_bind_failure", provider_bind_failure);
  hako_write_kv(fd, "shim_provider_alloc_count", provider_alloc_count);
  hako_write_kv(fd, "shim_provider_calloc_count", provider_calloc_count);
  hako_write_kv(fd, "shim_provider_realloc_count", provider_realloc_count);
  hako_write_kv(fd, "shim_provider_free_count", provider_free_count);
  hako_write_kv(fd, "shim_provider_free_claim_count", provider_free_claim_count);
  hako_write_kv(fd, "shim_provider_free_not_owned_count", provider_free_not_owned_count);
  hako_write_kv(fd, "shim_provider_free_claim_bound", provider_free_claim_bound);
  hako_write_kv(fd, "shim_provider_usable_size_claim_count", provider_usable_size_claim_count);
  hako_write_kv(fd, "shim_provider_usable_size_not_owned_count", provider_usable_size_not_owned_count);
  hako_write_kv(fd, "shim_provider_usable_size_claim_bound", provider_usable_size_claim_bound);
  hako_write_kv(fd, "shim_provider_realloc_claim_count", provider_realloc_claim_count);
  hako_write_kv(fd, "shim_provider_realloc_not_owned_count", provider_realloc_not_owned_count);
  hako_write_kv(fd, "shim_provider_realloc_failed_count", provider_realloc_failed_count);
  hako_write_kv(fd, "shim_provider_realloc_claim_bound", provider_realloc_claim_bound);
  hako_write_kv(fd, "shim_host_allocator_init_bound", host_allocator_init_bound);
  hako_write_kv(fd, "shim_host_allocator_init_result", host_allocator_init_result);
  hako_write_kv(fd, "shim_host_allocator_vtable_init_count", host_allocator_vtable_init_count);
  hako_write_kv(fd, "shim_host_allocator_usable_size_bound", host_allocator_usable_size_bound);
  hako_write_kv(fd, "shim_runtime_real_fallback_count", runtime_real_fallback_count);
  hako_write_kv(fd, "shim_init_real_fallback_count", init_real_fallback_count);
  hako_write_kv(fd, "shim_init_fallback_loading_provider_count", init_fallback_loading_provider_count);
  hako_write_kv(fd, "shim_init_fallback_resolving_real_count", init_fallback_resolving_real_count);
  hako_write_kv(fd, "shim_init_fallback_in_provider_call_count", init_fallback_in_provider_call_count);
  hako_write_kv(fd, "shim_malloc_init_fallback_count", malloc_init_fallback_count);
  hako_write_kv(fd, "shim_calloc_init_fallback_count", calloc_init_fallback_count);
  hako_write_kv(fd, "shim_realloc_init_fallback_count", realloc_init_fallback_count);
  hako_write_kv(fd, "shim_free_init_fallback_count", free_init_fallback_count);
  hako_write_kv(fd, "shim_host_passthrough_count", host_passthrough_count);
  hako_write_kv(fd, "shim_pointer_table_overflow", pointer_table_overflow);
  hako_write_kv(fd, "shim_track_probe_total", track_probe_total);
  hako_write_kv(fd, "shim_track_probe_max", track_probe_max);
  hako_write_kv(fd, "shim_find_probe_total", find_probe_total);
  hako_write_kv(fd, "shim_find_probe_max", find_probe_max);
  hako_write_kv(fd, "shim_tombstone_hit_count", tombstone_hit_count);
  hako_write_kv(fd, "shim_tracked_hit_count", tracked_hit_count);
  hako_write_kv(fd, "shim_tracked_miss_count", tracked_miss_count);
  hako_write_kv(fd, "shim_calloc_zero_bytes", calloc_zero_bytes);
  hako_write_kv(fd, "shim_realloc_copy_bytes", realloc_copy_bytes);
  hako_write_kv(fd, "shim_realloc_tracked_count", realloc_tracked_count);
  hako_write_kv(fd, "shim_realloc_host_passthrough_count", realloc_host_passthrough_count);
  hako_write_kv(fd, "shim_realloc_null_count", realloc_null_count);
  hako_write_kv(fd, "shim_realloc_free_count", realloc_free_count);
  hako_write_kv(fd, "shim_usable_size_mode_enabled", usable_size_mode_enabled);
  hako_write_kv(fd, "shim_usable_size_symbol_bound", usable_size_symbol_bound);
  hako_write_kv(fd, "shim_usable_size_lookup_count", usable_size_lookup_count);
  hako_write_kv(fd, "shim_usable_size_lookup_failure_count", usable_size_lookup_failure_count);
  hako_write_kv(fd, "shim_tracking_bypassed_count", tracking_bypassed_count);
  hako_write_kv(fd, "shim_claim_mainline_mode_enabled", claim_mainline_mode_enabled);
  hako_write_kv(fd, "shim_assume_owned_mode_enabled", assume_owned_mode_enabled);
  hako_write_kv(fd, "shim_assume_owned_free_count", assume_owned_free_count);
  hako_write_kv(fd, "shim_assume_owned_realloc_count", assume_owned_realloc_count);
  close(fd);
}
"""
