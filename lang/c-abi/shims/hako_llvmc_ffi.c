// hako_llvmc_ffi.c — Minimal FFI bridge that forwards to hako_aot.c
// Exports functions that hako_aot.c dlopens when HAKO_AOT_USE_FFI=1.
// Phase 21.2 introduced a guarded "pure C-API" toggle (HAKO_CAPI_PURE=1).
// Phase 29ck now uses that pure subset as the first boundary compile step for
// supported seeds, while unsupported shapes in the pure-first lane replay the
// explicit `--driver harness` keep lane directly from this shim.
// The default export surface still presents as a thin hako_aot forwarder.

#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#if !defined(_WIN32)
#include <unistd.h>
#endif

// hako_aot.h provides hako_aot_compile_json / hako_aot_link_obj
#include "../include/hako_aot.h"
#include "hako_json_v1.h"
#include "yyjson.h"
#if !defined(_WIN32)
#include <dlfcn.h>
#endif

static int capi_pure_enabled(void) {
  const char* v = getenv("HAKO_CAPI_PURE");
  return (v && v[0] == '1');
}

static int set_err_owned(char** err_out, const char* msg) {
  if (!err_out) return -1;
  if (!msg) { *err_out = NULL; return -1; }
  size_t n = strlen(msg);
  char* p = (char*)malloc(n + 1);
  if (!p) { *err_out = NULL; return -1; }
  memcpy(p, msg, n + 1);
  *err_out = p;
  return -1;
}

static void hako_llvmc_set_env_value(const char* key, const char* value) {
#if defined(_WIN32)
  _putenv_s(key, value ? value : "");
#else
  if (value) {
    setenv(key, value, 1);
  } else {
    unsetenv(key);
  }
#endif
}

static const char* hako_llvmc_tmp_dir_fallback(void) {
  const char* tmp = getenv("TMPDIR");
  return (tmp && tmp[0]) ? tmp : "/tmp";
}

static int hako_llvmc_file_exists(const char* path) {
  FILE* f;
  if (!path || !path[0]) return 0;
  f = fopen(path, "rb");
  if (!f) return 0;
  fclose(f);
  return 1;
}

static char* hako_llvmc_read_first_line_owned(const char* path) {
  FILE* f = fopen(path, "rb");
  char buf[512];
  size_t n;
  char* out;
  if (!f) return NULL;
  if (!fgets(buf, sizeof(buf), f)) {
    fclose(f);
    return NULL;
  }
  fclose(f);
  n = strcspn(buf, "\r\n");
  buf[n] = '\0';
  if (!buf[0]) return NULL;
  out = (char*)malloc(n + 1);
  if (!out) return NULL;
  memcpy(out, buf, n + 1);
  return out;
}

static int hako_llvmc_build_harness_log_path(char* log_path, size_t log_path_len) {
  int n = snprintf(
      log_path,
      log_path_len,
      "%s/hako_llvmc_harness_compile_%ld.log",
      hako_llvmc_tmp_dir_fallback(),
      (long)getpid());
  return (n <= 0 || (size_t)n >= log_path_len) ? -1 : 0;
}

static int hako_llvmc_ascii_strlen(const char* s, long long* out_len) {
  size_t n = 0;
  if (!s) return 0;
  while (s[n]) {
    if (((unsigned char)s[n]) & 0x80) {
      return 0;
    }
    n++;
  }
  if (out_len) {
    *out_len = (long long)n;
  }
  return 1;
}

static int hako_llvmc_ascii_index_of(const char* haystack, const char* needle, long long* out_idx) {
  size_t hlen = 0;
  size_t nlen = 0;
  size_t i = 0;
  if (!hako_llvmc_ascii_strlen(haystack, (long long*)&hlen)) return 0;
  if (!hako_llvmc_ascii_strlen(needle, (long long*)&nlen)) return 0;
  if (nlen == 0) {
    if (out_idx) *out_idx = 0;
    return 1;
  }
  if (nlen > hlen) {
    if (out_idx) *out_idx = -1;
    return 1;
  }
  for (i = 0; i + nlen <= hlen; i++) {
    if (memcmp(haystack + i, needle, nlen) == 0) {
      if (out_idx) *out_idx = (long long)i;
      return 1;
    }
  }
  if (out_idx) *out_idx = -1;
  return 1;
}

static int compile_json_compat_harness_keep(const char* json_in, const char* obj_out, char** err_out) {
  const char* llvmc = getenv("NYASH_NY_LLVM_COMPILER");
  char log_path[1024];
  char cmd[4096];
  char* first_line;
  int n;
  int rc;
  if (!llvmc || !*llvmc) llvmc = "target/release/ny-llvmc";
  if (!hako_llvmc_file_exists(llvmc)) {
    hako_set_last_error("NOT_FOUND");
    return set_err_owned(err_out, "ny-llvmc not found (NYASH_NY_LLVM_COMPILER)");
  }
  if (hako_llvmc_build_harness_log_path(log_path, sizeof(log_path)) != 0) {
    hako_set_last_error("FAILED");
    return set_err_owned(err_out, "hako_llvmc harness log path too long");
  }
  remove(log_path);
  n = snprintf(
      cmd,
      sizeof(cmd),
      "\"%s\" --driver harness --in \"%s\" --emit obj --out \"%s\" 2> \"%s\"",
      llvmc,
      json_in,
      obj_out,
      log_path);
  if (n <= 0 || (size_t)n >= sizeof(cmd)) {
    hako_set_last_error("FAILED");
    return set_err_owned(err_out, "hako_llvmc harness compile command too long");
  }
  remove(obj_out);
  rc = system(cmd);
  if (rc == 0 && hako_llvmc_file_exists(obj_out)) {
    remove(log_path);
    return 0;
  }
  hako_set_last_error("FAILED");
  first_line = hako_llvmc_read_first_line_owned(log_path);
  remove(log_path);
  if (first_line) {
    if (err_out) {
      *err_out = first_line;
    } else {
      free(first_line);
    }
    return -1;
  }
  if (rc == 0) {
    return set_err_owned(err_out, "ny-llvmc harness compile finished without object");
  }
  return set_err_owned(err_out, "ny-llvmc harness compile failed");
}

static int hako_llvmc_forward_compile_to_aot_without_ffi(
    const char* json_in,
    const char* obj_out,
    char** err_out) {
  const char* prev_raw = getenv("HAKO_AOT_USE_FFI");
  char prev_buf[64];
  int had_prev = prev_raw && prev_raw[0];
  if (had_prev) {
    snprintf(prev_buf, sizeof(prev_buf), "%s", prev_raw);
  }
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", "0");
  int rc = hako_aot_compile_json(json_in, obj_out, err_out);
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", had_prev ? prev_buf : NULL);
  return rc;
}

static int hako_llvmc_forward_link_to_aot_without_ffi(
    const char* obj_in,
    const char* exe_out,
    const char* extra_ldflags,
    char** err_out) {
  const char* prev_raw = getenv("HAKO_AOT_USE_FFI");
  char prev_buf[64];
  int had_prev = prev_raw && prev_raw[0];
  if (had_prev) {
    snprintf(prev_buf, sizeof(prev_buf), "%s", prev_raw);
  }
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", "0");
  int rc = hako_aot_link_obj(obj_in, exe_out, extra_ldflags, err_out);
  hako_llvmc_set_env_value("HAKO_AOT_USE_FFI", had_prev ? prev_buf : NULL);
  return rc;
}

static int forward_compile_json_to_aot(const char* json_in, const char* obj_out, char** err_out) {
  return hako_llvmc_forward_compile_to_aot_without_ffi(json_in, obj_out, err_out);
}

static int forward_link_obj_to_aot(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return hako_llvmc_forward_link_to_aot_without_ffi(obj_in, exe_out, extra_ldflags, err_out);
}

static int compile_json_compat_pure(const char* json_in, const char* obj_out, char** err_out);

// Exported symbols expected by hako_aot.c when loading libhako_llvmc_ffi.so
// Signature must match: int (*)(const char*, const char*, char**)
__attribute__((visibility("default")))
int hako_llvmc_compile_json(const char* json_in, const char* obj_out, char** err_out) {
  if (!capi_pure_enabled()) {
    return forward_compile_json_to_aot(json_in, obj_out, err_out);
  }
  return compile_json_compat_pure(json_in, obj_out, err_out);
}

