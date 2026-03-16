// hako_diag_mem_shared_impl.inc
// Shared diagnostics + libc-backed memory foundation for C shims.

#if defined(_MSC_VER)
__declspec(thread) static const char* hako_tls_last_error = "OK";
#elif defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L)
static _Thread_local const char* hako_tls_last_error = "OK";
#else
static __thread const char* hako_tls_last_error = "OK";
#endif

struct hako_ctx;

const char* hako_last_error(struct hako_ctx* ctx) {
  (void)ctx;
  return hako_tls_last_error ? hako_tls_last_error : "OK";
}

void hako_set_last_error(const char* short_msg) {
  hako_tls_last_error = short_msg ? short_msg : "OK";
}

void* hako_mem_alloc(uint64_t size) {
  if (size == 0) size = 1;
  void* p = malloc((size_t)size);
  if (!p) {
    hako_set_last_error("OOM");
  }
  return p;
}

void* hako_mem_realloc(void* ptr, uint64_t new_size) {
  if (new_size == 0) new_size = 1;
  void* p = realloc(ptr, (size_t)new_size);
  if (!p) {
    hako_set_last_error("OOM");
  }
  return p;
}

void hako_mem_free(void* ptr) {
  if (ptr) free(ptr);
}
