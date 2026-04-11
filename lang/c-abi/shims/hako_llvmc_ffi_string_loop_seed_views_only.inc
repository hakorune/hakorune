
static int hako_llvmc_match_substring_views_only_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long len_dst = 0;
  long long source_len = 0;
  const char* source_text = NULL;
  long long loop_bound = 0;
  long long i_phi = 0;
  long long left_phi = 0;
  long long right_phi = 0;
  long long split_dst = 0;
  long long left_body_dst = 0;
  long long right_body_dst = 0;
  size_t i;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (i = 0; i < yyjson_arr_size(fns); i++) {
    yyjson_val* candidate = yyjson_arr_get(fns, i);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (name && strcmp(name, "main") == 0) {
      fn = candidate;
      break;
    }
  }
  if (!fn) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_select_entry_runes(fn, &selection);

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  #define READ_I64(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? (long long)yyjson_get_sint(yyjson_obj_get((obj), (key))) : 0)
  #define READ_S(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? yyjson_get_str(yyjson_obj_get((obj), (key))) : NULL)
  #define BUILD_INTERESTING_NO_COPY_LOCAL(block_val) \
    do { \
      yyjson_val* _insts = yyjson_obj_get((block_val), "instructions"); \
      size_t _ii; \
      n = 0; \
      if (!(_insts && yyjson_is_arr(_insts))) { yyjson_doc_free(doc); return -1; } \
      for (_ii = 0; _ii < yyjson_arr_size(_insts) && n < (int)(sizeof(interesting) / sizeof(interesting[0])); _ii++) { \
        yyjson_val* _ins = yyjson_arr_get(_insts, _ii); \
        const char* _op = READ_S(_ins, "op"); \
        if (_op && strcmp(_op, "copy") != 0) interesting[n++] = _ins; \
      } \
    } while (0)

  {
    yyjson_val* interesting[32];
    int n = 0;
    yyjson_val* pre = yyjson_arr_get(blocks, 0);
    yyjson_val* header = yyjson_arr_get(blocks, 1);
    yyjson_val* body = yyjson_arr_get(blocks, 2);
    yyjson_val* latch = yyjson_arr_get(blocks, 3);
    yyjson_val* exitb = yyjson_arr_get(blocks, 4);
    yyjson_val* pre_insts = pre ? yyjson_obj_get(pre, "instructions") : NULL;
    yyjson_val* header_insts = header ? yyjson_obj_get(header, "instructions") : NULL;
    yyjson_val* body_insts = body ? yyjson_obj_get(body, "instructions") : NULL;
    yyjson_val* latch_insts = latch ? yyjson_obj_get(latch, "instructions") : NULL;
    yyjson_val* exit_insts = exitb ? yyjson_obj_get(exitb, "instructions") : NULL;

    if (!(pre_insts && yyjson_is_arr(pre_insts) &&
          header_insts && yyjson_is_arr(header_insts) &&
          body_insts && yyjson_is_arr(body_insts) &&
          latch_insts && yyjson_is_arr(latch_insts) &&
          exit_insts && yyjson_is_arr(exit_insts))) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(pre);
    if (n < 5 || !hako_llvmc_str_eq(READ_S(interesting[n - 1], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (i = 0; i < (size_t)n; i++) {
      yyjson_val* ins = interesting[i];
      const char* op = READ_S(ins, "op");
      if (!(op && strcmp(op, "mir_call") == 0)) continue;
      {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? READ_S(cal, "type") : NULL;
        const char* method = cal ? READ_S(cal, "name") : NULL;
        long long receiver = cal ? READ_I64(cal, "receiver") : 0;
        if (!(ctype && strcmp(ctype, "Method") == 0 &&
              method && (strcmp(method, "length") == 0 || strcmp(method, "len") == 0) &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
              receiver != 0)) {
          continue;
        }
        len_dst = READ_I64(ins, "dst");
        for (size_t jj = 0; jj < (size_t)n; jj++) {
          yyjson_val* seed = interesting[jj];
          yyjson_val* vobj = yyjson_obj_get(seed, "value");
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* seed_op = READ_S(seed, "op");
          const char* box_type = ty ? READ_S(ty, "box_type") : NULL;
          const char* value = vobj ? READ_S(vobj, "value") : NULL;
          long long seed_dst = READ_I64(seed, "dst");
          if (!(seed_op && strcmp(seed_op, "const") == 0 &&
                box_type && strcmp(box_type, "StringBox") == 0 &&
                value && seed_dst != 0 &&
                hako_llvmc_copy_graph_reaches_reg(pre_insts, seed_dst, receiver) &&
                hako_llvmc_ascii_strlen(value, &source_len))) {
            continue;
          }
          source_text = value;
          break;
        }
        break;
      }
    }
    if (!(len_dst != 0 && source_len > 0 && source_text)) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(header);
    if (n != 6 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "compare") ||
        !hako_llvmc_str_eq(READ_S(interesting[5], "op"), "branch")) {
      yyjson_doc_free(doc);
      return -1;
    }
    i_phi = READ_I64(interesting[0], "dst");
    left_phi = READ_I64(interesting[1], "dst");
    right_phi = READ_I64(interesting[2], "dst");
    loop_bound = READ_I64(yyjson_obj_get(interesting[3], "value"), "value");
    if (!(i_phi != 0 && left_phi != 0 && right_phi != 0 && loop_bound > 0 &&
          hako_llvmc_str_eq(READ_S(interesting[4], "operation"), "<"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      long long cmp_lhs = READ_I64(interesting[4], "lhs");
      long long cmp_rhs = READ_I64(interesting[4], "rhs");
      long long bound_reg = READ_I64(interesting[3], "dst");
      if (!(hako_llvmc_copy_graph_reaches_reg(header_insts, i_phi, cmp_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(header_insts, i_phi, cmp_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(header_insts, bound_reg, cmp_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(header_insts, bound_reg, cmp_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(body);
    if (n != 7 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[5], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[6], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (READ_I64(yyjson_obj_get(interesting[0], "value"), "value") != 0 ||
        READ_I64(yyjson_obj_get(interesting[1], "value"), "value") != 2 ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "operation"), "/")) {
      yyjson_doc_free(doc);
      return -1;
    }
    split_dst = READ_I64(interesting[2], "dst");
    {
      long long divisor_reg = READ_I64(interesting[1], "dst");
      long long div_lhs = READ_I64(interesting[2], "lhs");
      long long div_rhs = READ_I64(interesting[2], "rhs");
      yyjson_val* body_vobj = yyjson_obj_get(interesting[3], "value");
      yyjson_val* body_ty = body_vobj ? yyjson_obj_get(body_vobj, "type") : NULL;
      const char* body_box_type = body_ty ? READ_S(body_ty, "box_type") : NULL;
      const char* body_text = body_vobj ? READ_S(body_vobj, "value") : NULL;
      if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst, div_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst, div_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, divisor_reg, div_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(body_insts, divisor_reg, div_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(body_box_type && strcmp(body_box_type, "StringBox") == 0 &&
            body_text && strcmp(body_text, source_text) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    for (int call_idx = 4; call_idx <= 5; call_idx++) {
      yyjson_val* ins = interesting[call_idx];
      yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      const char* ctype = cal ? READ_S(cal, "type") : NULL;
      const char* method = cal ? READ_S(cal, "name") : NULL;
      long long receiver = cal ? READ_I64(cal, "receiver") : 0;
      long long source_reg = READ_I64(interesting[3], "dst");
      if (!(ctype && strcmp(ctype, "Method") == 0 &&
            method && (strcmp(method, "substring") == 0 || strcmp(method, "slice") == 0) &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2 &&
            hako_llvmc_copy_graph_reaches_reg(body_insts, source_reg, receiver))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (call_idx == 4) {
        if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, READ_I64(interesting[0], "dst"),
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 0))) &&
              hako_llvmc_copy_graph_reaches_reg(body_insts, split_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 1))))) {
          yyjson_doc_free(doc);
          return -1;
        }
        left_body_dst = READ_I64(ins, "dst");
      } else {
        if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, split_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 0))) &&
              hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 1))))) {
          yyjson_doc_free(doc);
          return -1;
        }
        right_body_dst = READ_I64(ins, "dst");
      }
    }
    if (!(left_body_dst != 0 && right_body_dst != 0)) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(latch);
    if (n != 5 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "jump") ||
        READ_I64(yyjson_obj_get(interesting[2], "value"), "value") != 1 ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* incoming_left = yyjson_obj_get(interesting[0], "incoming");
      yyjson_val* incoming_right = yyjson_obj_get(interesting[1], "incoming");
      long long one_reg = READ_I64(interesting[2], "dst");
      long long add_lhs = READ_I64(interesting[3], "lhs");
      long long add_rhs = READ_I64(interesting[3], "rhs");
      if (!(incoming_left && yyjson_is_arr(incoming_left) && yyjson_arr_size(incoming_left) == 1 &&
            incoming_right && yyjson_is_arr(incoming_right) && yyjson_arr_size(incoming_right) == 1)) {
        yyjson_doc_free(doc);
        return -1;
      }
      if ((long long)yyjson_get_sint(yyjson_arr_get(yyjson_arr_get(incoming_left, 0), 0)) != left_body_dst ||
          (long long)yyjson_get_sint(yyjson_arr_get(yyjson_arr_get(incoming_right, 0), 0)) != right_body_dst) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(latch_insts, i_phi, add_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(latch_insts, i_phi, add_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(latch_insts, one_reg, add_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(latch_insts, one_reg, add_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(exitb);
    if (n != 5 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "ret") ||
        READ_I64(interesting[4], "value") != READ_I64(interesting[3], "dst")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (int call_idx = 0; call_idx <= 1; call_idx++) {
      yyjson_val* ins = interesting[call_idx];
      yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      const char* ctype = cal ? READ_S(cal, "type") : NULL;
      const char* method = cal ? READ_S(cal, "name") : NULL;
      long long receiver = cal ? READ_I64(cal, "receiver") : 0;
      long long expected = call_idx == 0 ? left_phi : right_phi;
      if (!(ctype && strcmp(ctype, "Method") == 0 &&
            method && (strcmp(method, "length") == 0 || strcmp(method, "len") == 0) &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
            hako_llvmc_copy_graph_reaches_reg(exit_insts, expected, receiver))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      long long len1_dst = READ_I64(interesting[0], "dst");
      long long len2_dst = READ_I64(interesting[1], "dst");
      long long add1_dst = READ_I64(interesting[2], "dst");
      long long add1_lhs = READ_I64(interesting[2], "lhs");
      long long add1_rhs = READ_I64(interesting[2], "rhs");
      long long add2_lhs = READ_I64(interesting[3], "lhs");
      long long add2_rhs = READ_I64(interesting[3], "rhs");
      if (!(hako_llvmc_str_eq(READ_S(interesting[2], "operation"), "+") &&
            hako_llvmc_str_eq(READ_S(interesting[3], "operation"), "+"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!((hako_llvmc_copy_graph_reaches_reg(exit_insts, len1_dst, add1_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len2_dst, add1_rhs)) ||
            (hako_llvmc_copy_graph_reaches_reg(exit_insts, len2_dst, add1_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len1_dst, add1_rhs)))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!((hako_llvmc_copy_graph_reaches_reg(exit_insts, add1_dst, add2_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len_dst, add2_rhs)) ||
            (hako_llvmc_copy_graph_reaches_reg(exit_insts, len_dst, add2_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, add1_dst, add2_rhs)))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  #undef READ_I64
  #undef READ_S
  #undef BUILD_INTERESTING_NO_COPY_LOCAL

  yyjson_doc_free(doc);
  return hako_llvmc_emit_substring_views_only_exit_len_ir(
      obj_out,
      &selection,
      source_len);
}
