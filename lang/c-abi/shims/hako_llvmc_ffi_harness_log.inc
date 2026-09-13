/* Invocation-owned diagnostic log storage for the named compatibility harness. */

#define HAKO_LLVMC_HARNESS_LOG_PATH_CAP 1024
#define HAKO_LLVMC_HARNESS_LOG_RETRIES 8

typedef struct hako_llvmc_harness_log {
  char path[HAKO_LLVMC_HARNESS_LOG_PATH_CAP];
  int owned;
} hako_llvmc_harness_log;

#ifndef HAKO_LLVMC_HARNESS_LOG_MKSTEMP
#define HAKO_LLVMC_HARNESS_LOG_MKSTEMP(path) mkstemp(path)
#endif
#ifndef HAKO_LLVMC_HARNESS_LOG_CLOSE
#define HAKO_LLVMC_HARNESS_LOG_CLOSE(fd) close(fd)
#endif
#ifndef HAKO_LLVMC_HARNESS_LOG_MKTEMP_S
#define HAKO_LLVMC_HARNESS_LOG_MKTEMP_S(path, length) _mktemp_s(path, length)
#endif
#ifndef HAKO_LLVMC_HARNESS_LOG_OPEN
#define HAKO_LLVMC_HARNESS_LOG_OPEN(path) \
  _open(path, _O_CREAT | _O_EXCL | _O_WRONLY | _O_BINARY, _S_IREAD | _S_IWRITE)
#endif
#ifndef HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN
#define HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN(fd) _close(fd)
#endif

static int hako_llvmc_harness_log_build_template(
    char* path, size_t path_len) {
  int n;
  if (!path || path_len == 0) return -1;
  n = snprintf(
      path,
      path_len,
      "%s/hako_llvmc_harness_compile_XXXXXX",
      hako_llvmc_tmp_dir_fallback());
  return (n <= 0 || (size_t)n >= path_len) ? -1 : 0;
}

static void hako_llvmc_harness_log_cleanup(
    hako_llvmc_harness_log* log) {
  if (!log || !log->owned) return;
  (void)remove(log->path);
  log->owned = 0;
}

static int hako_llvmc_harness_log_reserve(
    hako_llvmc_harness_log* log) {
  int fd;
  if (!log || log->owned || !log->path[0]) return -1;
#if defined(_WIN32)
  int attempt;
  for (attempt = 0; attempt < HAKO_LLVMC_HARNESS_LOG_RETRIES; attempt++) {
    char candidate[HAKO_LLVMC_HARNESS_LOG_PATH_CAP];
    int n;
    if (hako_llvmc_harness_log_build_template(
            candidate, sizeof(candidate)) != 0 ||
        HAKO_LLVMC_HARNESS_LOG_MKTEMP_S(candidate, sizeof(candidate)) != 0) {
      return -1;
    }
    fd = HAKO_LLVMC_HARNESS_LOG_OPEN(candidate);
    if (fd >= 0) {
      n = snprintf(log->path, sizeof(log->path), "%s", candidate);
      if (n <= 0 || (size_t)n >= sizeof(log->path)) {
        (void)HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN(fd);
        (void)remove(candidate);
        return -1;
      }
      log->owned = 1;
      if (HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN(fd) != 0) return -1;
      return 0;
    }
    if (errno != EEXIST) return -1;
  }
  return -1;
#else
  fd = HAKO_LLVMC_HARNESS_LOG_MKSTEMP(log->path);
  if (fd < 0) return -1;
  log->owned = 1;
  if (HAKO_LLVMC_HARNESS_LOG_CLOSE(fd) != 0) return -1;
  return 0;
#endif
}
