/* One private implementation shared by the file ABI and borrowed compiler.
 * Root validation never owns or frees a document. */
static yyjson_doc* hako_json_v1_read_owned_file(const char* path, char** err_out) {
    if (!path || !*path) { set_err_owned(err_out, "invalid json path"); return NULL; }
    yyjson_read_err rerr;
    yyjson_doc* doc = yyjson_read_file(path, 0, NULL, &rerr);
    if (!doc) {
        char buf[128];
        snprintf(buf, sizeof(buf), "json read error: %s (%ld)", rerr.msg ? rerr.msg : "unknown", (long)rerr.code);
        set_err_owned(err_out, buf);
        return NULL;
    }
    return doc;
}

static int hako_json_v1_validate_root(yyjson_val* root, char** err_out) {
    if (!yyjson_is_obj(root)) {
        return set_err_owned(err_out, "root is not object");
    }
    yyjson_val* schema = yyjson_obj_get(root, "schema_version");
    if (schema && !yyjson_is_str(schema)) {
        return set_err_owned(err_out, "invalid schema_version");
    }
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    if (!fns || !yyjson_is_arr(fns) || yyjson_arr_size(fns) == 0) {
        return set_err_owned(err_out, "missing functions[]");
    }
    yyjson_val* fn0 = yyjson_arr_get_first(fns);
    if (!fn0 || !yyjson_is_obj(fn0)) {
        return set_err_owned(err_out, "functions[0] not object");
    }
    yyjson_val* blocks = yyjson_obj_get(fn0, "blocks");
    if (!blocks || !yyjson_is_arr(blocks) || yyjson_arr_size(blocks) == 0) {
        return set_err_owned(err_out, "missing blocks[]");
    }
    // Quick check first block has instructions
    yyjson_val* b0 = yyjson_arr_get_first(blocks);
    if (!b0 || !yyjson_is_obj(b0)) {
        return set_err_owned(err_out, "blocks[0] not object");
    }
    yyjson_val* insts = yyjson_obj_get(b0, "instructions");
    if (!insts || !yyjson_is_arr(insts)) {
        return set_err_owned(err_out, "missing instructions[]");
    }
    return 0;
}
