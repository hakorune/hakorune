/* Private invocation-owned harness-log proof; no compiler child is started. */
#define _GNU_SOURCE
#if defined(_WIN32)
#define _CRT_SECURE_NO_WARNINGS
#endif
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(_WIN32)
#include <direct.h>
#include <fcntl.h>
#include <io.h>
#include <sys/stat.h>
#include <windows.h>
#define access _access
#define F_OK 0
#define rmdir _rmdir
#else
#include <unistd.h>
#endif

static const char* test_tmp_dir;
static int force_eexist;
static int force_close_failure;

static const char* hako_llvmc_tmp_dir_fallback(void) {
  return test_tmp_dir;
}

#if defined(_WIN32)
static int test_mktemp_s(char* path, size_t length) {
  return _mktemp_s(path, length);
}

static int test_open(const char* path) {
  if (force_eexist > 0) {
    force_eexist--;
    errno = EEXIST;
    return -1;
  }
  return _open(path, _O_CREAT | _O_EXCL | _O_WRONLY | _O_BINARY,
               _S_IREAD | _S_IWRITE);
}

static int test_close_win(int fd) {
  int result = _close(fd);
  if (force_close_failure) {
    errno = EIO;
    return -1;
  }
  return result;
}

#define HAKO_LLVMC_HARNESS_LOG_MKTEMP_S(path, length) \
  test_mktemp_s(path, length)
#define HAKO_LLVMC_HARNESS_LOG_OPEN(path) test_open(path)
#define HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN(fd) test_close_win(fd)
#else
static int test_mkstemp(char* path) {
  if (force_eexist > 0) {
    force_eexist--;
    errno = EEXIST;
    return -1;
  }
  return mkstemp(path);
}

static int test_close(int fd) {
  int result = close(fd);
  if (force_close_failure) {
    errno = EIO;
    return -1;
  }
  return result;
}

#define HAKO_LLVMC_HARNESS_LOG_MKSTEMP(path) test_mkstemp(path)
#define HAKO_LLVMC_HARNESS_LOG_CLOSE(fd) test_close(fd)
#endif
#include "../shims/hako_llvmc_ffi_harness_log.inc"
#undef HAKO_LLVMC_HARNESS_LOG_MKSTEMP
#undef HAKO_LLVMC_HARNESS_LOG_CLOSE
#undef HAKO_LLVMC_HARNESS_LOG_MKTEMP_S
#undef HAKO_LLVMC_HARNESS_LOG_OPEN
#undef HAKO_LLVMC_HARNESS_LOG_CLOSE_WIN

static void write_text(const char* path, const char* text) {
  FILE* file = fopen(path, "wb");
  assert(file);
  assert(fputs(text, file) >= 0);
  assert(fclose(file) == 0);
}

int main(void) {
#if defined(_WIN32)
  char directory[HAKO_LLVMC_HARNESS_LOG_PATH_CAP];
  char temp_path[MAX_PATH];
  char temp_file[MAX_PATH];
  DWORD temp_len = GetTempPathA(sizeof(temp_path), temp_path);
  assert(temp_len > 0 && temp_len < sizeof(temp_path));
  assert(GetTempFileNameA(temp_path, "hlo", 0, temp_file) != 0);
  assert(DeleteFileA(temp_file));
  assert(_mkdir(temp_file) == 0);
  assert(strlen(temp_file) + 1 < sizeof(directory));
  strcpy(directory, temp_file);
#else
  char directory[] = "/tmp/hako-harness-log-test-XXXXXX";
  assert(mkdtemp(directory));
#endif
  hako_llvmc_harness_log first = {{0}, 0};
  hako_llvmc_harness_log second = {{0}, 0};
  hako_llvmc_harness_log collision = {{0}, 0};
  hako_llvmc_harness_log close_failed = {{0}, 0};
  test_tmp_dir = directory;

  assert(hako_llvmc_harness_log_build_template(
      first.path, sizeof(first.path)) == 0);
  assert(hako_llvmc_harness_log_reserve(&first) == 0);
  assert(first.owned && first.path[0] && access(first.path, F_OK) == 0);
  assert(hako_llvmc_harness_log_build_template(
      second.path, sizeof(second.path)) == 0);
  assert(hako_llvmc_harness_log_reserve(&second) == 0);
  assert(second.owned && strcmp(first.path, second.path) != 0);
  write_text(first.path, "first\n");
  write_text(second.path, "second\n");
  hako_llvmc_harness_log_cleanup(&first);
  assert(!first.owned && access(first.path, F_OK) != 0);
  assert(second.owned && access(second.path, F_OK) == 0);
  hako_llvmc_harness_log_cleanup(&second);
  assert(!second.owned && access(second.path, F_OK) != 0);

  force_eexist = 1;
  assert(hako_llvmc_harness_log_build_template(
      collision.path, sizeof(collision.path)) == 0);
#if defined(_WIN32)
  /* Windows retries an exclusive-create collision with a fresh candidate. */
  assert(hako_llvmc_harness_log_reserve(&collision) == 0);
  assert(collision.owned && access(collision.path, F_OK) == 0);
#else
  assert(hako_llvmc_harness_log_reserve(&collision) != 0);
  assert(!collision.owned);
#endif
  hako_llvmc_harness_log_cleanup(&collision);
  assert(!collision.owned);

  force_close_failure = 1;
  assert(hako_llvmc_harness_log_build_template(
      close_failed.path, sizeof(close_failed.path)) == 0);
  assert(hako_llvmc_harness_log_reserve(&close_failed) != 0);
  assert(close_failed.owned && access(close_failed.path, F_OK) == 0);
  hako_llvmc_harness_log_cleanup(&close_failed);
  assert(!close_failed.owned && access(close_failed.path, F_OK) != 0);
  force_close_failure = 0;

  test_tmp_dir = "/definitely/missing/hako-harness-log-dir";
  assert(hako_llvmc_harness_log_build_template(
      close_failed.path, sizeof(close_failed.path)) == 0);
  assert(hako_llvmc_harness_log_reserve(&close_failed) != 0);
  assert(!close_failed.owned);

  assert(rmdir(directory) == 0);
  puts("harness log ownership: PASS");
  return 0;
}