static int compile_json_compat_pure(const char* json_in, const char* obj_out, char** err_out) {
  // Phase 21.2: validate v1 JSON, try generic pure lowering (CFG/phi),
  // then fall back to a few pattern lowers, and finally to AOT helper.
  char* verr = NULL;
  if (hako_json_v1_validate_file(json_in, &verr) != 0) {
    return set_err_owned(err_out, verr ? verr : "invalid v1 json");
  }

    // --- Generic CFG/PHI lowering (minimal i64 subset) ---
    // Supported ops: const/compare/branch/jump/ret/phi, mir_call (Array/Map minimal, Global print)
    do {
      yyjson_read_err rerr_g; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_g);
      if (!d) break;
      yyjson_val* root = yyjson_doc_get_root(d);
      yyjson_val* fns = yyjson_obj_get(root, "functions");
      yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
      yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
      if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) >= 1)) { yyjson_doc_free(d); break; }

      enum { T_NONE=0, T_I64=1, T_I1=2 };
      struct { long long reg; long long val; } consts[1024]; size_t consts_n = 0;
      struct { long long reg; int ty; } types[2048]; size_t types_n = 0;
      struct { long long reg; int kind; } scan_origin[2048]; size_t scan_origin_n = 0;
      // Track simple origin kinds for selective unbox at ret
      enum { ORG_NONE=0, ORG_MAP_GET=1, ORG_MAP_BIRTH=2, ORG_ARRAY_BIRTH=3 };
      struct { long long reg; int kind; } origin[2048]; size_t origin_n = 0;
      auto int get_scan_origin(long long r){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r) return scan_origin[i].kind; } return ORG_NONE; }
      auto void set_scan_origin(long long r,int k){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r){ scan_origin[i].kind=k; return;} } if(scan_origin_n<2048){ scan_origin[scan_origin_n].reg=r; scan_origin[scan_origin_n].kind=k; scan_origin_n++; } }
      auto int get_origin(long long r){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r) return origin[i].kind; } return ORG_NONE; }
      auto void set_origin(long long r,int k){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r){ origin[i].kind=k; return;} } if(origin_n<2048){ origin[origin_n].reg=r; origin[origin_n].kind=k; origin_n++; } }
      // Dynamic fallback (by-name) method strings
      struct { char name[64]; int len; } mnames[64]; int mnames_n = 0;
      auto int find_mname(const char* s){ for(int i=0;i<mnames_n;i++){ if (strcmp(mnames[i].name,s)==0) return i; } return -1; }
      auto int add_mname(const char* s){ int idx=find_mname(s); if (idx>=0) return idx; if (mnames_n<64){ strncpy(mnames[mnames_n].name, s, 63); mnames[mnames_n].name[63]='\0'; mnames[mnames_n].len=strlen(mnames[mnames_n].name); return mnames_n++; } return -1; }
      struct Incoming { long long pred; long long val_reg; };
      struct PhiRec { long long dst; struct Incoming in[16]; int in_n; };
      struct BlockPhi { long long bid; struct PhiRec recs[16]; int rec_n; } phis[512]; int phi_n = 0;

      // small helpers
      #define ARR_LEN(a) ((int)(sizeof(a)/sizeof((a)[0])))
      auto int get_type(long long r) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r) return types[i].ty; } return T_NONE; }
      auto void set_type(long long r, int t) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r){ types[i].ty=t; return; } } if (types_n<2048){ types[types_n].reg=r; types[types_n].ty=t; types_n++; } }
      auto int has_const(long long r, long long* out){ for (size_t i=0;i<consts_n;i++){ if (consts[i].reg==r){ if(out) *out=consts[i].val; return 1; } } return 0; }
      auto void put_const(long long r, long long v){ if (consts_n<1024){ consts[consts_n].reg=r; consts[consts_n].val=v; consts_n++; } }
      auto struct BlockPhi* ensure_phi_block(long long bid){ for (int i=0;i<phi_n;i++){ if (phis[i].bid==bid) return &phis[i]; } if (phi_n<512){ phis[phi_n].bid=bid; phis[phi_n].rec_n=0; return &phis[phi_n++]; } return NULL; }
      auto long long read_int(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? (long long)yyjson_get_sint(v) : 0; }
      auto const char* read_str(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? yyjson_get_str(v) : NULL; }

      // Pre-scan: consts + phis + needed method decls
      int need_map_birth=0, need_map_set=0, need_map_size=0, need_map_get=0, need_map_has=0;
      int need_arr_birth=0, need_arr_push=0, need_arr_len=0, need_arr_set=0, need_arr_get=0, need_arr_has=0;
      int need_printf=0;
      size_t blen = yyjson_arr_size(blocks);
      for (size_t bi=0; bi<blen; bi++) {
        yyjson_val* b = yyjson_arr_get(blocks, bi);
        long long bid = read_int(b, "id");
        yyjson_val* insts = yyjson_obj_get(b, "instructions"); if (!insts || !yyjson_is_arr(insts)) continue;
        size_t ilen = yyjson_arr_size(insts);
        for (size_t ii=0; ii<ilen; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op"); if (!op) continue;
          if (strcmp(op, "const")==0) {
            long long dst = read_int(ins, "dst");
            yyjson_val* vobj = yyjson_obj_get(ins, "value"); long long v = vobj? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
            put_const(dst, v); set_type(dst, T_I64);
          } else if (strcmp(op, "phi")==0) {
            struct BlockPhi* pb = ensure_phi_block(bid); if (!pb) { yyjson_doc_free(d); goto GEN_END; }
            struct PhiRec pr; pr.dst = read_int(ins, "dst"); pr.in_n=0;
            yyjson_val* vals = yyjson_obj_get(ins, "values"); if (!vals) vals = yyjson_obj_get(ins, "incoming");
            if (vals && yyjson_is_arr(vals)) {
              size_t vn = yyjson_arr_size(vals);
              for (size_t vi=0; vi<vn && pr.in_n<16; vi++) {
                yyjson_val* ent = yyjson_arr_get(vals, vi);
                long long pred = read_int(ent, "pred"); if (!pred) pred = read_int(ent, "block");
                long long vin = read_int(ent, "value");
                pr.in[pr.in_n].pred = pred; pr.in[pr.in_n].val_reg = vin; pr.in_n++;
              }
            } else {
              long long pred = read_int(ins, "pred"); long long vin = read_int(ins, "value");
              if (vin) { pr.in[pr.in_n].pred=pred; pr.in[pr.in_n].val_reg=vin; pr.in_n++; }
            }
            if (pb->rec_n < 16) { pb->recs[pb->rec_n++] = pr; set_type(pr.dst, T_I64); }
          } else if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal? read_str(cal, "type") : NULL;
            const char* bname = cal? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
            const char* mname = cal? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            if (ctype && strcmp(ctype, "Constructor")==0) {
              long long dst = read_int(ins, "dst");
              if (bname && strcmp(bname, "MapBox")==0) { need_map_birth=1; if (dst) set_scan_origin(dst, ORG_MAP_BIRTH); }
              if (bname && strcmp(bname, "ArrayBox")==0) { need_arr_birth=1; if (dst) set_scan_origin(dst, ORG_ARRAY_BIRTH); }
            } else if (ctype && strcmp(ctype, "Method")==0) {
              long long recv = cal ? read_int(cal, "receiver") : 0;
              int scan_org = recv ? get_scan_origin(recv) : ORG_NONE;
              if (bname && strcmp(bname, "MapBox")==0) {
                if (mname) {
                  if (strcmp(mname, "set")==0) need_map_set=1; else if (strcmp(mname, "size")==0||strcmp(mname, "len")==0) need_map_size=1;
                  else if (strcmp(mname, "get")==0) need_map_get=1; else if (strcmp(mname, "has")==0) need_map_has=1;
                }
              } else if (bname && strcmp(bname, "ArrayBox")==0) {
                if (mname) {
                  if (strcmp(mname, "push")==0) need_arr_push=1; else if (strcmp(mname, "len")==0||strcmp(mname, "length")==0||strcmp(mname, "size")==0) need_arr_len=1;
                  else if (strcmp(mname, "set")==0) need_arr_set=1; else if (strcmp(mname, "get")==0) need_arr_get=1;
                }
              } else if (bname && strcmp(bname, "RuntimeDataBox")==0) {
                if (scan_org == ORG_ARRAY_BIRTH && mname && strcmp(mname, "get")==0) need_arr_get=1;
                if (scan_org == ORG_ARRAY_BIRTH && mname && strcmp(mname, "push")==0) need_arr_push=1;
                if (scan_org == ORG_ARRAY_BIRTH && mname && (strcmp(mname, "len")==0||strcmp(mname, "length")==0||strcmp(mname, "size")==0)) need_arr_len=1;
                if (scan_org == ORG_ARRAY_BIRTH && mname && strcmp(mname, "has")==0) need_arr_has=1;
                if (scan_org == ORG_MAP_BIRTH && mname && strcmp(mname, "get")==0) need_map_get=1;
                if (scan_org == ORG_MAP_BIRTH && mname && (strcmp(mname, "len")==0||strcmp(mname, "length")==0||strcmp(mname, "size")==0)) need_map_size=1;
                if (scan_org == ORG_MAP_BIRTH && mname && strcmp(mname, "has")==0) need_map_has=1;
              }
            } else if (ctype && strcmp(ctype, "Global")==0) {
              if (mname && strcmp(mname, "print")==0) need_printf=1;
            }
          }
        }
      }

      // IR temp file
      char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_gen_%d.ll", "/tmp", (int)getpid());
      FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(d); break; }
      fprintf(f, "; nyash pure IR (generic)\n");
      fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
      if (need_map_birth) fprintf(f, "declare i64 @\"nyash.map.birth_h\"()\n");
      if (need_map_set)   fprintf(f, "declare i64 @\"nyash.map.set_h\"(i64, i64, i64)\n");
      if (need_map_get)   fprintf(f, "declare i64 @\"nyash.map.get_h\"(i64, i64)\n");
      if (need_map_has)   fprintf(f, "declare i64 @\"nyash.map.has_h\"(i64, i64)\n");
      if (need_map_size)  fprintf(f, "declare i64 @\"nyash.map.size_h\"(i64)\n");
      if (need_arr_birth) fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
      if (need_arr_push)  fprintf(f, "declare i64 @\"nyash.array.push_h\"(i64, i64)\n");
      if (need_arr_len)   fprintf(f, "declare i64 @\"nyash.array.len_h\"(i64)\n");
      if (need_arr_set)   fprintf(f, "declare i64 @\"nyash.array.set_h\"(i64, i64, i64)\n");
      if (need_arr_get)   fprintf(f, "declare i64 @\"nyash.array.get_h\"(i64, i64)\n");
      if (need_arr_has)   fprintf(f, "declare i64 @\"nyash.array.has_hi\"(i64, i64)\n");
      if (need_printf)    fprintf(f, "@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%%ld\\0A\\00\", align 1\n");
      // Unboxer (declare opportunistically; low cost)
      fprintf(f, "declare i64 @\"nyash.integer.get_h\"(i64)\n");
      if (need_printf)    fprintf(f, "declare i32 @printf(ptr, ...)\n");
      fprintf(f, "\n");
      // Dynamic fallback invoke decl (optional utilization)
      fprintf(f, "declare i64 @\"nyash.plugin.invoke_by_name_i64\"(i64, i8*, i64, i64, i64)\n");
      // Emit method name constants collected for fallback
      for (int si=0; si<mnames_n; si++) {
        // Note: method names are assumed ASCII and safe here
        fprintf(f, "@.hako_mname_%d = private unnamed_addr constant [%d x i8] c\"%s\\00\", align 1\n", si, mnames[si].len+1, mnames[si].name);
      }
      // Unboxer (declare opportunistically; low cost)
      fprintf(f, "define i64 @ny_main() {\n");
      // Emit blocks
      #define EMIT(...) do { fprintf(f, __VA_ARGS__); } while(0)
      auto void emit_block_label(long long bid){ EMIT("bb%lld:\n", bid); }
      auto void emit_phi(long long dst, struct Incoming* in, int in_n){ EMIT("  %%r%lld = phi i64 ", dst); for (int i=0;i<in_n;i++){ long long cv; int hc=has_const(in[i].val_reg,&cv); EMIT("[ %s%lld%s, %%bb%lld ]%s", hc?"":"%r", hc?cv:in[i].val_reg, hc?"":"", in[i].pred, (i+1<in_n)?", ":""); } EMIT("\n"); }
      auto void emit_icmp(long long dst, const char* pred, long long lhs_reg, long long rhs_reg){ long long vL,vR; int lc=has_const(lhs_reg,&vL), rc=has_const(rhs_reg,&vR); EMIT("  %%r%lld = icmp %s i64 %s%lld%s, %s%lld%s\n", dst, pred, lc?"":"%r", lc?vL:lhs_reg, lc?"":"", rc?"":"%r", rc?vR:rhs_reg, rc?"":""); set_type(dst,T_I1); }
      auto void emit_branch(long long cond_reg, long long then_id, long long else_id){ int ty=get_type(cond_reg); if (ty==T_I1) EMIT("  br i1 %%r%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id); else { long long cv; if (has_const(cond_reg,&cv)) { EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", cond_reg, cv); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} else { EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", cond_reg, cond_reg); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} } }
      auto void emit_jump(long long target){ EMIT("  br label %%bb%lld\n", target); }
      auto void emit_ret(long long reg){ long long cv; if (has_const(reg,&cv)) EMIT("  ret i64 %lld\n", cv); else EMIT("  ret i64 %%r%lld\n", reg); }
      auto void emit_call_assign(long long dst, const char* sym, const char* args){ EMIT("  %%r%lld = call i64 @\"%s\"(%s)\n", dst, sym, args); set_type(dst, T_I64); }
      auto void emit_call_noret(const char* sym, const char* args){ EMIT("  %%_ = call i64 @\"%s\"(%s)\n", sym, args); }

      for (size_t bi=0; bi<blen; bi++) {
        yyjson_val* b = yyjson_arr_get(blocks, bi);
        long long bid = read_int(b, "id");
        emit_block_label(bid);
        // phi first
        for (int pi=0; pi<phi_n; pi++) if (phis[pi].bid==bid){ for (int r=0;r<phis[pi].rec_n;r++){ emit_phi(phis[pi].recs[r].dst, phis[pi].recs[r].in, phis[pi].recs[r].in_n); } }
        yyjson_val* insts = yyjson_obj_get(b, "instructions"); if (!insts || !yyjson_is_arr(insts)) continue;
        size_t ilen = yyjson_arr_size(insts);
        for (size_t ii=0; ii<ilen; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op"); if (!op) { yyjson_doc_free(d); goto GEN_ABORT; }
          if (strcmp(op, "phi")==0) continue; // already emitted
          if (strcmp(op, "const")==0) continue; // inlined via const map
          if (strcmp(op, "compare")==0) {
            const char* pred = read_str(ins, "cmp"); if (!pred) pred = read_str(ins, "operation");
            const char* p2 = NULL; if (pred){ if (!strcmp(pred,"Lt")||!strcmp(pred,"lt")||!strcmp(pred,"<")) p2="slt"; else if (!strcmp(pred,"Le")||!strcmp(pred,"le")||!strcmp(pred,"<=")) p2="sle"; else if (!strcmp(pred,"Eq")||!strcmp(pred,"eq")||!strcmp(pred,"==")) p2="eq"; else if (!strcmp(pred,"Ne")||!strcmp(pred,"ne")||!strcmp(pred,"!=")) p2="ne"; else if (!strcmp(pred,"Ge")||!strcmp(pred,"ge")||!strcmp(pred,">=")) p2="sge"; else if (!strcmp(pred,"Gt")||!strcmp(pred,"gt")||!strcmp(pred,">")) p2="sgt"; }
            if (!p2) { yyjson_doc_free(d); goto GEN_ABORT; }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); if (!lhs) lhs=read_int(ins, "left"); long long rhs = read_int(ins, "rhs"); if (!rhs) rhs=read_int(ins, "right"); if (!lhs && !rhs){ yyjson_doc_free(d); goto GEN_ABORT; }
            // emit icmp
            long long vL,vR; int lc=has_const(lhs,&vL), rc2=has_const(rhs,&vR);
            EMIT("  %%r%lld = icmp %s i64 %s%lld%s, %s%lld%s\n", dst, p2, lc?"":"%r", lc?vL:lhs, lc?"":"", rc2?"":"%r", rc2?vR:rhs, rc2?"":""); set_type(dst, T_I1);
            continue;
          }
          if (strcmp(op, "branch")==0) { long long cond=read_int(ins, "cond"); long long th=read_int(ins,"then"); long long el=read_int(ins,"else"); if(!el) el=read_int(ins,"else_id"); int ty=get_type(cond); if (ty==T_I1) EMIT("  br i1 %%r%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el); else { long long cv; if (has_const(cond,&cv)) { EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", cond, cv); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el);} else { EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", cond, cond); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el);} } continue; }
          if (strcmp(op, "jump")==0) { long long tgt=read_int(ins, "target"); EMIT("  br label %%bb%lld\n", tgt); continue; }
          if (strcmp(op, "ret")==0) {
            long long v=read_int(ins, "value"); long long cv;
            if (has_const(v,&cv)) { EMIT("  ret i64 %lld\n", cv); }
            else {
              int org = get_origin(v);
              if (org == ORG_MAP_GET) {
                // Auto-unbox map.get result as Integer (minimal MVP)
                EMIT("  %%map_get_unbox_%lld = call i64 @\"nyash.integer.get_h\"(i64 %%r%lld)\n", v, v);
                EMIT("  ret i64 %%map_get_unbox_%lld\n", v);
              } else {
                EMIT("  ret i64 %%r%lld\n", v);
              }
            }
            continue;
          }
          if (strcmp(op, "binop")==0) {
            const char* k = read_str(ins, "op_kind"); if (!k) k = read_str(ins, "operation");
            const char* irop = NULL;
            if (k) {
              if (!strcmp(k, "Add") || !strcmp(k, "+")) irop = "add";
              else if (!strcmp(k, "Sub") || !strcmp(k, "-")) irop = "sub";
              else if (!strcmp(k, "Mul") || !strcmp(k, "*")) irop = "mul";
              else if (!strcmp(k, "Div") || !strcmp(k, "/")) irop = "sdiv";
              else if (!strcmp(k, "Mod") || !strcmp(k, "%")) irop = "srem";
            }
            if (!irop) { yyjson_doc_free(d); goto GEN_ABORT; }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); long long rhs = read_int(ins, "rhs");
            long long vL, vR; int lc = has_const(lhs, &vL), rc2 = has_const(rhs, &vR);
            if (lc && rc2) {
              EMIT("  %%r%lld = %s i64 %lld, %lld\n", dst, irop, vL, vR);
            } else {
              EMIT("  %%r%lld = %s i64 %s%lld%s, %s%lld%s\n", dst, irop,
                   lc?"":"%r", lc?vL:lhs, lc?"":"",
                   rc2?"":"%r", rc2?vR:rhs, rc2?"":"");
            }
            set_type(dst, T_I64);
            continue;
          }
          if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call"); yyjson_val* cal = mc? yyjson_obj_get(mc, "callee") : NULL; const char* ctype = cal? read_str(cal, "type") : NULL; const char* bname = cal? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL; const char* mname = cal? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL; long long dst = read_int(ins, "dst"); long long recv = read_int(cal, "receiver"); yyjson_val* args = mc? yyjson_obj_get(mc, "args") : NULL; long long a0=0,a1=0; if (args && yyjson_is_arr(args)) { if (yyjson_arr_size(args)>=1) a0=(long long)yyjson_get_sint(yyjson_arr_get(args,0)); if (yyjson_arr_size(args)>=2) a1=(long long)yyjson_get_sint(yyjson_arr_get(args,1)); }
            char ab[256]; ab[0]='\0';
            auto void app(long long reg, int first){ long long cv; char tmp[64]; if (has_const(reg,&cv)) snprintf(tmp,sizeof(tmp),"i64 %lld", cv); else snprintf(tmp,sizeof(tmp),"i64 %%r%lld", reg); snprintf(ab + strlen(ab), sizeof(ab)-strlen(ab), "%s%s", first?"":", ", tmp); };
            if (ctype && !strcmp(ctype, "Constructor")) {
              if (bname && !strcmp(bname, "MapBox")) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst); set_type(dst, T_I64); set_origin(dst, ORG_MAP_BIRTH);} else { EMIT("  %%_ = call i64 @\"nyash.map.birth_h\"()\n"); } }
              else if (bname && !strcmp(bname, "ArrayBox")) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.array.birth_h\"()\n", dst); set_type(dst, T_I64); set_origin(dst, ORG_ARRAY_BIRTH);} else { EMIT("  %%_ = call i64 @\"nyash.array.birth_h\"()\n"); } }
              else { yyjson_doc_free(d); goto GEN_ABORT; }
            } else if (ctype && !strcmp(ctype, "Method")) {
              int recv_org = recv ? get_origin(recv) : ORG_NONE;
              int runtime_array_len = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_BIRTH;
              int runtime_array_get = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_BIRTH;
              int runtime_array_push = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_BIRTH;
              int runtime_array_has = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_BIRTH;
              int runtime_map_get = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_MAP_BIRTH;
              int runtime_map_size = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_MAP_BIRTH;
              int runtime_map_has = bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_MAP_BIRTH;
              if (recv) app(recv, 1);
              if (mname && !strcmp(mname, "set")) { if (a0) app(a0, ab[0]=='\0'); if (a1) app(a1, 0); if (bname && !strcmp(bname, "MapBox")) EMIT("  %%_ = call i64 @\"nyash.map.set_h\"(%s)\n", ab); else if (bname && !strcmp(bname, "ArrayBox")) EMIT("  %%_ = call i64 @\"nyash.array.set_h\"(%s)\n", ab); else { yyjson_doc_free(d); goto GEN_ABORT; } }
              else if (mname && !strcmp(mname, "get")) { if (a0) app(a0, ab[0]=='\0'); if ((bname && !strcmp(bname, "MapBox")) || runtime_map_get) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.map.get_h\"(%s)\n", dst, ab); set_type(dst, T_I64); set_origin(dst, ORG_MAP_GET);} else { EMIT("  %%_ = call i64 @\"nyash.map.get_h\"(%s)\n", ab);} } else if ((bname && !strcmp(bname, "ArrayBox")) || runtime_array_get) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.array.get_h\"(%s)\n", dst, ab); set_type(dst, T_I64);} else { EMIT("  %%_ = call i64 @\"nyash.array.get_h\"(%s)\n", ab);} } else { yyjson_doc_free(d); goto GEN_ABORT; } }
              else if (mname && (!strcmp(mname, "len")||!strcmp(mname, "length")||!strcmp(mname, "size"))) { if ((bname && !strcmp(bname, "MapBox")) || runtime_map_size) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.map.size_h\"(%s)\n", dst, ab); set_type(dst, T_I64);} else { EMIT("  %%_ = call i64 @\"nyash.map.size_h\"(%s)\n", ab);} } else if ((bname && !strcmp(bname, "ArrayBox")) || runtime_array_len) { if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.array.len_h\"(%s)\n", dst, ab); set_type(dst, T_I64);} else { EMIT("  %%_ = call i64 @\"nyash.array.len_h\"(%s)\n", ab);} } else { yyjson_doc_free(d); goto GEN_ABORT; } }
              else if (mname && !strcmp(mname, "push")) {
                if (a0) app(a0, ab[0]=='\0');
                if (runtime_array_push) {
                  if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.array.push_h\"(%s)\n", dst, ab); set_type(dst, T_I64); }
                  else { EMIT("  %%_ = call i64 @\"nyash.array.push_h\"(%s)\n", ab); }
                } else if (bname && !strcmp(bname, "ArrayBox")) EMIT("  %%_ = call i64 @\"nyash.array.push_h\"(%s)\n", ab);
                else { yyjson_doc_free(d); goto GEN_ABORT; }
              }
              else if (mname && !strcmp(mname, "has")) {
                if (a0) app(a0, ab[0]=='\0');
                if (runtime_array_has) {
                  if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.array.has_hi\"(%s)\n", dst, ab); set_type(dst, T_I64); }
                  else { EMIT("  %%_ = call i64 @\"nyash.array.has_hi\"(%s)\n", ab); }
                } else if ((bname && !strcmp(bname, "MapBox")) || runtime_map_has) {
                  if (dst) { EMIT("  %%r%lld = call i64 @\"nyash.map.has_h\"(%s)\n", dst, ab); set_type(dst, T_I64); }
                  else { EMIT("  %%_ = call i64 @\"nyash.map.has_h\"(%s)\n", ab); }
                } else { yyjson_doc_free(d); goto GEN_ABORT; }
              }
              else {
                // Dynamic fallback by name (dev only): invoke_by_name(recv, "method", argc, a0, a1)
                const char* fb = getenv("HAKO_CAPI_DYN_FALLBACK");
                if (fb && fb[0]=='1' && mname && recv) {
                  int idx = add_mname(mname);
                  if (idx >= 0) {
                    long long argc = 0; if (a0) argc++; if (a1) argc++;
                    long long ctmp;
                    char arg0[64]; arg0[0]='\0';
                    char arg1[64]; arg1[0]='\0';
                    if (a0) { if (has_const(a0,&ctmp)) snprintf(arg0,sizeof(arg0),"%lld", ctmp); else snprintf(arg0,sizeof(arg0),"%%r%lld", a0); } else { snprintf(arg0,sizeof(arg0),"0"); }
                    if (a1) { if (has_const(a1,&ctmp)) snprintf(arg1,sizeof(arg1),"%lld", ctmp); else snprintf(arg1,sizeof(arg1),"%%r%lld", a1); } else { snprintf(arg1,sizeof(arg1),"0"); }
                    // build method ptr IR and call by-name
                    EMIT("  %%r%lld = call i64 @\"nyash.plugin.invoke_by_name_i64\"(i64 %%r%lld, i8* getelementptr inbounds ([%d x i8], [%d x i8]* @.hako_mname_%d, i64 0, i64 0), i64 %lld, i64 %s, i64 %s)\n",
                         dst?dst:0, recv, mnames[idx].len+1, mnames[idx].len+1, idx, argc, arg0, arg1);
                    set_type(dst, T_I64);
                  } else { yyjson_doc_free(d); goto GEN_ABORT; }
                } else { yyjson_doc_free(d); goto GEN_ABORT; }
              }
            } else if (ctype && !strcmp(ctype, "Global")) {
              if (mname && !strcmp(mname, "print")) {
                long long print_arg = a0;
                long long cv = 0;
                if (!print_arg) { yyjson_doc_free(d); goto GEN_ABORT; }
                if (has_const(print_arg, &cv)) {
                  EMIT("  %%print_call_%lld = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %lld)\n",
                       print_arg, cv);
                } else {
                  EMIT("  %%print_call_%lld = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %%r%lld)\n",
                       print_arg, print_arg);
                }
              } else { yyjson_doc_free(d); goto GEN_ABORT; }
            } else { yyjson_doc_free(d); goto GEN_ABORT; }
            continue;
          }
          // any other op unsupported
          yyjson_doc_free(d); goto GEN_ABORT;
        }
      }
      fprintf(f, "}\n");
      fclose(f);

      // Optional: try LLVM TargetMachine emit (dlopen C-API) when HAKO_CAPI_TM=1
      int rc = -1;
