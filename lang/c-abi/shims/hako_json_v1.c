#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "hako_json_v1.h"

// Reuse vendored yyjson under plugins/nyash-json-plugin/c/yyjson (passed via -I)
#include "yyjson.h"

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

int hako_json_v1_validate_file(const char* path, char** err_out) {
    if (!path || !*path) return set_err_owned(err_out, "invalid json path");
    yyjson_read_err rerr;
    yyjson_doc* doc = yyjson_read_file(path, 0, NULL, &rerr);
    if (!doc) {
        char buf[128];
        snprintf(buf, sizeof(buf), "json read error: %s (%ld)", rerr.msg ? rerr.msg : "unknown", (long)rerr.code);
        return set_err_owned(err_out, buf);
    }
    yyjson_val* root = yyjson_doc_get_root(doc);
    if (!yyjson_is_obj(root)) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "root is not object");
    }
    yyjson_val* schema = yyjson_obj_get(root, "schema_version");
    if (schema && !yyjson_is_str(schema)) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "invalid schema_version");
    }
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    if (!fns || !yyjson_is_arr(fns) || yyjson_arr_size(fns) == 0) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "missing functions[]");
    }
    yyjson_val* fn0 = yyjson_arr_get_first(fns);
    if (!fn0 || !yyjson_is_obj(fn0)) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "functions[0] not object");
    }
    yyjson_val* blocks = yyjson_obj_get(fn0, "blocks");
    if (!blocks || !yyjson_is_arr(blocks) || yyjson_arr_size(blocks) == 0) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "missing blocks[]");
    }
    // Quick check first block has instructions
    yyjson_val* b0 = yyjson_arr_get_first(blocks);
    if (!b0 || !yyjson_is_obj(b0)) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "blocks[0] not object");
    }
    yyjson_val* insts = yyjson_obj_get(b0, "instructions");
    if (!insts || !yyjson_is_arr(insts)) {
        yyjson_doc_free(doc);
        return set_err_owned(err_out, "missing instructions[]");
    }
    yyjson_doc_free(doc);
    return 0;
}
