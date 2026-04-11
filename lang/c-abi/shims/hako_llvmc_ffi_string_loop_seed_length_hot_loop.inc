
static int hako_llvmc_match_runtime_data_length_hot_loop_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long lit_reg = 0;
  long long newbox_dst = 0;
  long long loop_bound = 0;
  long long lit_len = 0;
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

  {
    yyjson_val* pre = yyjson_arr_get(blocks, 0);
    yyjson_val* header = yyjson_arr_get(blocks, 1);
    yyjson_val* body = yyjson_arr_get(blocks, 2);
    yyjson_val* exitb = yyjson_arr_get(blocks, 4);
    yyjson_val* pre_insts = pre ? yyjson_obj_get(pre, "instructions") : NULL;
    yyjson_val* header_insts = header ? yyjson_obj_get(header, "instructions") : NULL;
    yyjson_val* body_insts = body ? yyjson_obj_get(body, "instructions") : NULL;
    yyjson_val* exit_insts = exitb ? yyjson_obj_get(exitb, "instructions") : NULL;
    int saw_length_call = 0;
    int saw_acc_add = 0;

    if (!(pre_insts && yyjson_is_arr(pre_insts) &&
          header_insts && yyjson_is_arr(header_insts) &&
          body_insts && yyjson_is_arr(body_insts) &&
          exit_insts && yyjson_is_arr(exit_insts) &&
          yyjson_arr_size(exit_insts) == 1)) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(pre_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(pre_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* box_type = ty ? yyjson_get_str(yyjson_obj_get(ty, "box_type")) : NULL;
        const char* value = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "value")) : NULL;
        if (box_type && strcmp(box_type, "StringBox") == 0 &&
            value && hako_llvmc_ascii_strlen(value, &lit_len)) {
          lit_reg = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        }
      } else if (strcmp(op, "newbox") == 0) {
        yyjson_val* args = yyjson_obj_get(ins, "args");
        yyjson_val* arg0 = args && yyjson_is_arr(args) ? yyjson_arr_get_first(args) : NULL;
        const char* ty = yyjson_get_str(yyjson_obj_get(ins, "type"));
        long long dst = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        long long src = arg0 ? (long long)yyjson_get_sint(arg0) : 0;
        if (ty && strcmp(ty, "StringBox") == 0 && lit_reg != 0 && src == lit_reg) {
          newbox_dst = dst;
        }
      }
    }
    if (lit_reg == 0 || newbox_dst == 0 || lit_len < 0) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(header_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(header_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        if (yyjson_is_str(ty) && strcmp(yyjson_get_str(ty), "i64") == 0) {
          loop_bound = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
        }
      }
    }
    if (loop_bound <= 0) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(body_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(body_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* box_name = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* box_type = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_type")) : NULL;
        const char* method = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
        long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        if (!(ctype && strcmp(ctype, "Method") == 0 &&
              (box_name || box_type) &&
              strcmp(box_name ? box_name : box_type, "RuntimeDataBox") == 0 &&
              method && strcmp(method, "length") == 0 &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
              call_dst != 0 &&
              hako_llvmc_copy_chain_reaches_reg(body_insts, newbox_dst, recv))) {
          yyjson_doc_free(doc);
          return -1;
        }
        saw_length_call = 1;
      } else if (strcmp(op, "binop") == 0) {
        const char* kind = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (kind && strcmp(kind, "+") == 0) {
          saw_acc_add = 1;
        }
      }
    }

    if (!(saw_length_call && saw_acc_add)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  yyjson_doc_free(doc);
  return hako_llvmc_emit_string_length_hot_loop_ir(obj_out, &selection, loop_bound, lit_len);
}