#if !defined(_WIN32)
      {
        const char* tm = getenv("HAKO_CAPI_TM");
        if (tm && tm[0]=='1') {
          // Minimal dynamic loader for a subset of LLVM C-API
          typedef void* (*p_LLVMContextCreate)(void);
          typedef void  (*p_LLVMContextDispose)(void*);
          typedef void* (*p_LLVMCreateMemoryBufferWithContentsOfFile)(const char*, char**);
          typedef void  (*p_LLVMDisposeMemoryBuffer)(void*);
          typedef int   (*p_LLVMParseIRInContext)(void*, void*, void**, char**);
          typedef char* (*p_LLVMGetDefaultTargetTriple)(void);
          typedef void  (*p_LLVMDisposeMessage)(char*);
          typedef int   (*p_LLVMGetTargetFromTriple)(const char*, void**, char**);
          typedef void* (*p_LLVMCreateTargetMachine)(void*, const char*, const char*, const char*, int, int, int);
          typedef int   (*p_LLVMTargetMachineEmitToFile)(void*, void*, char*, int, char**);
          typedef void  (*p_LLVMDisposeTargetMachine)(void*);
          typedef void  (*p_LLVMDisposeModule)(void*);
          typedef void* (*p_LLVMModuleCreateWithNameInContext)(const char*, void*);
          // Target init (X86)
          typedef void (*p_LLVMInitializeX86TargetInfo)(void);
          typedef void (*p_LLVMInitializeX86Target)(void);
          typedef void (*p_LLVMInitializeX86TargetMC)(void);
          typedef void (*p_LLVMInitializeX86AsmPrinter)(void);

          const char* cand[] = { "libLLVM-18.so", "libLLVM.so.18", "libLLVM.so", NULL };
          void* h = NULL; for (int i=0;cand[i];i++){ h = dlopen(cand[i], RTLD_LAZY|RTLD_LOCAL); if (h) break; }
          if (h) {
            // Resolve required symbols
            p_LLVMContextCreate                 f_LLVMContextCreate                 = (p_LLVMContextCreate)dlsym(h, "LLVMContextCreate");
            p_LLVMContextDispose                f_LLVMContextDispose                = (p_LLVMContextDispose)dlsym(h, "LLVMContextDispose");
            p_LLVMCreateMemoryBufferWithContentsOfFile f_LLVMCreateMemoryBufferWithContentsOfFile = (p_LLVMCreateMemoryBufferWithContentsOfFile)dlsym(h, "LLVMCreateMemoryBufferWithContentsOfFile");
            p_LLVMDisposeMemoryBuffer           f_LLVMDisposeMemoryBuffer           = (p_LLVMDisposeMemoryBuffer)dlsym(h, "LLVMDisposeMemoryBuffer");
            p_LLVMParseIRInContext              f_LLVMParseIRInContext              = (p_LLVMParseIRInContext)dlsym(h, "LLVMParseIRInContext");
            p_LLVMGetDefaultTargetTriple        f_LLVMGetDefaultTargetTriple        = (p_LLVMGetDefaultTargetTriple)dlsym(h, "LLVMGetDefaultTargetTriple");
            p_LLVMDisposeMessage                f_LLVMDisposeMessage                = (p_LLVMDisposeMessage)dlsym(h, "LLVMDisposeMessage");
            p_LLVMGetTargetFromTriple           f_LLVMGetTargetFromTriple           = (p_LLVMGetTargetFromTriple)dlsym(h, "LLVMGetTargetFromTriple");
            p_LLVMCreateTargetMachine           f_LLVMCreateTargetMachine           = (p_LLVMCreateTargetMachine)dlsym(h, "LLVMCreateTargetMachine");
            p_LLVMTargetMachineEmitToFile       f_LLVMTargetMachineEmitToFile       = (p_LLVMTargetMachineEmitToFile)dlsym(h, "LLVMTargetMachineEmitToFile");
            p_LLVMDisposeTargetMachine          f_LLVMDisposeTargetMachine          = (p_LLVMDisposeTargetMachine)dlsym(h, "LLVMDisposeTargetMachine");
            p_LLVMDisposeModule                 f_LLVMDisposeModule                 = (p_LLVMDisposeModule)dlsym(h, "LLVMDisposeModule");
            p_LLVMModuleCreateWithNameInContext f_LLVMModuleCreateWithNameInContext = (p_LLVMModuleCreateWithNameInContext)dlsym(h, "LLVMModuleCreateWithNameInContext");
            // Target init
            p_LLVMInitializeX86TargetInfo       f_LLVMInitializeX86TargetInfo       = (p_LLVMInitializeX86TargetInfo)dlsym(h, "LLVMInitializeX86TargetInfo");
            p_LLVMInitializeX86Target           f_LLVMInitializeX86Target           = (p_LLVMInitializeX86Target)dlsym(h, "LLVMInitializeX86Target");
            p_LLVMInitializeX86TargetMC         f_LLVMInitializeX86TargetMC         = (p_LLVMInitializeX86TargetMC)dlsym(h, "LLVMInitializeX86TargetMC");
            p_LLVMInitializeX86AsmPrinter       f_LLVMInitializeX86AsmPrinter       = (p_LLVMInitializeX86AsmPrinter)dlsym(h, "LLVMInitializeX86AsmPrinter");

            int ok = f_LLVMContextCreate && f_LLVMCreateMemoryBufferWithContentsOfFile && f_LLVMParseIRInContext &&
                     f_LLVMGetDefaultTargetTriple && f_LLVMGetTargetFromTriple && f_LLVMCreateTargetMachine &&
                     f_LLVMTargetMachineEmitToFile && f_LLVMDisposeTargetMachine && f_LLVMDisposeMessage &&
                     f_LLVMDisposeMemoryBuffer && f_LLVMContextDispose && f_LLVMDisposeModule &&
                     f_LLVMInitializeX86TargetInfo && f_LLVMInitializeX86Target && f_LLVMInitializeX86TargetMC && f_LLVMInitializeX86AsmPrinter;
            if (ok) {
              // Init targets
              f_LLVMInitializeX86TargetInfo(); f_LLVMInitializeX86Target(); f_LLVMInitializeX86TargetMC(); f_LLVMInitializeX86AsmPrinter();
              void* ctx = f_LLVMContextCreate();
              char* emsg = NULL; void* buf = f_LLVMCreateMemoryBufferWithContentsOfFile(llpath, &emsg);
              if (!buf) { if (emsg) f_LLVMDisposeMessage(emsg); goto TM_END; }
              void* mod = NULL; if (f_LLVMParseIRInContext(ctx, buf, &mod, &emsg)) { if (emsg) f_LLVMDisposeMessage(emsg); f_LLVMDisposeMemoryBuffer(buf); goto TM_CTX; }
              char* triple = f_LLVMGetDefaultTargetTriple();
              void* tgt = NULL; if (f_LLVMGetTargetFromTriple(triple, &tgt, &emsg)) { if (emsg) f_LLVMDisposeMessage(emsg); goto TM_MOD; }
              // Opt level
              const char* ol = getenv("HAKO_LLVM_OPT_LEVEL"); if (!ol) ol = getenv("NYASH_LLVM_OPT_LEVEL"); if (!ol) ol = "0";
              int opt = (ol[0]=='3')?3:(ol[0]=='2')?2:(ol[0]=='1')?1:0; // LLVMCodeGenOptLevel
              void* tmachine = f_LLVMCreateTargetMachine(tgt, triple, "", "", opt, /*Reloc*/0, /*CodeModel*/0);
              if (!tmachine) { goto TM_MOD; }
              // Emit object (1 = LLVMObjectFile)
              if (f_LLVMTargetMachineEmitToFile(tmachine, mod, (char*)obj_out, /*Object*/1, &emsg)) {
                if (emsg) f_LLVMDisposeMessage(emsg);
              } else {
                rc = 0;
              }
              f_LLVMDisposeTargetMachine(tmachine);
            TM_MOD:
              if (mod) f_LLVMDisposeModule(mod);
              if (triple) f_LLVMDisposeMessage(triple);
              f_LLVMDisposeMemoryBuffer(buf);
            TM_CTX:
              f_LLVMContextDispose(ctx);
            TM_END: ;
            }
            dlclose(h);
          }
        }
      }
