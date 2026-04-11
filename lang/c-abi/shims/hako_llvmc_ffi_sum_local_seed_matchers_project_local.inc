static int hako_llvmc_match_variant_project_local_i64_seed(
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
  const char* project_enum;
  const char* project_variant;
  const char* project_payload_type;
  char sum_subject[256];
  long long const_dst;
  long long payload_value;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long project_dst;
  long long project_value;
  long long project_tag;
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
        strcmp(op2, "variant_project") == 0 &&
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
  payload_value =
      const_value_obj ? (long long)yyjson_get_sint(yyjson_obj_get(const_value_obj, "value")) : 0;
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

  project_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  project_value = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  project_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));
  project_variant = yyjson_get_str(yyjson_obj_get(i2, "variant"));
  project_tag = (long long)yyjson_get_sint(yyjson_obj_get(i2, "tag"));
  project_payload_type = yyjson_get_str(yyjson_obj_get(i2, "payload_type"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type &&
        project_enum && project_variant && project_payload_type &&
        strcmp(sum_payload_type, "Integer") == 0 &&
        strcmp(project_payload_type, "Integer") == 0 &&
        strcmp(sum_enum, project_enum) == 0 &&
        strcmp(sum_variant, project_variant) == 0 &&
        variant_tag == project_tag &&
        sum_payload == const_dst &&
        project_value == sum_dst &&
        ret_value == project_dst)) {
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
          fn, "variant_project", sum_subject, "variant_project.payload_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_project", sum_subject, project_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_i64_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_project_local_i64_ir(
        obj_out, &selection, payload_value);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_project_local_f64_seed(
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
  const char* project_enum;
  const char* project_variant;
  const char* project_payload_type;
  char sum_subject[256];
  long long const_dst;
  double payload_value;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long project_dst;
  long long project_value;
  long long project_tag;
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
        strcmp(op2, "variant_project") == 0 &&
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
  payload_value =
      const_value_obj ? yyjson_get_real(yyjson_obj_get(const_value_obj, "value")) : 0.0;
  if (!(const_kind &&
        (strcmp(const_kind, "f64") == 0 || strcmp(const_kind, "Float") == 0))) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  project_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  project_value = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  project_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));
  project_variant = yyjson_get_str(yyjson_obj_get(i2, "variant"));
  project_tag = (long long)yyjson_get_sint(yyjson_obj_get(i2, "tag"));
  project_payload_type = yyjson_get_str(yyjson_obj_get(i2, "payload_type"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type &&
        project_enum && project_variant && project_payload_type &&
        strcmp(sum_payload_type, "Float") == 0 &&
        strcmp(project_payload_type, "Float") == 0 &&
        strcmp(sum_enum, project_enum) == 0 &&
        strcmp(sum_variant, project_variant) == 0 &&
        variant_tag == project_tag &&
        sum_payload == const_dst &&
        project_value == sum_dst &&
        ret_value == project_dst)) {
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
          fn, "variant_project", sum_subject, "variant_project.payload_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_project", sum_subject, project_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_f64_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_project_local_f64_ir(
        obj_out, &selection, payload_value);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_variant_project_local_handle_seed(
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
  const char* const_string = NULL;
  const char* op0;
  const char* op1;
  const char* op2;
  const char* op3;
  const char* sum_enum;
  const char* sum_variant;
  const char* sum_payload_type;
  const char* project_enum;
  const char* project_variant;
  const char* project_payload_type;
  char sum_subject[256];
  long long const_dst;
  long long sum_dst;
  long long sum_payload;
  long long variant_tag;
  long long project_dst;
  long long project_value;
  long long project_tag;
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
        strcmp(op2, "variant_project") == 0 &&
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
  const_string =
      const_value_obj ? yyjson_get_str(yyjson_obj_get(const_value_obj, "value")) : NULL;
  if (!(const_kind && (strcmp(const_kind, "string") == 0 || strcmp(const_kind, "String") == 0) &&
        const_string)) {
    yyjson_doc_free(doc);
    return -1;
  }

  sum_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
  sum_enum = yyjson_get_str(yyjson_obj_get(i1, "enum"));
  sum_variant = yyjson_get_str(yyjson_obj_get(i1, "variant"));
  variant_tag = (long long)yyjson_get_sint(yyjson_obj_get(i1, "tag"));
  sum_payload = (long long)yyjson_get_sint(yyjson_obj_get(i1, "payload"));
  sum_payload_type = yyjson_get_str(yyjson_obj_get(i1, "payload_type"));

  project_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
  project_value = (long long)yyjson_get_sint(yyjson_obj_get(i2, "value"));
  project_enum = yyjson_get_str(yyjson_obj_get(i2, "enum"));
  project_variant = yyjson_get_str(yyjson_obj_get(i2, "variant"));
  project_tag = (long long)yyjson_get_sint(yyjson_obj_get(i2, "tag"));
  project_payload_type = yyjson_get_str(yyjson_obj_get(i2, "payload_type"));

  ret_value = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));

  if (!(sum_enum && sum_variant && sum_payload_type &&
        project_enum && project_variant && project_payload_type &&
        strcmp(sum_payload_type, "String") == 0 &&
        strcmp(project_payload_type, "String") == 0 &&
        strcmp(sum_enum, project_enum) == 0 &&
        strcmp(sum_variant, project_variant) == 0 &&
        variant_tag == project_tag &&
        sum_payload == const_dst &&
        project_value == sum_dst &&
        ret_value == project_dst)) {
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
          fn, "variant_project", sum_subject, "variant_project.payload_local") &&
      hako_llvmc_sum_has_local_aggregate_fact(fn, "variant_make", sum_subject, sum_dst) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_make", sum_subject, sum_dst, 0, 0) &&
      hako_llvmc_sum_has_local_aggregate_selection(
          fn, "variant_project", sum_subject, project_dst, sum_dst, 1) &&
      hako_llvmc_sum_has_layout(
          fn, "variant_make", sum_subject, sum_dst, "tag_handle_payload");
  if (!metadata_ok) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);

  {
    int rc = hako_llvmc_emit_variant_project_local_handle_ir(
        obj_out, &selection, const_string);
    yyjson_doc_free(doc);
    return rc;
  }
}
