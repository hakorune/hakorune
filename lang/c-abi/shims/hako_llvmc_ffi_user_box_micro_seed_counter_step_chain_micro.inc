static int hako_llvmc_match_userbox_counter_step_chain_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* chain_fn = NULL;
  yyjson_val* leaf_fn = NULL;
  const char* chain_fn_name = NULL;
  const char* leaf_fn_name = NULL;
  yyjson_val* blocks;
  yyjson_val* chain_blocks;
  yyjson_val* leaf_blocks;
  long long box_reg = 0;
  long long alias_reg = 0;
  long long base_reg = 0;
  long long call_dst = 0;
  long long base_value = 0;
  long long delta_value = 0;
  long long field_reg = 0;
  long long add_reg = 0;
  long long chain_alias_reg = 0;
  long long chain_call_dst = 0;
  int counter_decl_ok = 0;
  int counter_selection_ok = 0;
  int known_receiver_ok = 0;
  int direct_bench_shape = 0;
  int simple_fixture_shape = 0;
  int direct_selection_ok = 0;
  hako_llvmc_entry_rune_selection selection;
  size_t ii;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  chain_fn = hako_llvmc_find_named_function(fns, "Counter.step_chain/1");
  if (!chain_fn) {
    chain_fn = hako_llvmc_find_named_function(fns, "Counter.step_chain/0");
  }
  leaf_fn = hako_llvmc_find_named_function(fns, "Counter.step/1");
  if (!leaf_fn) {
    leaf_fn = hako_llvmc_find_named_function(fns, "Counter.step/0");
  }
  if (!(fn && chain_fn && leaf_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }
  chain_fn_name = yyjson_get_str(yyjson_obj_get(chain_fn, "name"));
  leaf_fn_name = yyjson_get_str(yyjson_obj_get(leaf_fn, "name"));
  direct_bench_shape = chain_fn_name && leaf_fn_name &&
                       strcmp(chain_fn_name, "Counter.step_chain/0") == 0 &&
                       strcmp(leaf_fn_name, "Counter.step/0") == 0;
  simple_fixture_shape = chain_fn_name && leaf_fn_name &&
                         strcmp(chain_fn_name, "Counter.step_chain/1") == 0 &&
                         strcmp(leaf_fn_name, "Counter.step/1") == 0;
  if (!(direct_bench_shape || simple_fixture_shape)) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_emit_route_trace(
      "userbox_counter_step_chain_seed",
      "lookup",
      "ok",
      "");

  hako_llvmc_select_entry_runes(fn, &selection);
  counter_decl_ok = hako_llvmc_userbox_counter_decl_ok(root);
  counter_selection_ok =
      hako_llvmc_userbox_counter_inline_scalar_selection_ok(fn, leaf_fn);
  known_receiver_ok =
      hako_llvmc_userbox_has_known_receiver_selection(fn, "Counter.step_chain");
  direct_selection_ok =
      hako_llvmc_userbox_has_inline_scalar_selection(
          fn, "user_box_field_set", "Counter.value") &&
      hako_llvmc_userbox_has_known_receiver_selection(fn, "Counter.step_chain") &&
      hako_llvmc_userbox_has_known_receiver_selection(chain_fn, "Counter.step");
  if (direct_bench_shape) {
    if (!(counter_decl_ok && direct_selection_ok)) {
      yyjson_doc_free(doc);
      return -1;
    }
  } else if (!(counter_decl_ok && counter_selection_ok && known_receiver_ok)) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_emit_route_trace(
      "userbox_counter_step_chain_seed",
      "selection",
      "ok",
      "");

  if (direct_bench_shape) {
    yyjson_val* main_blocks = yyjson_obj_get(fn, "blocks");
    long long main_aliases[16] = {0};
    size_t main_alias_n = 0;
    long long main_box_reg = 0;
    long long main_base_reg = 0;
    long long main_ops_reg = 0;
    long long main_ops_value = 0;
    int main_field_set_count = 0;
    int main_call_count = 0;

    if (!(main_blocks && yyjson_is_arr(main_blocks) && yyjson_arr_size(main_blocks) == 5)) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (ii = 0; ii < yyjson_arr_size(main_blocks); ii++) {
      yyjson_val* block = yyjson_arr_get(main_blocks, ii);
      yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
      size_t jj;

      if (!(insts && yyjson_is_arr(insts))) {
        yyjson_doc_free(doc);
        return -1;
      }

      for (jj = 0; jj < yyjson_arr_size(insts); jj++) {
        yyjson_val* ins = yyjson_arr_get(insts, jj);
        const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
        if (!op) continue;
        if (strcmp(op, "newbox") == 0) {
          const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
          if (!(box_type && strcmp(box_type, "Counter") == 0)) {
            yyjson_doc_free(doc);
            return -1;
          }
          main_box_reg = hako_llvmc_userbox_read_int(ins, "dst");
          if (!(main_box_reg > 0)) {
            yyjson_doc_free(doc);
            return -1;
          }
          if (!hako_llvmc_userbox_reg_matches_aliases(
                  main_box_reg, main_aliases, main_alias_n)) {
            if (main_alias_n >= (sizeof(main_aliases) / sizeof(main_aliases[0]))) {
              yyjson_doc_free(doc);
              return -1;
            }
            main_aliases[main_alias_n++] = main_box_reg;
          }
        } else if (strcmp(op, "copy") == 0) {
          long long src = hako_llvmc_userbox_read_int(ins, "src");
          long long dst = hako_llvmc_userbox_read_int(ins, "dst");
          if (hako_llvmc_userbox_reg_matches_aliases(src, main_aliases, main_alias_n) &&
              !hako_llvmc_userbox_reg_matches_aliases(dst, main_aliases, main_alias_n)) {
            if (main_alias_n >= (sizeof(main_aliases) / sizeof(main_aliases[0]))) {
              yyjson_doc_free(doc);
              return -1;
            }
            main_aliases[main_alias_n++] = dst;
          }
        } else if (strcmp(op, "const") == 0) {
          yyjson_val* vobj = yyjson_obj_get(ins, "value");
          yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* kind = yyjson_is_str(type_val)
                                 ? yyjson_get_str(type_val)
                                 : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                             : NULL);
          long long iv = vobj ? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
          if (!(kind && strcmp(kind, "i64") == 0)) {
            yyjson_doc_free(doc);
            return -1;
          }
          if (iv == 41) {
            main_base_reg = hako_llvmc_userbox_read_int(ins, "dst");
            base_value = iv;
          } else if (iv == 2000000) {
            main_ops_reg = hako_llvmc_userbox_read_int(ins, "dst");
            main_ops_value = iv;
          }
        } else if (strcmp(op, "field_set") == 0) {
          const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
          yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
          const char* box_type =
              declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
          long long box = hako_llvmc_userbox_read_int(ins, "box");
          long long value = hako_llvmc_userbox_read_int(ins, "value");
          if (!(field && strcmp(field, "value") == 0 && box_type &&
                strcmp(box_type, "IntegerBox") == 0 &&
                hako_llvmc_userbox_reg_matches_aliases(box, main_aliases, main_alias_n) &&
                value == main_base_reg)) {
            yyjson_doc_free(doc);
            return -1;
          }
          main_field_set_count++;
        } else if (strcmp(op, "mir_call") == 0) {
          yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
          const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
          const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
          long long receiver = cal ? hako_llvmc_userbox_read_int(cal, "receiver") : 0;
          if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
                strcmp(bname, "Counter") == 0 && mname &&
                strcmp(mname, "step_chain") == 0 && args && yyjson_is_arr(args) &&
                yyjson_arr_size(args) == 0 &&
                hako_llvmc_userbox_reg_matches_aliases(
                    receiver, main_aliases, main_alias_n))) {
            yyjson_doc_free(doc);
            return -1;
          }
          main_call_count++;
        }
      }
    }

    if (!(main_box_reg > 0 && main_base_reg > 0 && main_ops_reg > 0 &&
          main_field_set_count == 1 && main_call_count == 2 &&
          base_value == 41 && main_ops_value == 2000000)) {
      yyjson_doc_free(doc);
      return -1;
    }

    chain_blocks = yyjson_obj_get(chain_fn, "blocks");
    if (!(chain_blocks && yyjson_is_arr(chain_blocks) && yyjson_arr_size(chain_blocks) == 1)) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* block = yyjson_arr_get(chain_blocks, 0);
      yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
      yyjson_val* params = yyjson_obj_get(chain_fn, "params");
      long long chain_alias_reg = 0;
      long long chain_call_dst = 0;
      long long chain_prev_reg = 0;
      int chain_copy_count = 0;
      if (!(params && yyjson_is_arr(params) && yyjson_arr_size(params) == 1 &&
            yyjson_get_sint(yyjson_arr_get(params, 0)) == 0 &&
            insts && yyjson_is_arr(insts) &&
            (yyjson_arr_size(insts) == 3 || yyjson_arr_size(insts) == 4))) {
        yyjson_doc_free(doc);
        return -1;
      }
      for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
        yyjson_val* ins = yyjson_arr_get(insts, ii);
        const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
        if (!op) continue;
        if (strcmp(op, "copy") == 0) {
          long long src = hako_llvmc_userbox_read_int(ins, "src");
          long long dst = hako_llvmc_userbox_read_int(ins, "dst");
          if (chain_copy_count == 0 && src == 0) {
            chain_prev_reg = dst;
            chain_alias_reg = dst;
            chain_copy_count = 1;
          } else if (chain_copy_count == 1 && src == chain_prev_reg) {
            chain_prev_reg = dst;
            chain_alias_reg = dst;
            chain_copy_count = 2;
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
          const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
          long long receiver = cal ? hako_llvmc_userbox_read_int(cal, "receiver") : 0;
          if (ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Counter") == 0 && mname &&
              strcmp(mname, "step") == 0 &&
              receiver == chain_alias_reg && args && yyjson_is_arr(args) &&
              yyjson_arr_size(args) == 0) {
            /* recursive delegate route */
          } else if (ctype && strcmp(ctype, "Global") == 0 && mname &&
                     strcmp(mname, "Counter.step/0") == 0 && args &&
                     yyjson_is_arr(args) && yyjson_arr_size(args) == 1 &&
                     yyjson_get_sint(yyjson_arr_get(args, 0)) == chain_alias_reg) {
            /* direct global callee route */
          } else {
            yyjson_doc_free(doc);
            return -1;
          }
          chain_call_dst = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (strcmp(op, "ret") == 0) {
          if (hako_llvmc_userbox_read_int(ins, "value") != chain_call_dst) {
            yyjson_doc_free(doc);
            return -1;
          }
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      }
      if (!((chain_copy_count == 1 || chain_copy_count == 2) &&
            yyjson_arr_size(insts) == (size_t)(chain_copy_count + 2) &&
            chain_alias_reg > 0 && chain_call_dst > 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }

    leaf_blocks = yyjson_obj_get(leaf_fn, "blocks");
    if (!(leaf_blocks && yyjson_is_arr(leaf_blocks) && yyjson_arr_size(leaf_blocks) == 1)) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* block = yyjson_arr_get(leaf_blocks, 0);
      yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
      yyjson_val* params = yyjson_obj_get(leaf_fn, "params");
      long long leaf_recv_copy = 0;
      long long leaf_field_reg = 0;
      long long leaf_copy2_reg = 0;
      long long leaf_copy3_reg = 0;
      long long leaf_const_reg = 0;
      long long leaf_add_reg = 0;
      if (!(params && yyjson_is_arr(params) && yyjson_arr_size(params) == 1 &&
            yyjson_get_sint(yyjson_arr_get(params, 0)) == 0 &&
            insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 7)) {
        yyjson_doc_free(doc);
        return -1;
      }
      for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
        yyjson_val* ins = yyjson_arr_get(insts, ii);
        const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
        if (!op) continue;
        if (ii == 0 && strcmp(op, "copy") == 0) {
          long long src = hako_llvmc_userbox_read_int(ins, "src");
          if (src != 0) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_recv_copy = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (ii == 1 && strcmp(op, "field_get") == 0) {
          const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
          yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
          const char* box_type =
              declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
          if (!(field && strcmp(field, "value") == 0 && box_type &&
                strcmp(box_type, "IntegerBox") == 0 &&
                hako_llvmc_userbox_read_int(ins, "box") == leaf_recv_copy)) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_field_reg = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (ii == 2 && strcmp(op, "copy") == 0) {
          if (hako_llvmc_userbox_read_int(ins, "src") != leaf_field_reg) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_copy2_reg = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (ii == 3 && strcmp(op, "copy") == 0) {
          if (hako_llvmc_userbox_read_int(ins, "src") != leaf_copy2_reg) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_copy3_reg = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (ii == 4 && strcmp(op, "const") == 0) {
          yyjson_val* vobj = yyjson_obj_get(ins, "value");
          yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* kind = yyjson_is_str(type_val)
                                 ? yyjson_get_str(type_val)
                                 : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                             : NULL);
          if (!(kind && strcmp(kind, "i64") == 0 &&
                (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) == 2)) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_const_reg = hako_llvmc_userbox_read_int(ins, "dst");
          delta_value = 2;
        } else if (ii == 5 && strcmp(op, "binop") == 0) {
          const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
          if (!(operation && strcmp(operation, "+") == 0 &&
                hako_llvmc_userbox_read_int(ins, "lhs") == leaf_copy3_reg &&
                hako_llvmc_userbox_read_int(ins, "rhs") == leaf_const_reg)) {
            yyjson_doc_free(doc);
            return -1;
          }
          leaf_add_reg = hako_llvmc_userbox_read_int(ins, "dst");
        } else if (ii == 6 && strcmp(op, "ret") == 0) {
          if (hako_llvmc_userbox_read_int(ins, "value") != leaf_add_reg) {
            yyjson_doc_free(doc);
            return -1;
          }
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      }
      if (!(leaf_recv_copy > 0 && leaf_field_reg > 0 && leaf_copy2_reg > 0 &&
            leaf_copy3_reg > 0 && leaf_const_reg > 0 && leaf_add_reg > 0 &&
            delta_value == 2)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    hako_llvmc_emit_route_trace(
        "userbox_counter_step_chain_seed",
        "shape",
        "ok_direct",
        "");

    {
      int rc = hako_llvmc_emit_userbox_counter_step_local_i64_ir(
          obj_out, &selection, base_value, delta_value);
      yyjson_doc_free(doc);
      return rc;
    }
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    yyjson_val* block = yyjson_arr_get(blocks, 0);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 6)) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = yyjson_is_str(type_val)
                               ? yyjson_get_str(type_val)
                               : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                           : NULL);
        if (!(kind && strcmp(kind, "i64") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        base_reg = hako_llvmc_userbox_read_int(ins, "dst");
        base_value = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
      } else if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (!(box_type && strcmp(box_type, "Counter") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        box_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && strcmp(field, "value") == 0 && box_type &&
              strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == box_reg &&
              hako_llvmc_userbox_read_int(ins, "value") == base_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "copy") == 0) {
        if (!(hako_llvmc_userbox_read_int(ins, "src") == box_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
        alias_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Counter") == 0 && mname &&
              strcmp(mname, "step_chain") == 0 &&
              hako_llvmc_userbox_read_int(cal, "receiver") == alias_reg &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        call_dst = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "ret") == 0) {
        if (hako_llvmc_userbox_read_int(ins, "value") != call_dst) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  chain_blocks = yyjson_obj_get(chain_fn, "blocks");
  if (!(chain_blocks && yyjson_is_arr(chain_blocks) &&
        yyjson_arr_size(chain_blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* block = yyjson_arr_get(chain_blocks, 0);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    yyjson_val* params = yyjson_obj_get(chain_fn, "params");
    if (!(params && yyjson_is_arr(params) && yyjson_arr_size(params) == 1 &&
          yyjson_get_sint(yyjson_arr_get(params, 0)) == 0 &&
          insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 3)) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "copy") == 0) {
        if (!(hako_llvmc_userbox_read_int(ins, "src") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        chain_alias_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (ctype && strcmp(ctype, "Method") == 0 && bname &&
            strcmp(bname, "Counter") == 0 && mname &&
            strcmp(mname, "step") == 0 &&
            hako_llvmc_userbox_read_int(cal, "receiver") == chain_alias_reg &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0) {
          /* recursive Method-forwarding shape */
        } else if (ctype && strcmp(ctype, "Global") == 0 && mname &&
                   strcmp(mname, "Counter.step/0") == 0 && args &&
                   yyjson_is_arr(args) && yyjson_arr_size(args) == 1 &&
                   yyjson_get_sint(yyjson_arr_get(args, 0)) == chain_alias_reg) {
          /* direct global callee shape */
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
        chain_call_dst = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "ret") == 0) {
        if (hako_llvmc_userbox_read_int(ins, "value") != chain_call_dst) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  leaf_blocks = yyjson_obj_get(leaf_fn, "blocks");
  if (!(leaf_blocks && yyjson_is_arr(leaf_blocks) &&
        yyjson_arr_size(leaf_blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* block = yyjson_arr_get(leaf_blocks, 0);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    yyjson_val* params = yyjson_obj_get(leaf_fn, "params");
    if (!(params && yyjson_is_arr(params) && yyjson_arr_size(params) == 1 &&
          yyjson_get_sint(yyjson_arr_get(params, 0)) == 0 &&
          insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 4)) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "field_get") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && strcmp(field, "value") == 0 && box_type &&
              strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        field_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = yyjson_is_str(type_val)
                               ? yyjson_get_str(type_val)
                               : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                           : NULL);
        if (!(kind && strcmp(kind, "i64") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        delta_value = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
      } else if (strcmp(op, "binop") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!(operation && strcmp(operation, "+") == 0 &&
              hako_llvmc_userbox_read_int(ins, "lhs") == field_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
        add_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "ret") == 0) {
        if (hako_llvmc_userbox_read_int(ins, "value") != add_reg) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  if (!(box_reg > 0 && alias_reg > 0 && base_reg > 0 && call_dst > 0 &&
        chain_alias_reg > 0 && chain_call_dst > 0 && field_reg > 0 && add_reg > 0 &&
        base_value == 41 && delta_value == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_emit_route_trace(
      "userbox_counter_step_chain_seed",
      "shape",
      "ok",
      "");

  {
    int rc = hako_llvmc_emit_userbox_counter_step_local_i64_ir(
        obj_out, &selection, base_value, delta_value);
    yyjson_doc_free(doc);
    return rc;
  }
}