#endif

      if (rc != 0) {
        char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
        rc = system(cmd);
      }
      remove(llpath);
      yyjson_doc_free(d);
      if (rc == 0) return 0;
    GEN_ABORT:;
      // fall through to pattern lowers
    GEN_END: ;
    } while(0);
    // Try minimal pure path #1: recognize simple Ret(Const)
    {
      yyjson_read_err rerr0; yyjson_doc* d0 = yyjson_read_file(json_in, 0, NULL, &rerr0);
      if (d0) {
        yyjson_val* root0 = yyjson_doc_get_root(d0);
        yyjson_val* fns0 = yyjson_obj_get(root0, "functions");
        yyjson_val* fn00 = fns0 && yyjson_is_arr(fns0) ? yyjson_arr_get_first(fns0) : NULL;
        yyjson_val* blocks0 = fn00 && yyjson_is_obj(fn00) ? yyjson_obj_get(fn00, "blocks") : NULL;
        long long ret_const = 0; int have_ret = 0;
        if (blocks0 && yyjson_is_arr(blocks0)) {
          // Build dst->const map across blocks
          // Note: i64 only for the minimal path
          long long const_map_id[64]; long long const_map_val[64]; size_t const_n=0;
          size_t blen0 = yyjson_arr_size(blocks0);
          for (size_t bi=0; bi<blen0; bi++) {
            yyjson_val* b = yyjson_arr_get(blocks0, bi);
            yyjson_val* insts = yyjson_obj_get(b, "instructions"); if (!insts || !yyjson_is_arr(insts)) continue;
            size_t ilen = yyjson_arr_size(insts);
            for (size_t ii=0; ii<ilen; ii++) {
              yyjson_val* ins = yyjson_arr_get(insts, ii);
              const char* op = yyjson_get_str(yyjson_obj_get(ins, "op")); if (!op) continue;
              if (strcmp(op, "const")==0) {
                yyjson_val* dst = yyjson_obj_get(ins, "dst");
                yyjson_val* vv = yyjson_obj_get(yyjson_obj_get(ins, "value"), "value");
                if (dst && vv && const_n < 64) { const_map_id[const_n] = (long long)yyjson_get_sint(dst); const_map_val[const_n] = (long long)yyjson_get_sint(vv); const_n++; }
              } else if (strcmp(op, "ret")==0) {
                yyjson_val* v = yyjson_obj_get(ins, "value");
                if (v) {
                  long long vid = (long long)yyjson_get_sint(v);
                  for (size_t k=0;k<const_n;k++){ if (const_map_id[k]==vid){ ret_const = const_map_val[k]; have_ret=1; break; } }
                }
              }
            }
          }
        }
        if (have_ret) {
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_ret_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(d0); return set_err_owned(err_out, "failed to open .ll"); }
          fprintf(f, "; nyash minimal pure IR (ret const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "define i64 @ny_main() {\n  ret i64 %lld\n}\n", ret_const);
          fclose(f);
          char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(d0);
          if (rc == 0) return 0;
          // else continue to try pattern #2 or fallback
        } else {
          yyjson_doc_free(d0);
        }
      }
    }

    // Try minimal pure path #2: recognize simple If (const/compare/branch + two const blocks + merge ret)
    // Parse JSON quickly via yyjson and synthesize IR when possible.
    yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
    if (!doc) {
      return set_err_owned(err_out, "json read failed");
    }
    yyjson_val* root = yyjson_doc_get_root(doc);
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
    yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
    if (blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) >= 3) {
      // Expect block0: const a, const b, compare Lt, branch then=t else=e
      yyjson_val* b0 = yyjson_arr_get_first(blocks);
      yyjson_val* i0 = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
      if (i0 && yyjson_is_arr(i0) && yyjson_arr_size(i0) >= 4) {
        yyjson_val* ins0 = yyjson_arr_get(i0, 0);
        yyjson_val* ins1 = yyjson_arr_get(i0, 1);
        yyjson_val* ins2 = yyjson_arr_get(i0, 2);
        yyjson_val* ins3 = yyjson_arr_get(i0, 3);
        const char *op0 = yyjson_get_str(yyjson_obj_get(ins0, "op"));
        const char *op1 = yyjson_get_str(yyjson_obj_get(ins1, "op"));
        const char *op2 = yyjson_get_str(yyjson_obj_get(ins2, "op"));
        const char *op3 = yyjson_get_str(yyjson_obj_get(ins3, "op"));
        if (op0 && op1 && op2 && op3 &&
            strcmp(op0,"const")==0 &&
            strcmp(op1,"const")==0 &&
            strcmp(op2,"compare")==0 &&
            strcmp(op3,"branch")==0) {
          yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(ins0, "value"), "value");
          yyjson_val* v1 = yyjson_obj_get(yyjson_obj_get(ins1, "value"), "value");
          long long c0 = v0 ? (long long)yyjson_get_sint(v0) : 0;
          long long c1 = v1 ? (long long)yyjson_get_sint(v1) : 0;
          const char* cmp = yyjson_get_str(yyjson_obj_get(ins2, "cmp"));
          const char* pred = NULL;
          if (cmp) {
            if (strcmp(cmp,"Lt")==0 || strcmp(cmp,"lt")==0) pred = "slt";
            else if (strcmp(cmp,"Le")==0 || strcmp(cmp,"LE")==0 || strcmp(cmp,"le")==0) pred = "sle";
            else if (strcmp(cmp,"Eq")==0 || strcmp(cmp,"eq")==0) pred = "eq";
            else if (strcmp(cmp,"Ne")==0 || strcmp(cmp,"ne")==0) pred = "ne";
            else if (strcmp(cmp,"Ge")==0 || strcmp(cmp,"ge")==0) pred = "sge";
            else if (strcmp(cmp,"Gt")==0 || strcmp(cmp,"gt")==0) pred = "sgt";
          }
          if (pred) {
            int then_id = (int)yyjson_get_sint(yyjson_obj_get(ins3, "then"));
            int else_id = (int)yyjson_get_sint(yyjson_obj_get(ins3, "else"));
            yyjson_val* b_then = NULL;
            yyjson_val* b_else = NULL;
            yyjson_val* b_merge = NULL;
            size_t blen = yyjson_arr_size(blocks);
            for (size_t i=0;i<blen;i++) {
              yyjson_val* bi = yyjson_arr_get(blocks, i);
              int bid = (int)yyjson_get_sint(yyjson_obj_get(bi, "id"));
              if (bid == then_id) b_then = bi;
              else if (bid == else_id) b_else = bi;
            }
            for (size_t i=0;i<blen;i++) {
              yyjson_val* bi = yyjson_arr_get(blocks, i);
              yyjson_val* insts = yyjson_obj_get(bi, "instructions");
              size_t ilen = insts && yyjson_is_arr(insts) ? yyjson_arr_size(insts) : 0;
              for (size_t k=0;k<ilen;k++) {
                yyjson_val* ins = yyjson_arr_get(insts, k);
                const char* op = yyjson_get_str(yyjson_obj_get(ins, "op"));
                if (op && strcmp(op, "ret")==0) { b_merge = bi; break; }
              }
              if (b_merge) break;
            }
            if (b_then && b_else && b_merge) {
              yyjson_val* it = yyjson_obj_get(b_then, "instructions");
              yyjson_val* ie = yyjson_obj_get(b_else, "instructions");
              yyjson_val* t0 = it ? yyjson_arr_get(it, 0) : NULL;
              yyjson_val* e0 = ie ? yyjson_arr_get(ie, 0) : NULL;
              if (t0 && e0) {
                yyjson_val* tv = yyjson_obj_get(yyjson_obj_get(t0, "value"), "value");
                yyjson_val* ev = yyjson_obj_get(yyjson_obj_get(e0, "value"), "value");
                long long then_const = tv ? (long long)yyjson_get_sint(tv) : 0;
                long long else_const = ev ? (long long)yyjson_get_sint(ev) : 0;
                char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_%d.ll", "/tmp", (int)getpid());
                FILE* f = fopen(llpath, "wb");
                if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
                fprintf(f, "; nyash minimal pure IR\n");
                fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
                fprintf(f, "define i64 @ny_main() {\n");
                fprintf(f, "bb0:\n");
                fprintf(f, "  %%cmp = icmp %s i64 %lld, %lld\n", pred, c0, c1);
                fprintf(f, "  br i1 %%cmp, label %%bb_then, label %%bb_else\n\n");
                fprintf(f, "bb_then:\n  br label %%bb_merge\n\n");
                fprintf(f, "bb_else:\n  br label %%bb_merge\n\n");
                fprintf(f, "bb_merge:\n  %%r = phi i64 [ %lld, %%bb_then ], [ %lld, %%bb_else ]\n  ret i64 %%r\n}\n", then_const, else_const);
                fclose(f);
                char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
                int rc = system(cmd);
                remove(llpath);
                yyjson_doc_free(doc);
                if (rc != 0) {
                  return compile_json_compat_harness_keep(json_in, obj_out, err_out);
                }
                return 0;
              }
            }
          }
        }
      }
    }
    yyjson_doc_free(doc);
    // Try minimal pure path #3: Map birth → set → size → ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long key_c = 0, val_c = 0; int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 5) {
          // const, const, mir_call(Constructor:MapBox), mir_call(Method:set), mir_call(Method:size), ret
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          yyjson_val* i4 = yyjson_arr_get(insts, 4);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          const char* op4 = yyjson_get_str(yyjson_obj_get(i4, "op"));
          if (op0 && op1 && op2 && op3 && op4 &&
              strcmp(op0, "const")==0 && strcmp(op1, "const")==0 &&
              strcmp(op2, "mir_call")==0 && strcmp(op3, "mir_call")==0 && strcmp(op4, "mir_call")==0) {
            yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            yyjson_val* v1 = yyjson_obj_get(yyjson_obj_get(i1, "value"), "value");
            key_c = v0 ? (long long)yyjson_get_sint(v0) : 0;
            val_c = v1 ? (long long)yyjson_get_sint(v1) : 0;
            // Constructor MapBox
            yyjson_val* mc2 = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal2 = mc2 ? yyjson_obj_get(mc2, "callee") : NULL;
            const char* ctype = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "type")) : NULL;
            const char* cname = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "name")) : NULL;
            // Method set/size
            yyjson_val* mc3 = yyjson_obj_get(i3, "mir_call");
            yyjson_val* cal3 = mc3 ? yyjson_obj_get(mc3, "callee") : NULL;
            const char* m3 = cal3 ? yyjson_get_str(yyjson_obj_get(cal3, "name")) : NULL;
            yyjson_val* mc4 = yyjson_obj_get(i4, "mir_call");
            yyjson_val* cal4 = mc4 ? yyjson_obj_get(mc4, "callee") : NULL;
            const char* m4 = cal4 ? yyjson_get_str(yyjson_obj_get(cal4, "name")) : NULL;
            if (ctype && cname && strcmp(ctype, "Constructor")==0 && strcmp(cname, "MapBox")==0 &&
                m3 && strcmp(m3, "set")==0 && m4 && (strcmp(m4, "size")==0 || strcmp(m4, "len")==0)) {
              have = 1;
            }
          }
        }
        if (have) {
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_map_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          fprintf(f, "; nyash minimal pure IR (map set->size)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "declare i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "declare i64 @\"nyash.map.set_h\"(i64, i64, i64)\n");
          fprintf(f, "declare i64 @\"nyash.map.size_h\"(i64)\n\n");
          fprintf(f, "define i64 @ny_main() {\n");
          fprintf(f, "  %%h = call i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "  %%_s = call i64 @\"nyash.map.set_h\"(i64 %%h, i64 %lld, i64 %lld)\n", key_c, val_c);
          fprintf(f, "  %%sz = call i64 @\"nyash.map.size_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%sz\n}\n");
          fclose(f);
          char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #4: Array birth → push → len → ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long val_c = 0; int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 4) {
          // const, mir_call(Constructor:ArrayBox), mir_call(Method:push), mir_call(Method:len/length/size), ret
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          if (op0 && op1 && op2 && op3 && strcmp(op0, "const")==0 && strcmp(op1, "mir_call")==0 && strcmp(op2, "mir_call")==0 && strcmp(op3, "mir_call")==0) {
            yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            val_c = v0 ? (long long)yyjson_get_sint(v0) : 0;
            // Constructor ArrayBox
            yyjson_val* mc1 = yyjson_obj_get(i1, "mir_call");
            yyjson_val* cal1 = mc1 ? yyjson_obj_get(mc1, "callee") : NULL;
            const char* ctype = cal1 ? yyjson_get_str(yyjson_obj_get(cal1, "type")) : NULL;
            const char* cname = cal1 ? yyjson_get_str(yyjson_obj_get(cal1, "name")) : NULL;
            // Method push/len
            yyjson_val* mc2 = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal2 = mc2 ? yyjson_obj_get(mc2, "callee") : NULL;
            const char* m2 = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "name")) : NULL;
            yyjson_val* mc3 = yyjson_obj_get(i3, "mir_call");
            yyjson_val* cal3 = mc3 ? yyjson_obj_get(mc3, "callee") : NULL;
            const char* m3 = cal3 ? yyjson_get_str(yyjson_obj_get(cal3, "name")) : NULL;
            if (ctype && cname && strcmp(ctype, "Constructor")==0 && strcmp(cname, "ArrayBox")==0 &&
                m2 && strcmp(m2, "push")==0 && m3 && (strcmp(m3, "len")==0 || strcmp(m3, "length")==0 || strcmp(m3, "size")==0)) {
              have = 1;
            }
          }
        }
        if (have) {
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_array_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          fprintf(f, "; nyash minimal pure IR (array push->len)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
          fprintf(f, "declare i64 @\"nyash.array.push_h\"(i64, i64)\n");
          fprintf(f, "declare i64 @\"nyash.array.len_h\"(i64)\n\n");
          fprintf(f, "define i64 @ny_main() {\n");
          fprintf(f, "  %%h = call i64 @\"nyash.array.birth_h\"()\n");
          fprintf(f, "  %%_p = call i64 @\"nyash.array.push_h\"(i64 %%h, i64 %lld)\n", val_c);
          fprintf(f, "  %%len = call i64 @\"nyash.array.len_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%len\n}\n");
          fclose(f);
          char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #5: const ASCII string handle -> newbox StringBox ->
    // StringBox/RuntimeDataBox length/size -> ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long lit_len = 0;
        int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 4) {
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          if (op0 && op1 && op2 && op3 &&
              strcmp(op0, "const") == 0 &&
              strcmp(op1, "newbox") == 0 &&
              strcmp(op2, "mir_call") == 0 &&
              strcmp(op3, "ret") == 0) {
            long long lit_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
            long long box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
            long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
            long long ret_val = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));
            const char* box_type = yyjson_get_str(yyjson_obj_get(i1, "type"));
            yyjson_val* val0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            const char* lit = val0 ? yyjson_get_str(val0) : NULL;
            yyjson_val* nb_args = yyjson_obj_get(i1, "args");
            yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
            long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
            yyjson_val* mc = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
            const char* bname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
            const char* mname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
            long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
            yyjson_val* call_args = mc ? yyjson_obj_get(mc, "args") : NULL;
            size_t argc = call_args && yyjson_is_arr(call_args) ? yyjson_arr_size(call_args) : 0;
            if (lit &&
                hako_llvmc_ascii_strlen(lit, &lit_len) &&
                box_type && strcmp(box_type, "StringBox") == 0 &&
                lit_dst != 0 &&
                arg0 == lit_dst &&
                ctype && strcmp(ctype, "Method") == 0 &&
                bname && (strcmp(bname, "StringBox") == 0 || strcmp(bname, "RuntimeDataBox") == 0) &&
                mname && (strcmp(mname, "length") == 0 || strcmp(mname, "size") == 0) &&
                recv == box_dst &&
                argc == 0 &&
                call_dst != 0 &&
                ret_val == call_dst) {
              have = 1;
            }
          }
        }
        if (have) {
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_string_len_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          fprintf(f, "; nyash minimal pure IR (string length const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "define i64 @ny_main() {\n  ret i64 %lld\n}\n", lit_len);
          fclose(f);
          char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #6: const ASCII string handle -> newbox StringBox ->
    // const ASCII needle -> newbox StringBox -> StringBox.indexOf(needle) -> ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = fns && yyjson_is_arr(fns) ? yyjson_arr_get_first(fns) : NULL;
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long found_idx = -1;
        int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 6) {
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          yyjson_val* i4 = yyjson_arr_get(insts, 4);
          yyjson_val* i5 = yyjson_arr_get(insts, 5);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          const char* op4 = yyjson_get_str(yyjson_obj_get(i4, "op"));
          const char* op5 = yyjson_get_str(yyjson_obj_get(i5, "op"));
          if (op0 && op1 && op2 && op3 && op4 && op5 &&
              strcmp(op0, "const") == 0 &&
              strcmp(op1, "newbox") == 0 &&
              strcmp(op2, "const") == 0 &&
              strcmp(op3, "newbox") == 0 &&
              strcmp(op4, "mir_call") == 0 &&
              strcmp(op5, "ret") == 0) {
            long long hay_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
            long long hay_box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
            long long needle_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
            long long needle_box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i3, "dst"));
            long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(i4, "dst"));
            long long ret_val = (long long)yyjson_get_sint(yyjson_obj_get(i5, "value"));
            const char* box_type1 = yyjson_get_str(yyjson_obj_get(i1, "type"));
            const char* box_type3 = yyjson_get_str(yyjson_obj_get(i3, "type"));
            yyjson_val* hay_val = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            yyjson_val* needle_val = yyjson_obj_get(yyjson_obj_get(i2, "value"), "value");
            const char* hay = hay_val ? yyjson_get_str(hay_val) : NULL;
            const char* needle = needle_val ? yyjson_get_str(needle_val) : NULL;
            yyjson_val* nb1_args = yyjson_obj_get(i1, "args");
            yyjson_val* nb3_args = yyjson_obj_get(i3, "args");
            yyjson_val* nb1_0 = nb1_args && yyjson_is_arr(nb1_args) ? yyjson_arr_get_first(nb1_args) : NULL;
            yyjson_val* nb3_0 = nb3_args && yyjson_is_arr(nb3_args) ? yyjson_arr_get_first(nb3_args) : NULL;
            long long arg1_0 = nb1_0 ? (long long)yyjson_get_sint(nb1_0) : 0;
            long long arg3_0 = nb3_0 ? (long long)yyjson_get_sint(nb3_0) : 0;
            yyjson_val* mc = yyjson_obj_get(i4, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
            const char* bname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
            const char* mname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
            long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
            yyjson_val* call_args = mc ? yyjson_obj_get(mc, "args") : NULL;
            yyjson_val* call_arg0 = call_args && yyjson_is_arr(call_args) ? yyjson_arr_get_first(call_args) : NULL;
            size_t argc = call_args && yyjson_is_arr(call_args) ? yyjson_arr_size(call_args) : 0;
            long long call_arg0_v = call_arg0 ? (long long)yyjson_get_sint(call_arg0) : 0;
            if (hay &&
                needle &&
                hako_llvmc_ascii_index_of(hay, needle, &found_idx) &&
                box_type1 && strcmp(box_type1, "StringBox") == 0 &&
                box_type3 && strcmp(box_type3, "StringBox") == 0 &&
                hay_dst != 0 &&
                needle_dst != 0 &&
                arg1_0 == hay_dst &&
                arg3_0 == needle_dst &&
                ctype && strcmp(ctype, "Method") == 0 &&
                bname && strcmp(bname, "StringBox") == 0 &&
                mname && strcmp(mname, "indexOf") == 0 &&
                recv == hay_box_dst &&
                argc == 1 &&
                call_arg0_v == needle_box_dst &&
                call_dst != 0 &&
                ret_val == call_dst) {
              have = 1;
            }
          }
        }
        if (have) {
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_string_indexof_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          fprintf(f, "; nyash minimal pure IR (string indexOf const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "define i64 @ny_main() {\n  ret i64 %lld\n}\n", found_idx);
          fclose(f);
          char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }
  return compile_json_compat_harness_keep(json_in, obj_out, err_out);
}

__attribute__((visibility("default")))
int hako_llvmc_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return forward_link_obj_to_aot(obj_in, exe_out, extra_ldflags, err_out);
}
