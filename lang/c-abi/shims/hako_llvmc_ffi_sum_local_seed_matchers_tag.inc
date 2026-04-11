static int hako_llvmc_match_variant_tag_local_i64_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  yyjson_val* block0;
  yyjson_val* insts;
  yyjson_val* i0;
  yyjson_val* i1;
  yyjson_val* i2;
  yyjson_val* i3;
  yyjson_val* const_value_obj;
  yyjson_val* const_type_val;
  const char* const_kind = NULL;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* op3;
  const char* sum_enum;
  const char* sum_variant;
  const char* sum_payload_type;
  const char* tag_enum;
  char sum_subject[256];
  long long const_dst;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long tag_dst;
  long long tag_source;
  long long ret_value;
  hako_llvmc_entry_rune_selection selection;
  int metadata_ok;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  block0 = yyjson_arr_get_first(blocks);
  insts = block0 ? yyjson_obj_get(block0, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  i0 = yyjson_arr_get(insts, 0);
  i1 = yyjson_arr_get(insts, 1);
  i2 = yyjson_arr_get(insts, 2);
  i3 = yyjson_arr_get(insts, 3);
  op0 = i0 ? yyjson_get_str(yyjson_obj_get(i0, "op")) : NULL;
  op1 = i1 ? yyjson_get_str(yyjson_obj_get(i1, "op")) : NULL;
  op2 = i2 ? yyjson_get_str(yyjson_obj_get(i2, "op")) : NULL;
  op3 = i3 ? yyjson_get_str(yyjson_obj_get(i3, "op")) : NULL;
  if (!(op0 && op1 && op2 && op3 &&
        strcmp(op0, "const") == 0 &&
        strcmp(op1, "variant_make") == 0 &&
        strcmp(op2, "variant_tag") == 0 &&
        strcmp(op3, "ret") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  const_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
  const_value_obj = yyjson_obj_get(i0, "value");
  const_type_val = const_value_obj ? yyjson_obj_get(const_value_obj, "type") : NULL;
  if (const_type_val) {
    if (yyjson_is_str(const_type_val)) {
      const_kind = yyjson_get_str(const_type_val);
    } else if (yyjson_is_obj(const_type_val)) {
      const_kind = yyjson_get_str(yyjson_obj_get(const_type_val, "type"));
    }
  }
  if (!(const_kind && strcmp(const_kind, "i64") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  tag_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  tag_source = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  tag_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type && tag_enum &&
        strcmp(sum_payload_type, "Integer") == 0 &&
        strcmp(sum_enum, tag_enum) == 0 &&
        sum_payload == const_dst &&
        tag_source == sum_dst &&
        ret_value == tag_dst)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (snprintf(sum_subject, sizeof(sum_subject), "%s::%s", sum_enum, sum_variant) >=
      (int)sizeof(sum_subject)) {
    yyjson_doc_free(doc);
    return -1;
  }

  metadata_ok =
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_make", sum_subject, "variant_make.aggregate_local") &&
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_tag", sum_enum, "variant_tag.tag_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_tag", sum_enum, tag_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_tag", sum_enum, tag_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_i64_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_tag_local_i64_ir(
        obj_out, &selection, variant_tag);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_tag_local_tag_only_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  yyjson_val* block0;
  yyjson_val* insts;
  yyjson_val* i0;
  yyjson_val* i1;
  yyjson_val* i2;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* sum_enum;
  const char* sum_variant;
  const char* tag_enum;
  char sum_subject[256];
  long long sum_dst;
  long long variant_tag;
  long long tag_dst;
  long long tag_source;
  long long ret_value;
  hako_llvmc_entry_rune_selection selection;
  int metadata_ok;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  block0 = yyjson_arr_get_first(blocks);
  insts = block0 ? yyjson_obj_get(block0, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  i0 = yyjson_arr_get(insts, 0);
  i1 = yyjson_arr_get(insts, 1);
  i2 = yyjson_arr_get(insts, 2);
  op0 = i0 ? yyjson_get_str(yyjson_obj_get(i0, "op")) : NULL;
  op1 = i1 ? yyjson_get_str(yyjson_obj_get(i1, "op")) : NULL;
  op2 = i2 ? yyjson_get_str(yyjson_obj_get(i2, "op")) : NULL;
  if (!(op0 && op1 && op2 &&
        strcmp(op0, "variant_make") == 0 &&
        strcmp(op1, "variant_tag") == 0 &&
        strcmp(op2, "ret") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (yyjson_obj_get(i0, "payload") || yyjson_obj_get(i0, "payload_type")) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i0, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i0, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i0, "tag"));

  tag_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  tag_source = (long long)yyjson_get_sint(yyjson_obj_get(i1, "value"));
  tag_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));

  if (!(sum_enum && sum_variant && tag_enum &&
        strcmp(sum_enum, tag_enum) == 0 &&
        tag_source == sum_dst &&
        ret_value == tag_dst)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (snprintf(sum_subject, sizeof(sum_subject), "%s::%s", sum_enum, sum_variant) >=
      (int)sizeof(sum_subject)) {
    yyjson_doc_free(doc);
    return -1;
  }

  metadata_ok =
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_make", sum_subject, "variant_make.aggregate_local") &&
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_tag", sum_enum, "variant_tag.tag_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_tag", sum_enum, tag_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_tag", sum_enum, tag_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_only");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_tag_local_i64_ir(
        obj_out, &selection, variant_tag);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_tag_local_f64_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  yyjson_val* block0;
  yyjson_val* insts;
  yyjson_val* i0;
  yyjson_val* i1;
  yyjson_val* i2;
  yyjson_val* i3;
  yyjson_val* const_value_obj;
  yyjson_val* const_type_val;
  const char* const_kind = NULL;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* op3;
  const char* sum_enum;
  const char* sum_variant;
  const char* sum_payload_type;
  const char* tag_enum;
  char sum_subject[256];
  long long const_dst;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long tag_dst;
  long long tag_source;
  long long ret_value;
  hako_llvmc_entry_rune_selection selection;
  int metadata_ok;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  block0 = yyjson_arr_get_first(blocks);
  insts = block0 ? yyjson_obj_get(block0, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  i0 = yyjson_arr_get(insts, 0);
  i1 = yyjson_arr_get(insts, 1);
  i2 = yyjson_arr_get(insts, 2);
  i3 = yyjson_arr_get(insts, 3);
  op0 = i0 ? yyjson_get_str(yyjson_obj_get(i0, "op")) : NULL;
  op1 = i1 ? yyjson_get_str(yyjson_obj_get(i1, "op")) : NULL;
  op2 = i2 ? yyjson_get_str(yyjson_obj_get(i2, "op")) : NULL;
  op3 = i3 ? yyjson_get_str(yyjson_obj_get(i3, "op")) : NULL;
  if (!(op0 && op1 && op2 && op3 &&
        strcmp(op0, "const") == 0 &&
        strcmp(op1, "variant_make") == 0 &&
        strcmp(op2, "variant_tag") == 0 &&
        strcmp(op3, "ret") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  const_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
  const_value_obj = yyjson_obj_get(i0, "value");
  const_type_val = const_value_obj ? yyjson_obj_get(const_value_obj, "type") : NULL;
  if (const_type_val) {
    if (yyjson_is_str(const_type_val)) {
      const_kind = yyjson_get_str(const_type_val);
    } else if (yyjson_is_obj(const_type_val)) {
      const_kind = yyjson_get_str(yyjson_obj_get(const_type_val, "type"));
    }
  }
  if (!(const_kind && (strcmp(const_kind, "f64") == 0 || strcmp(const_kind, "Float") == 0))) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  tag_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  tag_source = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  tag_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type && tag_enum &&
        strcmp(sum_payload_type, "Float") == 0 &&
        strcmp(sum_enum, tag_enum) == 0 &&
        sum_payload == const_dst &&
        tag_source == sum_dst &&
        ret_value == tag_dst)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (snprintf(sum_subject, sizeof(sum_subject), "%s::%s", sum_enum, sum_variant) >=
      (int)sizeof(sum_subject)) {
    yyjson_doc_free(doc);
    return -1;
  }

  metadata_ok =
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_make", sum_subject, "variant_make.aggregate_local") &&
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_tag", sum_enum, "variant_tag.tag_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_tag", sum_enum, tag_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_tag", sum_enum, tag_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_f64_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_tag_local_i64_ir(
        obj_out, &selection, variant_tag);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_tag_local_handle_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  yyjson_val* block0;
  yyjson_val* insts;
  yyjson_val* i0;
  yyjson_val* i1;
  yyjson_val* i2;
  yyjson_val* i3;
  yyjson_val* const_value_obj;
  yyjson_val* const_type_val;
  const char* const_kind = NULL;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* op3;
  const char* sum_enum;
  const char* sum_variant;
  const char* sum_payload_type;
  const char* tag_enum;
  char sum_subject[256];
  long long const_dst;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long tag_dst;
  long long tag_source;
  long long ret_value;
  hako_llvmc_entry_rune_selection selection;
  int metadata_ok;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  block0 = yyjson_arr_get_first(blocks);
  insts = block0 ? yyjson_obj_get(block0, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  i0 = yyjson_arr_get(insts, 0);
  i1 = yyjson_arr_get(insts, 1);
  i2 = yyjson_arr_get(insts, 2);
  i3 = yyjson_arr_get(insts, 3);
  op0 = i0 ? yyjson_get_str(yyjson_obj_get(i0, "op")) : NULL;
  op1 = i1 ? yyjson_get_str(yyjson_obj_get(i1, "op")) : NULL;
  op2 = i2 ? yyjson_get_str(yyjson_obj_get(i2, "op")) : NULL;
  op3 = i3 ? yyjson_get_str(yyjson_obj_get(i3, "op")) : NULL;
  if (!(op0 && op1 && op2 && op3 &&
        strcmp(op0, "const") == 0 &&
        strcmp(op1, "variant_make") == 0 &&
        strcmp(op2, "variant_tag") == 0 &&
        strcmp(op3, "ret") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  const_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
  const_value_obj = yyjson_obj_get(i0, "value");
  const_type_val = const_value_obj ? yyjson_obj_get(const_value_obj, "type") : NULL;
  if (const_type_val) {
    if (yyjson_is_str(const_type_val)) {
      const_kind = yyjson_get_str(const_type_val);
    } else if (yyjson_is_obj(const_type_val)) {
      const_kind = yyjson_get_str(yyjson_obj_get(const_type_val, "type"));
    }
  }
  if (!(const_kind && (strcmp(const_kind, "string") == 0 || strcmp(const_kind, "String") == 0))) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  tag_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  tag_source = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  tag_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type && tag_enum &&
        strcmp(sum_payload_type, "String") == 0 &&
        strcmp(sum_enum, tag_enum) == 0 &&
        sum_payload == const_dst &&
        tag_source == sum_dst &&
        ret_value == tag_dst)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (snprintf(sum_subject, sizeof(sum_subject), "%s::%s", sum_enum, sum_variant) >=
      (int)sizeof(sum_subject)) {
    yyjson_doc_free(doc);
    return -1;
  }

  metadata_ok =
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_make", sum_subject, "variant_make.aggregate_local") &&
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_tag", sum_enum, "variant_tag.tag_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_tag", sum_enum, tag_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_tag", sum_enum, tag_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_handle_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_tag_local_i64_ir(
        obj_out, &selection, variant_tag);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_tag_copy_local_i64_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  yyjson_val* block0;
  yyjson_val* insts;
  yyjson_val* i0;
  yyjson_val* i1;
  yyjson_val* i2;
  yyjson_val* i3;
  yyjson_val* i4;
  yyjson_val* const_value_obj;
  yyjson_val* const_type_val;
  const char* const_kind = NULL;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* op3;
  const char* op4;
  const char* sum_enum;
  const char* sum_variant;
  const char* sum_payload_type;
  const char* tag_enum;
  char sum_subject[256];
  long long const_dst;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long copy_dst;
  long long copy_src;
  long long tag_dst;
  long long tag_source;
  long long ret_value;
  hako_llvmc_entry_rune_selection selection;
  int metadata_ok;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  block0 = yyjson_arr_get_first(blocks);
  insts = block0 ? yyjson_obj_get(block0, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  i0 = yyjson_arr_get(insts, 0);
  i1 = yyjson_arr_get(insts, 1);
  i2 = yyjson_arr_get(insts, 2);
  i3 = yyjson_arr_get(insts, 3);
  i4 = yyjson_arr_get(insts, 4);
  op0 = i0 ? yyjson_get_str(yyjson_obj_get(i0, "op")) : NULL;
  op1 = i1 ? yyjson_get_str(yyjson_obj_get(i1, "op")) : NULL;
  op2 = i2 ? yyjson_get_str(yyjson_obj_get(i2, "op")) : NULL;
  op3 = i3 ? yyjson_get_str(yyjson_obj_get(i3, "op")) : NULL;
  op4 = i4 ? yyjson_get_str(yyjson_obj_get(i4, "op")) : NULL;
  if (!(op0 && op1 && op2 && op3 && op4 &&
        strcmp(op0, "const") == 0 &&
        strcmp(op1, "variant_make") == 0 &&
        strcmp(op2, "copy") == 0 &&
        strcmp(op3, "variant_tag") == 0 &&
        strcmp(op4, "ret") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  const_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
  const_value_obj = yyjson_obj_get(i0, "value");
  const_type_val = const_value_obj ? yyjson_obj_get(const_value_obj, "type") : NULL;
  if (const_type_val) {
    if (yyjson_is_str(const_type_val)) {
      const_kind = yyjson_get_str(const_type_val);
    } else if (yyjson_is_obj(const_type_val)) {
      const_kind = yyjson_get_str(yyjson_obj_get(const_type_val, "type"));
    }
  }
  if (!(const_kind && strcmp(const_kind, "i64") == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  copy_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  copy_src = (long long)yyjson_get_sint(yyjson_obj_get(i2, "src"));

  tag_dst = (long long)yyjson_get_sint(yyjson_obj_get(i3, "dst"));
  tag_source = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));
  tag_enum = yyjson_get_str(yyjson_obj_get(i3, "enum"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i4, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type && tag_enum &&
        strcmp(sum_payload_type, "Integer") == 0 &&
        strcmp(sum_enum, tag_enum) == 0 &&
        sum_payload == const_dst &&
        copy_src == sum_dst &&
        tag_source == copy_dst &&
        ret_value == tag_dst)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (snprintf(sum_subject, sizeof(sum_subject), "%s::%s", sum_enum, sum_variant) >=
      (int)sizeof(sum_subject)) {
    yyjson_doc_free(doc);
    return -1;
  }

  metadata_ok =
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_make", sum_subject, "variant_make.aggregate_local") &&
      hako_llvmc_sum_has_thin_internal_selection(
          fn, "variant_tag", sum_enum, "variant_tag.tag_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_tag", sum_enum, tag_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_tag", sum_enum, tag_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_i64_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_tag_local_i64_ir(
        obj_out, &selection, variant_tag);
    yyjson_doc_free(doc);
    return rc;
  }
}

