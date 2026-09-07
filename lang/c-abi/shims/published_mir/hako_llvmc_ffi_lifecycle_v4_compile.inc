/* One parsed document, one target session, one atomic object publication. */
#if !defined(_WIN32)
#include <spawn.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <errno.h>
extern char** environ;

static int lv4_preamble_matches(const char* path, const hako_lts_session* session) {
  FILE* f = fopen(path, "rb");
  char expected[2048], actual[2048];
  int n = snprintf(expected, sizeof(expected), "target triple = \"%s\"\ntarget datalayout = \"%s\"\n",
      session->triple, session->data_layout);
  int ok = 0;
  if (!f) return 0;
  if (n > 0 && (size_t)n < sizeof(expected) && fread(actual, 1, (size_t)n, f) == (size_t)n)
    ok = memcmp(actual, expected, (size_t)n) == 0;
  if (fclose(f) != 0) ok = 0;
  return ok;
}

static int lv4_llc(const char* input, const char* output, const hako_lts_session* session) {
  char triple[256];
  pid_t child;
  int status, rc;
  int n = snprintf(triple, sizeof(triple), "-mtriple=%s", session->triple);
  if (n < 0 || (size_t)n >= sizeof(triple)) return 0;
  char* argv[] = {"llc-18", "-filetype=obj", triple, "-mcpu=generic", "-O0",
      "-relocation-model=pic", "-o", (char*)output, (char*)input, NULL};
  if (posix_spawnp(&child, argv[0], NULL, NULL, argv, environ) != 0) return 0;
  do { rc = waitpid(child, &status, 0); } while (rc < 0 && errno == EINTR);
  return rc == child && WIFEXITED(status) && WEXITSTATUS(status) == 0;
}
#endif

__attribute__((visibility("default")))
int hako_llvmc_compile_published_lifecycle_physical_v4(
    const char* json_in, const hako_llvmc_lifecycle_target_session_v1* abi,
    const char* obj_out, char** error) {
#if defined(_WIN32)
  return set_err_owned(error, "[freeze:contract][published-lifecycle-v4/platform]");
#else
  yyjson_doc* doc = NULL;
  hako_lv4_input input = {0};
  hako_lts_session session = {0};
  char *ll = NULL, *object = NULL;
  FILE* file = NULL;
  int fd = -1, rc = -1, ll_owned = 0, object_owned = 0;
  struct stat st;
  const char* failure = "input";
  if (!json_in || !json_in[0] || !obj_out || !obj_out[0]) goto done;
  doc = yyjson_read_file(json_in, 0, NULL, NULL);
  if (!doc) { failure = "json"; goto done; }
  if (hako_llvmc_validate_published_lifecycle_physical_v1_doc(yyjson_doc_get_root(doc), error) != 0 ||
      lv4_admit(yyjson_doc_get_root(doc), &input, error) != 0 ||
      hako_lts_open(abi, &session, error) != 0) {
    failure = NULL; goto done;
  }
  failure = "target";
  if (strcmp(session.triple, "x86_64-unknown-linux-gnu") != 0) goto done;
  failure = "temporary-file";
  size_t len = strlen(obj_out) + 32;
  ll = malloc(len); object = malloc(len);
  if (!ll || !object) goto done;
  snprintf(ll, len, "%s.ll.XXXXXX", obj_out);
  snprintf(object, len, "%s.obj.XXXXXX", obj_out);
  fd = mkstemp(ll);
  if (fd < 0) goto done;
  ll_owned = 1;
  file = fdopen(fd, "wb");
  if (!file) { close(fd); fd = -1; goto done; }
  fd = -1;
  int emitted = lv4_emit(file, &input, abi, &session);
  if (fflush(file) != 0) emitted = 0;
  if (fclose(file) != 0) emitted = 0;
  file = NULL;
  failure = "module-write";
  if (!emitted || !lv4_preamble_matches(ll, &session)) goto done;
  fd = mkstemp(object);
  if (fd < 0) goto done;
  object_owned = 1;
  close(fd); fd = -1;
  failure = "llc";
  if (!lv4_llc(ll, object, &session) || stat(object, &st) != 0 || st.st_size <= 0) goto done;
  failure = "publish";
  if (rename(object, obj_out) != 0) goto done;
  object_owned = 0;
  rc = 0;
done:
  if (file) fclose(file);
  if (fd >= 0) close(fd);
  if (ll) { if (ll_owned) unlink(ll); free(ll); }
  if (object) { if (object_owned) unlink(object); free(object); }
  hako_lts_close(&session);
  if (doc) yyjson_doc_free(doc);
  if (rc != 0 && failure) {
    char message[128];
    snprintf(message, sizeof(message), "[freeze:contract][published-lifecycle-v4/%s]", failure);
    return set_err_owned(error, message);
  }
  return rc;
#endif
}
