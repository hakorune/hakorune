      /*
       * Same-module uniform ABI function emitter.
       *
       * This emits selected non-entry same-module function bodies for direct
       * global-call routes. It intentionally accepts only the small
       * string/env/generic-method/global-call subset recorded by MIR
       * LoweringPlan.
       */
      auto int same_module_function_value_is_string(long long reg) {
        long long base = resolve_copy_source_reg(reg);
        return get_origin(reg) == ORG_STRING ||
               get_scan_origin(reg) == ORG_STRING ||
               get_origin(base) == ORG_STRING ||
               get_scan_origin(base) == ORG_STRING;
      }

      auto int same_module_function_value_is_array(long long reg) {
        long long base = resolve_copy_source_reg(reg);
        int origin_kind = get_origin(reg);
        int scan = get_scan_origin(reg);
        int base_origin = get_origin(base);
        int base_scan = get_scan_origin(base);
        return origin_kind == ORG_ARRAY_BIRTH ||
               origin_kind == ORG_ARRAY_STRING_BIRTH ||
               scan == ORG_ARRAY_BIRTH ||
               scan == ORG_ARRAY_STRING_BIRTH ||
               base_origin == ORG_ARRAY_BIRTH ||
               base_origin == ORG_ARRAY_STRING_BIRTH ||
               base_scan == ORG_ARRAY_BIRTH ||
               base_scan == ORG_ARRAY_STRING_BIRTH;
      }

      #include "hako_llvmc_ffi_same_module_value_metadata.inc"

      auto void same_module_function_promote_array_string_push(
          long long recv,
          long long value) {
        if (!same_module_function_value_is_string(value)) return;
        promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        if (recv) {
          same_module_function_publish_origin(recv, ORG_ARRAY_STRING_BIRTH);
        }
      }

      auto int same_module_function_read_string_const(
          yyjson_val* ins,
          const char** out) {
        yyjson_val* value = ins ? yyjson_obj_get(ins, "value") : NULL;
        yyjson_val* ty = value ? yyjson_obj_get(value, "type") : NULL;
        const char* box_type = ty ? read_str(ty, "box_type") : NULL;
        const char* sval = value ? read_str(value, "value") : NULL;
        if (!(box_type && !strcmp(box_type, "StringBox") && sval)) return 0;
        if (out) *out = sval;
        return 1;
      }

      auto int same_module_function_read_i64_const(
          yyjson_val* ins,
          long long* out) {
        yyjson_val* value = ins ? yyjson_obj_get(ins, "value") : NULL;
        yyjson_val* ty = value ? yyjson_obj_get(value, "type") : NULL;
        yyjson_val* literal = value ? yyjson_obj_get(value, "value") : NULL;
        const char* ty_s = ty && yyjson_is_str(ty) ? yyjson_get_str(ty) : NULL;
        if (!(ty_s && !strcmp(ty_s, "i64") && literal && yyjson_is_num(literal))) {
          return 0;
        }
        if (out) *out = (long long)yyjson_get_sint(literal);
        return 1;
      }

      auto int same_module_function_binop_declares_i64(yyjson_val* ins) {
        const char* dst_type = read_str(ins, "dst_type");
        return dst_type && !strcmp(dst_type, "i64");
      }

      auto int same_module_function_read_void_sentinel_const(yyjson_val* ins) {
        yyjson_val* value = ins ? yyjson_obj_get(ins, "value") : NULL;
        yyjson_val* ty = value ? yyjson_obj_get(value, "type") : NULL;
        const char* ty_s = ty && yyjson_is_str(ty) ? yyjson_get_str(ty) : NULL;
        return ty_s && (!strcmp(ty_s, "void") || !strcmp(ty_s, "null"));
      }

      auto void same_module_function_publish_void_sentinel_const(long long dst) {
        put_const(dst, 0);
        set_type(dst, T_I64);
      }

      auto void same_module_function_format_i64_ref(
          char* out,
          size_t out_cap,
          long long reg) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        if (has_const(base, &cv)) {
          snprintf(out, out_cap, "%lld", cv);
        } else {
          snprintf(out, out_cap, "%%r%lld", base);
        }
      }

      #include "hako_llvmc_ffi_same_module_method_views.inc"

      struct SameModuleFunctionGetRouteView {
        int is_array_load;
        int is_module_function_array_item_get;
        int is_phi_incoming_array_item_get;
        int publishes_map_get_origin;
        int prepass_publishes_map_get_origin;
        const char* result_origin_box;
      };

      auto int same_module_function_read_get_route_view(
          const struct LoweringPlanGenericMethodView* method_view,
          struct SameModuleFunctionGetRouteView* out) {
        int is_array_get = 0;
        int is_mir_json_block_inst_array_item_get = 0;
        int is_mir_json_effects_array_item_get = 0;
        int is_mir_json_function_block_array_item_get = 0;
        int is_mir_json_params_array_item_get = 0;
        int is_mir_json_phi_incoming_pair_scalar_get = 0;
        int is_mir_json_vid_array_item_get = 0;
        int is_mir_json_flags_rec_array_get = 0;
        int is_map_get = 0;
        int is_mir_json_block_field_get = 0;
        int is_mir_json_const_value_field_get = 0;
        int is_mir_json_numeric_value_field_get = 0;
        int is_mir_json_callee_field_get = 0;
        int is_mir_json_inst_field_get = 0;
        int is_mir_json_function_field_get = 0;
        int is_mir_json_module_field_get = 0;
        int is_mir_json_flags_rec_map_get = 0;
        if (!out) return 0;
        out->is_array_load = 0;
        out->is_module_function_array_item_get = 0;
        out->is_phi_incoming_array_item_get = 0;
        out->publishes_map_get_origin = 0;
        out->prepass_publishes_map_get_origin = 0;
        out->result_origin_box = NULL;
        if (!method_view) return 0;
        out->result_origin_box = method_view->result_origin_box;
        is_array_get = same_module_method_view_is_direct_array_get(method_view);
        is_mir_json_block_inst_array_item_get =
            same_module_method_view_is_direct_mir_json_block_inst_array_item_get(method_view);
        is_mir_json_effects_array_item_get =
            same_module_method_view_is_direct_mir_json_effects_array_item_get(method_view);
        is_mir_json_function_block_array_item_get =
            same_module_method_view_is_direct_mir_json_function_block_array_item_get(method_view);
        out->is_module_function_array_item_get =
            same_module_method_view_is_direct_mir_json_module_function_array_item_get(method_view);
        is_mir_json_params_array_item_get =
            same_module_method_view_is_direct_mir_json_params_array_item_get(method_view);
        out->is_phi_incoming_array_item_get =
            same_module_method_view_is_direct_mir_json_phi_incoming_array_item_get(method_view);
        is_mir_json_phi_incoming_pair_scalar_get =
            same_module_method_view_is_direct_mir_json_phi_incoming_pair_scalar_get(method_view);
        is_mir_json_vid_array_item_get =
            same_module_method_view_is_direct_mir_json_vid_array_item_get(method_view);
        is_mir_json_flags_rec_array_get =
            same_module_method_view_is_direct_mir_json_flags_rec_array_get(method_view);
        is_map_get = same_module_method_view_is_direct_map_get(method_view);
        is_mir_json_block_field_get =
            same_module_method_view_is_direct_mir_json_block_field_get(method_view);
        is_mir_json_const_value_field_get =
            same_module_method_view_is_direct_mir_json_const_value_field_get(method_view);
        is_mir_json_numeric_value_field_get =
            same_module_method_view_is_direct_mir_json_numeric_value_field_get(method_view);
        is_mir_json_callee_field_get =
            same_module_method_view_is_direct_mir_json_callee_field_get(method_view);
        is_mir_json_inst_field_get =
            same_module_method_view_is_direct_mir_json_inst_field_get(method_view);
        is_mir_json_function_field_get =
            same_module_method_view_is_direct_mir_json_function_field_get(method_view);
        is_mir_json_module_field_get =
            same_module_method_view_is_direct_mir_json_module_field_get(method_view);
        is_mir_json_flags_rec_map_get =
            same_module_method_view_is_direct_mir_json_flags_rec_map_get(method_view);
        out->is_array_load = is_array_get ||
                             is_mir_json_block_inst_array_item_get ||
                             is_mir_json_effects_array_item_get ||
                             is_mir_json_function_block_array_item_get ||
                             out->is_module_function_array_item_get ||
                             is_mir_json_params_array_item_get ||
                             out->is_phi_incoming_array_item_get ||
                             is_mir_json_phi_incoming_pair_scalar_get ||
                             is_mir_json_vid_array_item_get ||
                             is_mir_json_flags_rec_array_get;
        out->publishes_map_get_origin =
            is_map_get ||
            is_mir_json_block_field_get ||
            is_mir_json_const_value_field_get ||
            is_mir_json_numeric_value_field_get ||
            is_mir_json_callee_field_get ||
            is_mir_json_inst_field_get ||
            is_mir_json_function_field_get ||
            is_mir_json_module_field_get ||
            is_mir_json_flags_rec_map_get;
        out->prepass_publishes_map_get_origin =
            is_map_get ||
            is_mir_json_block_field_get ||
            is_mir_json_const_value_field_get ||
            is_mir_json_callee_field_get ||
            (is_mir_json_inst_field_get && !out->result_origin_box) ||
            is_mir_json_flags_rec_map_get;
        return out->is_array_load || out->publishes_map_get_origin;
      }

      auto int same_module_function_read_arg_reg(
          yyjson_val* args,
          size_t index,
          long long* out) {
        yyjson_val* val = args && yyjson_is_arr(args) && yyjson_arr_size(args) > index
                            ? yyjson_arr_get(args, index)
                            : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int same_module_function_read_array_push_value(
          yyjson_val* args,
          long long recv,
          long long* out_value) {
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        long long value = 0;
        if (argc == 1) {
          if (!same_module_function_read_arg_reg(args, 0, &value)) return 0;
        } else if (argc == 2) {
          long long recv_arg = 0;
          int receiver_matches = 0;
          if (!same_module_function_read_arg_reg(args, 0, &recv_arg)) return 0;
          receiver_matches =
              recv &&
              resolve_copy_source_reg(recv_arg) == resolve_copy_source_reg(recv);
          if (!receiver_matches && !same_module_function_value_is_array(recv_arg)) {
            return 0;
          }
          if (!same_module_function_read_arg_reg(args, 1, &value)) return 0;
        } else {
          return 0;
        }
        if (out_value) *out_value = value;
        return 1;
      }

      auto int same_module_function_read_get_key_value(
          yyjson_val* args,
          long long recv,
          long long* out_key) {
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        long long key = 0;
        if (argc == 1) {
          if (!same_module_function_read_arg_reg(args, 0, &key)) return 0;
        } else if (argc == 2) {
          long long recv_arg = 0;
          if (!same_module_function_read_arg_reg(args, 0, &recv_arg)) return 0;
          if (resolve_copy_source_reg(recv_arg) != resolve_copy_source_reg(recv)) return 0;
          if (!same_module_function_read_arg_reg(args, 1, &key)) return 0;
        } else {
          return 0;
        }
        if (out_key) *out_key = key;
        return 1;
      }

      auto void same_module_function_emit_i64_call(
          long long dst,
          const char* symbol,
          const char* args) {
        if (dst) {
          EMIT("  %%r%lld = call i64 @\"%s\"(%s)\n", dst, symbol, args);
          set_type(dst, T_I64);
        } else {
          EMIT("  call i64 @\"%s\"(%s)\n", symbol, args);
        }
      }

      auto void same_module_function_emit_i64_birth_call(
          long long dst,
          const char* symbol,
          int birth_origin) {
        same_module_function_emit_i64_call(dst, symbol, "");
        if (dst) {
          set_origin(dst, birth_origin);
          set_scan_origin(dst, birth_origin);
        }
      }

      auto void same_module_function_emit_array_slot_append_call(
          long long dst,
          long long recv,
          long long value) {
        char ab[256];
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), value, 0);
        same_module_function_emit_i64_call(dst, "nyash.array.slot_append_hh", ab);
      }

      auto int same_module_function_emit_string_needle_i64_call(
          long long dst,
          long long recv,
          yyjson_val* args,
          const char* symbol) {
        char ab[256];
        long long needle = 0;
        if (!symbol) return 0;
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1)) return 0;
        if (!same_module_function_read_arg_reg(args, 0, &needle)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), needle, 0);
        same_module_function_emit_i64_call(dst, symbol, ab);
        return 1;
      }

      auto int same_module_function_record_phi(yyjson_val* ins, long long bid) {
        struct BlockPhi* pb = ensure_phi_block(bid);
        struct PhiRec pr;
        yyjson_val* vals = ins ? yyjson_obj_get(ins, "values") : NULL;
        yyjson_val* dst_type = ins ? yyjson_obj_get(ins, "dst_type") : NULL;
        const char* dst_type_s = ins ? read_str(ins, "dst_type") : NULL;
        int explicit_type = 0;
        if (!vals) vals = ins ? yyjson_obj_get(ins, "incoming") : NULL;
        if (!pb) return 0;
        pr.dst = read_int(ins, "dst");
        pr.in_n = 0;
        pr.explicit_type = 0;
        if (vals && yyjson_is_arr(vals)) {
          size_t vn = yyjson_arr_size(vals);
          for (size_t vi = 0; vi < vn && pr.in_n < 16; vi++) {
            yyjson_val* ent = yyjson_arr_get(vals, vi);
            long long pred = 0;
            long long vin = 0;
            if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 2) {
              yyjson_val* ent0 = yyjson_arr_get(ent, 0);
              yyjson_val* ent1 = yyjson_arr_get(ent, 1);
              vin = ent0 ? (long long)yyjson_get_sint(ent0) : 0;
              pred = ent1 ? (long long)yyjson_get_sint(ent1) : 0;
            } else {
              pred = read_int(ent, "pred");
              if (!pred) pred = read_int(ent, "block");
              vin = read_int(ent, "value");
            }
            pr.in[pr.in_n].pred = pred;
            pr.in[pr.in_n].val_reg = vin;
            pr.in_n++;
          }
        }
        if (pb->rec_n >= 24) return 0;
        if (dst_type_s &&
            (!strcmp(dst_type_s, "i1") || !strcmp(dst_type_s, "bool") ||
             !strcmp(dst_type_s, "Bool"))) {
          set_type(pr.dst, T_I1);
          explicit_type = 1;
        } else if (dst_type_s && !strcmp(dst_type_s, "i64")) {
          set_type(pr.dst, T_I64);
          explicit_type = 1;
        } else if (dst_type && yyjson_is_obj(dst_type)) {
          explicit_type =
              same_module_function_publish_handle_value_metadata(pr.dst, dst_type);
        } else if (pr.dst && pr.in_n > 0) {
          int all_typed_object = 1;
          int typed_plan_index = -1;
          for (int ii = 0; ii < pr.in_n; ii++) {
            int in_typed_plan_index = -1;
            if (!get_typed_object_binding(
                    pr.in[ii].val_reg, &in_typed_plan_index)) {
              all_typed_object = 0;
              break;
            }
            if (typed_plan_index < 0) {
              typed_plan_index = in_typed_plan_index;
            } else if (typed_plan_index != in_typed_plan_index) {
              all_typed_object = 0;
              break;
            }
          }
          if (all_typed_object && typed_plan_index >= 0) {
            same_module_function_publish_i64_origin(pr.dst, ORG_TYPED_OBJECT);
            set_typed_object_binding(pr.dst, typed_plan_index);
          }
        }
        pr.explicit_type = explicit_type;
        pb->recs[pb->rec_n++] = pr;
        return 1;
      }

      auto void same_module_function_refine_phi_types(void) {
        for (int pass = 0; pass < 8; pass++) {
          int changed = 0;
          for (int pi = 0; pi < phi_n; pi++) {
            for (int ri = 0; ri < phis[pi].rec_n; ri++) {
              struct PhiRec* pr = &phis[pi].recs[ri];
              int all_string = pr->in_n > 0;
              int all_bool = pr->in_n > 0;
              int all_typed_object = pr->in_n > 0;
              int typed_plan_index = -1;
              int saw_known = 0;
              if (!pr->dst) continue;
              for (int ii = 0; ii < pr->in_n; ii++) {
                long long in_reg = pr->in[ii].val_reg;
                int ty = get_type(in_reg);
                int is_string = same_module_function_value_is_string(in_reg);
                int in_typed_plan_index = -1;
                int is_typed_object =
                    get_typed_object_binding(in_reg, &in_typed_plan_index);
                all_string = all_string && is_string;
                all_bool = all_bool && is_bool_like_phi_input(in_reg);
                all_typed_object =
                    all_typed_object &&
                    is_typed_object &&
                    (typed_plan_index < 0 ||
                     typed_plan_index == in_typed_plan_index);
                if (is_typed_object && typed_plan_index < 0) {
                  typed_plan_index = in_typed_plan_index;
                }
                if (is_string || ty != T_NONE) saw_known = 1;
              }
              if (all_typed_object && typed_plan_index >= 0) {
                int current_plan_index = -1;
                if (!get_typed_object_binding(pr->dst, &current_plan_index) ||
                    current_plan_index != typed_plan_index) {
                  same_module_function_publish_i64_origin(pr->dst, ORG_TYPED_OBJECT);
                  set_typed_object_binding(pr->dst, typed_plan_index);
                  changed = 1;
                }
              } else if (all_string) {
                if (get_origin(pr->dst) != ORG_STRING) {
                  same_module_function_publish_i64_origin(pr->dst, ORG_STRING);
                  changed = 1;
                }
              } else if (!pr->explicit_type && all_bool) {
                if (get_type(pr->dst) != T_I1) {
                  set_type(pr->dst, T_I1);
                  changed = 1;
                }
              } else if (saw_known && get_type(pr->dst) == T_NONE) {
                set_type(pr->dst, T_I64);
                changed = 1;
              }
            }
          }
          if (!changed) break;
        }
      }

      auto int same_module_function_read_call_payload(
          yyjson_val* ins,
          yyjson_val** cal_out,
          yyjson_val** args_out) {
        const char* op = read_str(ins, "op");
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        if (op && !strcmp(op, "mir_call")) {
          yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
          cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          args = mc ? yyjson_obj_get(mc, "args") : NULL;
        } else if (op && !strcmp(op, "call")) {
          cal = yyjson_obj_get(ins, "callee");
          args = yyjson_obj_get(ins, "args");
        } else if (op && !strcmp(op, "boxcall")) {
          cal = ins;
          args = yyjson_obj_get(ins, "args");
        } else {
          return 0;
        }
        if (cal_out) *cal_out = cal;
        if (args_out) *args_out = args;
        return cal != NULL;
      }

      auto const char* same_module_function_call_type(const char* op, yyjson_val* cal) {
        if (op && !strcmp(op, "boxcall")) return "Method";
        return cal ? read_str(cal, "type") : NULL;
      }

      auto const char* same_module_function_call_name(const char* op, yyjson_val* cal) {
        const char* name = NULL;
        if (!cal) return NULL;
        if (op && !strcmp(op, "boxcall")) return read_str(cal, "method");
        name = read_str(cal, "name");
        if (!name) name = read_str(cal, "method");
        return name;
      }

      auto const char* same_module_function_call_box_name(const char* op, yyjson_val* cal) {
        const char* name = NULL;
        if (!cal) return NULL;
        name = read_str(cal, "box_name");
        if (!name) name = read_str(cal, "box_type");
        (void)op;
        return name;
      }

      auto int same_module_function_call_is_array_or_map_birth(
          const char* ctype,
          const char* mname,
          const char* bname) {
        return ctype && !strcmp(ctype, "Method") &&
               mname && !strcmp(mname, "birth") &&
               bname && (!strcmp(bname, "ArrayBox") || !strcmp(bname, "MapBox"));
      }

      auto long long same_module_function_call_receiver(const char* op, yyjson_val* cal) {
        if (!cal) return 0;
        if (op && !strcmp(op, "boxcall")) return read_int(cal, "box");
        return read_int(cal, "receiver");
      }

      #include "hako_llvmc_ffi_same_module_prepass.inc"

      long long same_module_function_prepass_failed_bid = -1;
      size_t same_module_function_prepass_failed_ii = 0;
      const char* same_module_function_prepass_failed_op = NULL;
      long long same_module_function_body_failed_bid = -1;
      size_t same_module_function_body_failed_ii = 0;
      const char* same_module_function_body_failed_op = NULL;

      auto void same_module_function_record_prepass_failure(
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        if (same_module_function_prepass_failed_op) return;
        same_module_function_prepass_failed_bid = bid;
        same_module_function_prepass_failed_ii = ii;
        same_module_function_prepass_failed_op = read_str(ins, "op");
      }

      auto void same_module_function_record_body_failure(
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        if (same_module_function_body_failed_op) return;
        same_module_function_body_failed_bid = bid;
        same_module_function_body_failed_ii = ii;
        same_module_function_body_failed_op = read_str(ins, "op");
      }

      auto int same_module_function_prepass_active_function(void) {
        same_module_function_prepass_failed_bid = -1;
        same_module_function_prepass_failed_ii = 0;
        same_module_function_prepass_failed_op = NULL;
        yyjson_val* params = fn0 ? yyjson_obj_get(fn0, "params") : NULL;
        if (!(params && yyjson_is_arr(params))) return 0;
        for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
          yyjson_val* param = yyjson_arr_get(params, pi);
          long long reg = param ? (long long)yyjson_get_sint(param) : 0;
          if (!same_module_function_publish_reg_metadata(reg)) {
            set_type(reg, T_I64);
          }
        }
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) return 0;
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            if (!same_module_function_prepass_instruction(block.id, ii, ins)) {
              same_module_function_record_prepass_failure(block.id, ii, ins);
              return 0;
            }
          }
        }
        same_module_function_refine_phi_types();
        return generic_capacity_ok();
      }

      #include "hako_llvmc_ffi_same_module_body_emit.inc"

      auto int same_module_function_emit_function_signature(
          FILE* f,
          yyjson_val* fn,
          const char* name) {
        yyjson_val* params = yyjson_obj_get(fn, "params");
        size_t argc = params && yyjson_is_arr(params) ? yyjson_arr_size(params) : 0;
        fprintf(f, "define i64 @\"%s\"(", name);
        for (size_t ai = 0; ai < argc; ai++) {
          yyjson_val* param = yyjson_arr_get(params, ai);
          long long reg = param ? (long long)yyjson_get_sint(param) : 0;
          fprintf(f, "%si64 %%r%lld", ai ? ", " : "", reg);
        }
        fprintf(f, ") {\n");
        return 1;
      }

      struct SameModuleFunctionContextSnapshot {
        hako_llvmc_generic_pure_function_view* active_function;
        yyjson_val* fn0;
        yyjson_val* fn_metadata;
        yyjson_val* blocks;
        size_t generic_pure_block_count;
        hako_llvmc_entry_rune_selection selection;
        struct GenericPureFunctionLoweringState lowering_state;
      };

      auto void same_module_function_save_function_context(
          struct SameModuleFunctionContextSnapshot* snapshot) {
        snapshot->active_function = active_function;
        snapshot->fn0 = fn0;
        snapshot->fn_metadata = fn_metadata;
        snapshot->blocks = blocks;
        snapshot->generic_pure_block_count = generic_pure_block_count;
        snapshot->selection = selection;
        snapshot->lowering_state = lowering_state;
      }

      auto void same_module_function_restore_function_context(
          const struct SameModuleFunctionContextSnapshot* snapshot) {
        free_owned_str_consts();
        reset_string_concat_tracking_state();
        lowering_state = snapshot->lowering_state;
        active_function = snapshot->active_function;
        fn0 = snapshot->fn0;
        fn_metadata = snapshot->fn_metadata;
        blocks = snapshot->blocks;
        generic_pure_block_count = snapshot->generic_pure_block_count;
        selection = snapshot->selection;
      }

      auto void same_module_function_activate_function_context(
          hako_llvmc_generic_pure_function_view* local_function) {
        active_function = local_function;
        fn0 = local_function->fn;
        fn_metadata = local_function->metadata;
        blocks = local_function->blocks;
        generic_pure_block_count = local_function->block_count;
        reset_generic_pure_function_lowering_state(&lowering_state);
        reset_string_concat_tracking_state();
      }

      auto int same_module_function_emit_function_pipeline(
          FILE* f,
          yyjson_val* fn,
          size_t fn_index,
          const char* name) {
        if (!same_module_function_prepass_active_function()) {
          note_pure_unsupported_shape_detail(
              same_module_function_prepass_failed_bid,
              same_module_function_prepass_failed_ii,
              same_module_function_prepass_failed_op
                  ? same_module_function_prepass_failed_op
                  : "module_function",
              "module_generic_prepass_failed",
              NULL,
              NULL,
              name,
              NULL);
          return -1;
        }
        same_module_function_emit_const_globals(f, fn_index);
        if (!same_module_function_emit_function_signature(f, fn, name)) {
          return -1;
        }
        if (!same_module_function_emit_active_body(f, fn_index)) {
          note_pure_unsupported_shape_detail(
              same_module_function_body_failed_bid,
              same_module_function_body_failed_ii,
              same_module_function_body_failed_op
                  ? same_module_function_body_failed_op
                  : "module_function",
              "module_generic_body_emit_failed",
              NULL,
              NULL,
              name,
              NULL);
          fprintf(f, "  unreachable\n");
          fprintf(f, "}\n\n");
          return -1;
        }
        fprintf(f, "}\n\n");
        return remember_emitted_same_module_function_symbol(name) ? 1 : -1;
      }

      auto int same_module_function_definition_is_eligible(
          yyjson_val* fn,
          const char* name) {
        if (!name ||
            !llvm_quoted_symbol_name_is_safe(name) ||
            !same_module_function_symbol_will_be_defined(name) ||
            same_module_function_symbol_was_emitted(name)) {
          return 0;
        }
        if (module_function_is_numeric_i64_leaf(fn) &&
            module_leaf_symbol_will_be_defined(name)) {
          return 0;
        }
        return 1;
      }

      auto int same_module_function_read_view(
          yyjson_val* fn,
          size_t fn_index,
          hako_llvmc_generic_pure_function_view* out) {
        return hako_llvmc_read_generic_pure_function_view(
                   fn,
                   fn_index,
                   0,
                   out) &&
               out->metadata;
      }

      struct SameModuleFunctionDefinitionTarget {
        yyjson_val* fn;
        size_t fn_index;
      };

      auto int same_module_function_find_planned_definition(
          const char* name,
          struct SameModuleFunctionDefinitionTarget* out) {
        yyjson_val* fn = NULL;
        size_t fn_index = 0;
        if (!out) return 0;
        out->fn = NULL;
        out->fn_index = 0;
        fn = same_module_function_find_with_index(name, &fn_index);
        if (!fn || fn == program.entry.fn) return 0;
        out->fn = fn;
        out->fn_index = fn_index;
        return 1;
      }

      auto int same_module_function_prepare_definition(
          yyjson_val* fn,
          size_t fn_index,
          const char** out_name,
          hako_llvmc_generic_pure_function_view* out_function) {
        const char* name = fn ? read_str(fn, "name") : NULL;
        if (!same_module_function_definition_is_eligible(fn, name)) {
          return 0;
        }
        if (!same_module_function_read_view(fn, fn_index, out_function)) {
          return -1;
        }
        if (out_name) *out_name = name;
        return 1;
      }

      auto int emit_same_module_function_definition(
          FILE* f,
          yyjson_val* fn,
          size_t fn_index) {
        struct SameModuleFunctionContextSnapshot saved_context;
        hako_llvmc_generic_pure_function_view local_function;
        const char* name = NULL;
        int rc = -1;
        rc = same_module_function_prepare_definition(
            fn,
            fn_index,
            &name,
            &local_function);
        if (rc <= 0) {
          return rc;
        }

        same_module_function_save_function_context(&saved_context);
        same_module_function_activate_function_context(&local_function);

        rc = same_module_function_emit_function_pipeline(
            f,
            fn,
            fn_index,
            name);

        same_module_function_restore_function_context(&saved_context);
        return rc;
      }

      auto int emit_planned_same_module_function_definition(
          FILE* f,
          const char* name,
          int* emitted) {
        struct SameModuleFunctionDefinitionTarget target;
        int rc = 0;
        if (!same_module_function_find_planned_definition(name, &target)) return 1;
        rc = emit_same_module_function_definition(
            f,
            target.fn,
            target.fn_index);
        if (rc < 0) return -1;
        if (rc > 0 && emitted) (*emitted)++;
        return 1;
      }

      auto int emit_same_module_function_definitions(FILE* f) {
        int emitted = 0;
        for (size_t i = 0; i < planned_same_module_function_symbols_n; i++) {
          const char* name = planned_same_module_function_symbols[i];
          int rc = emit_planned_same_module_function_definition(
              f,
              name,
              &emitted);
          if (rc < 0) return -1;
        }
        reset_string_concat_tracking_state();
        return emitted;
      }