static int hako_llvmc_match_string_length_hot_loop_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long loop_bound = 0;
  long long lit_len = 0;
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
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 6)) {
    yyjson_doc_free(doc);
    return -1;
  }

  #define READ_INT(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? (long long)yyjson_get_sint(yyjson_obj_get((obj), (key))) : 0)
  #define READ_STR(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? yyjson_get_str(yyjson_obj_get((obj), (key))) : NULL)
  #define BUILD_INTERESTING_NO_COPY(block_val) \
    do { \
      yyjson_val* _insts = yyjson_obj_get((block_val), "instructions"); \
      size_t _ii; \
      n = 0; \
      if (!(_insts && yyjson_is_arr(_insts))) { yyjson_doc_free(doc); return -1; } \
      for (_ii = 0; _ii < yyjson_arr_size(_insts) && n < (int)(sizeof(interesting) / sizeof(interesting[0])); _ii++) { \
        yyjson_val* _ins = yyjson_arr_get(_insts, _ii); \
        const char* _op = READ_STR(_ins, "op"); \
        if (_op && strcmp(_op, "copy") != 0) interesting[n++] = _ins; \
      } \
    } while (0)
  #define EXPECT_OP(idx, name) \
    do { \
      const char* _op = ((idx) < n) ? READ_STR(interesting[(idx)], "op") : NULL; \
      if (!_op || strcmp(_op, (name)) != 0) { yyjson_doc_free(doc); return -1; } \
    } while (0)

  {
    yyjson_val* interesting[32];
    int n = 0;
    long long receiver_reg = 0;
    long long call_dst = 0;
    long long total_phi = 0;
    long long add_lhs = 0;
    long long add_rhs = 0;
    long long lit_reg = 0;
    long long newbox_dst = 0;
    const char* lit = NULL;
    int found_body_birth = 0;
    int found_preheader_birth = 0;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
    if (n < 6) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[n - 3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op"), "compare") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 1], "op"), "branch")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (i = 0; i + 3 < (size_t)n; i++) {
      if (!hako_llvmc_str_eq(READ_STR(interesting[i], "op"), "phi")) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    loop_bound = READ_INT(yyjson_obj_get(interesting[n - 3], "value"), "value");
    if (loop_bound <= 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 2));
    if (n < 5) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (hako_llvmc_str_eq(READ_STR(interesting[0], "op"), "boxcall")) {
      receiver_reg = READ_INT(interesting[0], "box");
      call_dst = READ_INT(interesting[0], "dst");
      if (!(READ_STR(interesting[0], "method") &&
            hako_llvmc_str_eq(READ_STR(interesting[0], "method"), "length") &&
            yyjson_is_arr(yyjson_obj_get(interesting[0], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[0], "args")) == 0 &&
            READ_STR(interesting[0], "dst_type") &&
            hako_llvmc_str_eq(READ_STR(interesting[0], "dst_type"), "i64"))) {
      yyjson_doc_free(doc);
        return -1;
      }
    } else if (hako_llvmc_str_eq(READ_STR(interesting[0], "op"), "const") &&
               hako_llvmc_str_eq(READ_STR(interesting[1], "op"), "newbox") &&
               hako_llvmc_str_eq(READ_STR(interesting[2], "op"), "boxcall")) {
      yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      lit = vobj ? READ_STR(vobj, "value") : NULL;
      lit_reg = READ_INT(interesting[0], "dst");
      newbox_dst = READ_INT(interesting[1], "dst");
      receiver_reg = READ_INT(interesting[2], "box");
      call_dst = READ_INT(interesting[2], "dst");
      if (!(hako_llvmc_str_eq(box_type, "StringBox") &&
            lit && hako_llvmc_ascii_strlen(lit, &lit_len) &&
            READ_STR(interesting[1], "type") &&
            hako_llvmc_str_eq(READ_STR(interesting[1], "type"), "StringBox") &&
            yyjson_is_arr(yyjson_obj_get(interesting[1], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[1], "args")) == 1 &&
            (long long)yyjson_get_sint(yyjson_arr_get(yyjson_obj_get(interesting[1], "args"), 0)) == lit_reg &&
            receiver_reg == newbox_dst &&
            READ_STR(interesting[2], "method") &&
            hako_llvmc_str_eq(READ_STR(interesting[2], "method"), "length") &&
            yyjson_is_arr(yyjson_obj_get(interesting[2], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[2], "args")) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      found_body_birth = 1;
    } else {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!hako_llvmc_str_eq(READ_STR(interesting[n - 3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 1], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (READ_INT(yyjson_obj_get(interesting[n - 3], "value"), "value") != 1) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (!(hako_llvmc_str_eq(READ_STR(interesting[n - 2], "operation"), "+") ||
          hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op_kind"), "Add"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    add_lhs = READ_INT(found_body_birth ? interesting[3] : interesting[1], "lhs");
    add_rhs = READ_INT(found_body_birth ? interesting[3] : interesting[1], "rhs");
    if (add_lhs == call_dst) {
      total_phi = add_rhs;
    } else if (add_rhs == call_dst) {
      total_phi = add_lhs;
    } else {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!found_body_birth) {
      int receiver_phi_ok = 0;
      long long receiver_seed_reg = 0;
      BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 0));
      if (n < 4) {
        yyjson_doc_free(doc);
        return -1;
      }
      for (i = 0; i < (size_t)n; i++) {
        yyjson_val* ins = interesting[i];
        const char* op = READ_STR(ins, "op");
        if (op && strcmp(op, "const") == 0) {
          yyjson_val* vobj = yyjson_obj_get(ins, "value");
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
          const char* value = vobj ? READ_STR(vobj, "value") : NULL;
          if (hako_llvmc_str_eq(box_type, "StringBox") && value && hako_llvmc_ascii_strlen(value, &lit_len)) {
            lit_reg = READ_INT(ins, "dst");
            lit = value;
          }
        } else if (hako_llvmc_str_eq(op, "newbox")) {
          yyjson_val* args = yyjson_obj_get(ins, "args");
          if (hako_llvmc_str_eq(READ_STR(ins, "type"), "StringBox") &&
              lit &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1 &&
              (long long)yyjson_get_sint(yyjson_arr_get(args, 0)) == lit_reg) {
            newbox_dst = READ_INT(ins, "dst");
          }
        }
      }
      {
        yyjson_val* pre_insts = yyjson_obj_get(yyjson_arr_get(blocks, 0), "instructions");
        size_t j;
        receiver_seed_reg = newbox_dst;
        if (!(pre_insts && yyjson_is_arr(pre_insts))) {
          yyjson_doc_free(doc);
          return -1;
        }
        for (j = 0; j < yyjson_arr_size(pre_insts); j++) {
          yyjson_val* ins = yyjson_arr_get(pre_insts, j);
          if (hako_llvmc_str_eq(READ_STR(ins, "op"), "copy") &&
              READ_INT(ins, "src") == newbox_dst) {
            receiver_seed_reg = READ_INT(ins, "dst");
            break;
          }
        }
      }
      BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
      for (i = 0; i + 3 < (size_t)n; i++) {
        yyjson_val* phi = interesting[i];
        yyjson_val* incoming = yyjson_obj_get(phi, "incoming");
        size_t j;
        if (READ_INT(phi, "dst") != receiver_reg || !(incoming && yyjson_is_arr(incoming))) {
          continue;
        }
        for (j = 0; j < yyjson_arr_size(incoming); j++) {
          yyjson_val* ent = yyjson_arr_get(incoming, j);
          if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 1 &&
              ((long long)yyjson_get_sint(yyjson_arr_get(ent, 0)) == newbox_dst ||
               (long long)yyjson_get_sint(yyjson_arr_get(ent, 0)) == receiver_seed_reg)) {
            receiver_phi_ok = 1;
            break;
          }
        }
        if (receiver_phi_ok) {
          break;
        }
      }
      if (!(lit && newbox_dst != 0 && receiver_phi_ok)) {
        yyjson_doc_free(doc);
        return -1;
      }
      found_preheader_birth = 1;
    }

    if (!(found_body_birth || found_preheader_birth)) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (call_dst == 0 || total_phi == 0) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  yyjson_doc_free(doc);
  return hako_llvmc_emit_string_length_hot_loop_ir(obj_out, &selection, loop_bound, lit_len);

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING_NO_COPY
  #undef EXPECT_OP
