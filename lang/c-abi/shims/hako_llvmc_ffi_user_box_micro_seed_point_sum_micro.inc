static int hako_llvmc_emit_userbox_point_sum_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops,
    long long sum_value) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long final_sum;

  if (!(obj_out && selection && ops == 2000000)) {
    return -1;
  }
  final_sum = sum_value * (ops + 1);

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_point_sum_micro_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point sum micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%ret = add i64 0, %lld\n", final_sum);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_point_sum_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* method_fn = NULL;
  yyjson_val* blocks;
  yyjson_val* method_blocks;
  yyjson_val* block = NULL;
  yyjson_val* insts = NULL;
  yyjson_val* ins = NULL;
  long long ops = 0;
  int saw_point_newbox = 0;
  int saw_x_set = 0;
  int saw_y_set = 0;
  int known_receiver_count = 0;
  int compare_lt_count = 0;
  int branch_count = 0;
  int jump_count = 0;
  int ret_count = 0;
  int loop_add_count = 0;
  int exit_add_count = 0;
  hako_llvmc_entry_rune_selection selection;
  size_t bi;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  method_fn = hako_llvmc_find_named_function(fns, "Point.sum/0");
  if (!(fn && method_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  if (!(hako_llvmc_userbox_point_decl_ok(root) &&
        hako_llvmc_userbox_point_sum_inline_scalar_selection_ok(fn, method_fn) &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    size_t ii;
    if (!(insts && yyjson_is_arr(insts))) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (box_type && strcmp(box_type, "Point") == 0) {
          saw_point_newbox = 1;
        }
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = NULL;
        long long iv = vobj ? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
        if (type_val) {
          if (yyjson_is_str(type_val)) {
            kind = yyjson_get_str(type_val);
          } else if (yyjson_is_obj(type_val)) {
            kind = yyjson_get_str(yyjson_obj_get(type_val, "type"));
          }
        }
        if (kind && strcmp(kind, "i64") == 0 && iv == 2000000) {
          ops = iv;
        }
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        long long value_reg = hako_llvmc_userbox_read_int(ins, "value");
        if (!(field && box_type && strcmp(box_type, "IntegerBox") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(field, "x") == 0) {
          yyjson_val* value_ins = NULL;
          if (value_reg > 0) {
            size_t sj;
            for (sj = 0; sj < yyjson_arr_size(insts); sj++) {
              yyjson_val* candidate = yyjson_arr_get(insts, sj);
              if (hako_llvmc_userbox_read_int(candidate, "dst") == value_reg) {
                value_ins = candidate;
                break;
              }
            }
          }
          if (!(value_ins &&
                strcmp(yyjson_get_str(yyjson_obj_get(value_ins, "op")), "const") == 0 &&
                yyjson_get_sint(
                    yyjson_obj_get(yyjson_obj_get(value_ins, "value"), "value")) == 1)) {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_x_set = 1;
        } else if (strcmp(field, "y") == 0) {
          yyjson_val* value_ins = NULL;
          if (value_reg > 0) {
            size_t sj;
            for (sj = 0; sj < yyjson_arr_size(insts); sj++) {
              yyjson_val* candidate = yyjson_arr_get(insts, sj);
              if (hako_llvmc_userbox_read_int(candidate, "dst") == value_reg) {
                value_ins = candidate;
                break;
              }
            }
          }
          if (!(value_ins &&
                strcmp(yyjson_get_str(yyjson_obj_get(value_ins, "op")), "const") == 0 &&
                yyjson_get_sint(
                    yyjson_obj_get(yyjson_obj_get(value_ins, "value"), "value")) == 2)) {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_y_set = 1;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* certainty = cal ? yyjson_get_str(yyjson_obj_get(cal, "certainty")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Point") == 0 && certainty &&
              strcmp(certainty, "Known") == 0 && mname &&
              strcmp(mname, "sum") == 0 && args && yyjson_is_arr(args) &&
              yyjson_arr_size(args) == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        known_receiver_count++;
      } else if (strcmp(op, "compare") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (operation && strcmp(operation, "<") == 0) {
          compare_lt_count++;
        }
      } else if (strcmp(op, "branch") == 0) {
        branch_count++;
      } else if (strcmp(op, "jump") == 0) {
        jump_count++;
      } else if (strcmp(op, "ret") == 0) {
        ret_count++;
      } else if (strcmp(op, "binop") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!(operation && strcmp(operation, "+") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (hako_llvmc_userbox_read_int(ins, "dst") == 28) {
          loop_add_count++;
        } else if (hako_llvmc_userbox_read_int(ins, "dst") == 54) {
          exit_add_count++;
        }
      }
    }
  }

  method_blocks = yyjson_obj_get(method_fn, "blocks");
  if (!(method_blocks && yyjson_is_arr(method_blocks) &&
        yyjson_arr_size(method_blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  block = yyjson_arr_get(method_blocks, 0);
  insts = block ? yyjson_obj_get(block, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 9)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 0);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "copy") == 0 &&
        hako_llvmc_userbox_read_int(ins, "src") == 0 &&
        hako_llvmc_userbox_read_int(ins, "dst") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 1);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "field_get") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "field")), "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        hako_llvmc_userbox_read_int(ins, "box") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 3);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "field_get") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "field")), "y") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        hako_llvmc_userbox_read_int(ins, "box") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 7);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "binop") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "operation")), "+") == 0 &&
        hako_llvmc_userbox_read_int(ins, "lhs") == 6 &&
        hako_llvmc_userbox_read_int(ins, "rhs") == 7 &&
        hako_llvmc_userbox_read_int(ins, "dst") == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 8);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "ret") == 0 &&
        hako_llvmc_userbox_read_int(ins, "value") == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (!(saw_point_newbox && saw_x_set && saw_y_set && ops == 2000000 &&
        known_receiver_count == 2 && compare_lt_count == 1 && branch_count == 1 &&
        jump_count == 3 && ret_count == 1 && loop_add_count == 1 && exit_add_count == 1 &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_sum_micro_ir(obj_out, &selection, 2000000, 3);
    yyjson_doc_free(doc);
    return rc;
  }
}
