/* Private invocation-owned harness-log proof; no compiler child is started. */
#define _GNU_SOURCE
#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static const char* test_tmp_dir;
static int force_eexist;
static int force_close_failure;

static const char* hako_llvmc_tmp_dir_fallback(void) {
  return test_tmp_dir;
}

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
#include "../shims/hako_llvmc_ffi_harness_log.inc"
#undef HAKO_LLVMC_HARNESS_LOG_MKSTEMP
#undef HAKO_LLVMC_HARNESS_LOG_CLOSE

static void write_text(const char* path, const char* text) {
  FILE* file = fopen(path, "wb");
  assert(file);
  assert(fputs(text, file) >= 0);
  assert(fclose(file) == 0);
}

int main(void) {
  char directory[] = "/tmp/hako-harness-log-test-XXXXXX";
  hako_llvmc_harness_log first = {{0}, 0};
  hako_llvmc_harness_log second = {{0}, 0};
  hako_llvmc_harness_log collision = {{0}, 0};
  hako_llvmc_harness_log close_failed = {{0}, 0};
  assert(mkdtemp(directory));
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
  assert(hako_llvmc_harness_log_reserve(&collision) != 0);
  assert(!collision.owned);
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
