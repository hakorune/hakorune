      /*
       * Module generic pure string function emitter.
       *
       * This is the first non-entry same-module function body emitter for
       * `generic_pure_string_body`. It intentionally accepts only the small
       * string/env/global-call subset recorded by MIR LoweringPlan.
       */
      auto int module_generic_string_value_is_string(long long reg) {
        long long base = resolve_copy_source_reg(reg);
        return get_origin(reg) == ORG_STRING ||
               get_scan_origin(reg) == ORG_STRING ||
               get_origin(base) == ORG_STRING ||
               get_scan_origin(base) == ORG_STRING;
      }

      auto int module_generic_string_value_is_array(long long reg) {
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

      auto void module_generic_string_publish_origin(
          long long reg,
          int origin_kind) {
        set_origin(reg, origin_kind);
        set_scan_origin(reg, origin_kind);
      }

      auto void module_generic_string_publish_i64_origin(
          long long reg,
          int origin_kind) {
        set_type(reg, T_I64);
        module_generic_string_publish_origin(reg, origin_kind);
      }

      auto void module_generic_string_promote_array_string_push(
          long long recv,
          long long value) {
        if (!module_generic_string_value_is_string(value)) return;
        promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        if (recv) {
          module_generic_string_publish_origin(recv, ORG_ARRAY_STRING_BIRTH);
        }
      }

      auto int module_generic_string_read_string_const(
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

      auto int module_generic_string_read_i64_const(
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

      auto int module_generic_string_binop_declares_i64(yyjson_val* ins) {
        const char* dst_type = read_str(ins, "dst_type");
        return dst_type && !strcmp(dst_type, "i64");
      }

      auto int module_generic_string_read_void_sentinel_const(yyjson_val* ins) {
        yyjson_val* value = ins ? yyjson_obj_get(ins, "value") : NULL;
        yyjson_val* ty = value ? yyjson_obj_get(value, "type") : NULL;
        const char* ty_s = ty && yyjson_is_str(ty) ? yyjson_get_str(ty) : NULL;
        return ty_s && (!strcmp(ty_s, "void") || !strcmp(ty_s, "null"));
      }

      auto void module_generic_string_format_i64_ref(
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

      #include "hako_llvmc_ffi_module_generic_string_method_views.inc"

      auto const char* module_generic_string_method_view_key_const_text(
          const struct LoweringPlanGenericMethodView* method_view) {
        return method_view && method_view->entry
                   ? read_str(method_view->entry, "key_const_text")
                   : NULL;
      }

      auto void module_generic_string_publish_inst_field_origin(
          long long dst,
          const struct LoweringPlanGenericMethodView* method_view,
          int publish_map_get_fallback) {
        const char* key =
            module_generic_string_method_view_key_const_text(method_view);
        if (key &&
            (!strcmp(key, "op") ||
             !strcmp(key, "operation") ||
             !strcmp(key, "op_kind") ||
             !strcmp(key, "cmp") ||
             !strcmp(key, "value"))) {
          module_generic_string_publish_origin(dst, ORG_STRING);
        } else if (key && (!strcmp(key, "args") || !strcmp(key, "effects"))) {
          module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
        } else if (publish_map_get_fallback) {
          set_origin(dst, ORG_MAP_GET);
        }
      }

      auto void module_generic_string_publish_function_field_origin(
          long long dst,
          const struct LoweringPlanGenericMethodView* method_view) {
        const char* key =
            module_generic_string_method_view_key_const_text(method_view);
        if (key && !strcmp(key, "name")) {
          module_generic_string_publish_origin(dst, ORG_STRING);
        } else if (key && (!strcmp(key, "params") || !strcmp(key, "blocks"))) {
          module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
        } else if (key && !strcmp(key, "flags")) {
          module_generic_string_publish_origin(dst, ORG_MAP_BIRTH);
        }
      }

      auto void module_generic_string_publish_module_field_origin(
          long long dst,
          const struct LoweringPlanGenericMethodView* method_view) {
        const char* key =
            module_generic_string_method_view_key_const_text(method_view);
        if (key && !strcmp(key, "functions")) {
          module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
        } else if (key && !strcmp(key, "functions_0")) {
          module_generic_string_publish_origin(dst, ORG_MAP_BIRTH);
        }
      }

      struct ModuleGenericStringGetRouteView {
        int is_array_load;
        int is_module_function_array_item_get;
        int is_phi_incoming_array_item_get;
        int publishes_map_get_origin;
        int prepass_publishes_map_get_origin;
        int is_inst_field_get;
        int is_function_field_get;
        int is_module_field_get;
      };

      auto int module_generic_string_read_get_route_view(
          const struct LoweringPlanGenericMethodView* method_view,
          struct ModuleGenericStringGetRouteView* out) {
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
        int is_mir_json_flags_rec_map_get = 0;
        if (!out) return 0;
        out->is_array_load = 0;
        out->is_module_function_array_item_get = 0;
        out->is_phi_incoming_array_item_get = 0;
        out->publishes_map_get_origin = 0;
        out->prepass_publishes_map_get_origin = 0;
        out->is_inst_field_get = 0;
        out->is_function_field_get = 0;
        out->is_module_field_get = 0;
        if (!method_view) return 0;
        is_array_get = module_generic_string_method_view_is_direct_array_get(method_view);
        is_mir_json_block_inst_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_block_inst_array_item_get(method_view);
        is_mir_json_effects_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_effects_array_item_get(method_view);
        is_mir_json_function_block_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_function_block_array_item_get(method_view);
        out->is_module_function_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_module_function_array_item_get(method_view);
        is_mir_json_params_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_params_array_item_get(method_view);
        out->is_phi_incoming_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_phi_incoming_array_item_get(method_view);
        is_mir_json_phi_incoming_pair_scalar_get =
            module_generic_string_method_view_is_direct_mir_json_phi_incoming_pair_scalar_get(method_view);
        is_mir_json_vid_array_item_get =
            module_generic_string_method_view_is_direct_mir_json_vid_array_item_get(method_view);
        is_mir_json_flags_rec_array_get =
            module_generic_string_method_view_is_direct_mir_json_flags_rec_array_get(method_view);
        is_map_get = module_generic_string_method_view_is_direct_map_get(method_view);
        is_mir_json_block_field_get =
            module_generic_string_method_view_is_direct_mir_json_block_field_get(method_view);
        is_mir_json_const_value_field_get =
            module_generic_string_method_view_is_direct_mir_json_const_value_field_get(method_view);
        is_mir_json_numeric_value_field_get =
            module_generic_string_method_view_is_direct_mir_json_numeric_value_field_get(method_view);
        is_mir_json_callee_field_get =
            module_generic_string_method_view_is_direct_mir_json_callee_field_get(method_view);
        out->is_inst_field_get =
            module_generic_string_method_view_is_direct_mir_json_inst_field_get(method_view);
        out->is_function_field_get =
            module_generic_string_method_view_is_direct_mir_json_function_field_get(method_view);
        out->is_module_field_get =
            module_generic_string_method_view_is_direct_mir_json_module_field_get(method_view);
        is_mir_json_flags_rec_map_get =
            module_generic_string_method_view_is_direct_mir_json_flags_rec_map_get(method_view);
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
            out->is_inst_field_get ||
            out->is_function_field_get ||
            out->is_module_field_get ||
            is_mir_json_flags_rec_map_get;
        out->prepass_publishes_map_get_origin =
            is_map_get ||
            is_mir_json_block_field_get ||
            is_mir_json_const_value_field_get ||
            is_mir_json_callee_field_get ||
            is_mir_json_flags_rec_map_get;
        return out->is_array_load || out->publishes_map_get_origin;
      }

      auto int module_generic_string_read_arg_reg(
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

      auto int module_generic_string_read_array_push_value(
          yyjson_val* args,
          long long* out_value) {
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        long long value = 0;
        if (argc == 1) {
          if (!module_generic_string_read_arg_reg(args, 0, &value)) return 0;
        } else if (argc == 2) {
          long long recv_arg = 0;
          if (!module_generic_string_read_arg_reg(args, 0, &recv_arg)) return 0;
          if (!module_generic_string_value_is_array(recv_arg)) return 0;
          if (!module_generic_string_read_arg_reg(args, 1, &value)) return 0;
        } else {
          return 0;
        }
        if (out_value) *out_value = value;
        return 1;
      }

      auto void module_generic_string_emit_i64_call(
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

      auto void module_generic_string_emit_i64_birth_call(
          long long dst,
          const char* symbol,
          int birth_origin) {
        module_generic_string_emit_i64_call(dst, symbol, "");
        if (dst) {
          set_origin(dst, birth_origin);
          set_scan_origin(dst, birth_origin);
        }
      }

      auto void module_generic_string_emit_array_slot_append_call(
          long long dst,
          long long recv,
          long long value) {
        char ab[256];
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), value, 0);
        module_generic_string_emit_i64_call(dst, "nyash.array.slot_append_hh", ab);
      }

      auto int module_generic_string_emit_string_needle_i64_call(
          long long dst,
          long long recv,
          yyjson_val* args,
          const char* symbol) {
        char ab[256];
        long long needle = 0;
        if (!symbol) return 0;
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1)) return 0;
        if (!module_generic_string_read_arg_reg(args, 0, &needle)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), needle, 0);
        module_generic_string_emit_i64_call(dst, symbol, ab);
        return 1;
      }

      auto int module_generic_string_record_phi(yyjson_val* ins, long long bid) {
        struct BlockPhi* pb = ensure_phi_block(bid);
        struct PhiRec pr;
        yyjson_val* vals = ins ? yyjson_obj_get(ins, "values") : NULL;
        const char* dst_type_s = ins ? read_str(ins, "dst_type") : NULL;
        if (!vals) vals = ins ? yyjson_obj_get(ins, "incoming") : NULL;
        if (!pb) return 0;
        pr.dst = read_int(ins, "dst");
        pr.in_n = 0;
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
        pb->recs[pb->rec_n++] = pr;
        if (dst_type_s &&
            (!strcmp(dst_type_s, "i1") || !strcmp(dst_type_s, "bool") ||
             !strcmp(dst_type_s, "Bool"))) {
          set_type(pr.dst, T_I1);
        } else if (dst_type_s && !strcmp(dst_type_s, "i64")) {
          set_type(pr.dst, T_I64);
        }
        return 1;
      }

      auto void module_generic_string_refine_phi_types(void) {
        for (int pass = 0; pass < 8; pass++) {
          int changed = 0;
          for (int pi = 0; pi < phi_n; pi++) {
            for (int ri = 0; ri < phis[pi].rec_n; ri++) {
              struct PhiRec* pr = &phis[pi].recs[ri];
              int all_string = pr->in_n > 0;
              int all_bool = pr->in_n > 0;
              int saw_known = 0;
              if (!pr->dst) continue;
              for (int ii = 0; ii < pr->in_n; ii++) {
                long long in_reg = pr->in[ii].val_reg;
                int ty = get_type(in_reg);
                int is_string = module_generic_string_value_is_string(in_reg);
                all_string = all_string && is_string;
                all_bool = all_bool && is_bool_like_phi_input(in_reg);
                if (is_string || ty != T_NONE) saw_known = 1;
              }
              if (all_string) {
                if (get_origin(pr->dst) != ORG_STRING) {
                  module_generic_string_publish_i64_origin(pr->dst, ORG_STRING);
                  changed = 1;
                }
              } else if (all_bool) {
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

      auto int module_generic_string_read_call_payload(
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

      auto const char* module_generic_string_call_type(const char* op, yyjson_val* cal) {
        if (op && !strcmp(op, "boxcall")) return "Method";
        return cal ? read_str(cal, "type") : NULL;
      }

      auto const char* module_generic_string_call_name(const char* op, yyjson_val* cal) {
        const char* name = NULL;
        if (!cal) return NULL;
        if (op && !strcmp(op, "boxcall")) return read_str(cal, "method");
        name = read_str(cal, "name");
        if (!name) name = read_str(cal, "method");
        return name;
      }

      auto const char* module_generic_string_call_box_name(const char* op, yyjson_val* cal) {
        const char* name = NULL;
        if (!cal) return NULL;
        name = read_str(cal, "box_name");
        if (!name) name = read_str(cal, "box_type");
        (void)op;
        return name;
      }

      auto int module_generic_string_call_is_array_or_map_birth(
          const char* ctype,
          const char* mname,
          const char* bname) {
        return ctype && !strcmp(ctype, "Method") &&
               mname && !strcmp(mname, "birth") &&
               bname && (!strcmp(bname, "ArrayBox") || !strcmp(bname, "MapBox"));
      }

      auto long long module_generic_string_call_receiver(const char* op, yyjson_val* cal) {
        if (!cal) return 0;
        if (op && !strcmp(op, "boxcall")) return read_int(cal, "box");
        return read_int(cal, "receiver");
      }

      auto int module_generic_string_prepass_generic_method_view(
          const char* op,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst,
          const struct LoweringPlanGenericMethodView* method_view) {
        struct ModuleGenericStringGetRouteView get_route;
        if (!method_view) return 0;
        if (module_generic_string_method_view_is_direct_string_len(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_array_len(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_array_push(method_view)) {
          long long recv = module_generic_string_call_receiver(op, cal);
          long long value = 0;
          if (dst) set_type(dst, T_I64);
          if (!module_generic_string_read_array_push_value(args, &value)) return -1;
          module_generic_string_promote_array_string_push(recv, value);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_mir_json_flags_keys(method_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_ARRAY_BIRTH);
          return 1;
        }
        if (module_generic_string_read_get_route_view(method_view, &get_route)) {
          set_type(dst, T_I64);
          if (get_route.is_module_function_array_item_get) {
            module_generic_string_publish_origin(dst, ORG_MAP_BIRTH);
          } else if (get_route.is_phi_incoming_array_item_get) {
            module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
          } else if (get_route.prepass_publishes_map_get_origin) {
            set_origin(dst, ORG_MAP_GET);
          }
          if (get_route.is_inst_field_get) {
            module_generic_string_publish_inst_field_origin(
                dst, method_view, 1);
          }
          if (get_route.is_function_field_get) {
            module_generic_string_publish_function_field_origin(
                dst, method_view);
          }
          if (get_route.is_module_field_get) {
            module_generic_string_publish_module_field_origin(
                dst, method_view);
          }
          return 1;
        }
        if (module_generic_string_method_view_is_direct_map_set(method_view)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_string_substring(method_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_string_indexof(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_string_lastindexof(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (module_generic_string_method_view_is_direct_string_contains(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        return 0;
      }

      auto int module_generic_string_prepass_extern_call_view(
          const struct LoweringPlanExternCallView* extern_view,
          long long dst) {
        if (!extern_view) return 0;
        if (lowering_plan_env_get_view_is_valid(extern_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (lowering_plan_env_set_view_is_valid(extern_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (lowering_plan_hostbridge_extern_invoke_view_is_valid(extern_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (lowering_plan_stage1_emit_program_json_view_is_valid(extern_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (lowering_plan_stage1_emit_mir_from_source_view_is_valid(extern_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (lowering_plan_stage1_emit_mir_from_program_json_view_is_valid(extern_view)) {
          module_generic_string_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        return 0;
      }

      auto int module_generic_string_prepass_global_call_view(
          const char* ctype,
          const char* mname,
          yyjson_val* args,
          long long dst,
          const struct LoweringPlanGlobalCallView* global_view) {
        if (global_view &&
            lowering_plan_global_call_view_requires_module_generic_definition(global_view)) {
          set_type(dst, T_I64);
          mark_global_call_result_origin_from_view(global_view, dst);
          return 1;
        }
        if (global_view &&
            lowering_plan_global_call_view_uses_leaf_i64_definition(global_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (ctype && !strcmp(ctype, "Global") &&
            classify_mir_call_global_surface(mname) ==
                HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1) {
          return 1;
        }
        return 0;
      }

      auto int module_generic_string_prepass_call_instruction(
          const char* op,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView method_view;
        struct LoweringPlanExternCallView extern_view;
        struct LoweringPlanGlobalCallView global_view;
        const char* ctype = NULL;
        const char* mname = NULL;
        const char* bname = NULL;
        int has_method_view = 0;
        int has_extern_view = 0;
        int has_global_view = 0;
        int method_prepass_rc = 0;
        long long dst = read_int(ins, "dst");
        if (!module_generic_string_read_call_payload(ins, &cal, &args)) return 0;
        ctype = module_generic_string_call_type(op, cal);
        mname = module_generic_string_call_name(op, cal);
        bname = module_generic_string_call_box_name(op, cal);
        has_method_view = read_lowering_plan_generic_method_view(entry, &method_view);
        has_extern_view = read_lowering_plan_extern_call_view(entry, &extern_view);
        has_global_view = read_lowering_plan_global_call_view(entry, &global_view);
        if (module_generic_string_call_is_array_or_map_birth(
                ctype, mname, bname)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (has_method_view) {
          method_prepass_rc = module_generic_string_prepass_generic_method_view(
              op,
              cal,
              args,
              dst,
              &method_view);
          if (method_prepass_rc < 0) return 0;
          if (method_prepass_rc > 0) return 1;
        }
        if (has_extern_view &&
            module_generic_string_prepass_extern_call_view(&extern_view, dst)) {
          return 1;
        }
        return module_generic_string_prepass_global_call_view(
            ctype,
            mname,
            args,
            dst,
            has_global_view ? &global_view : NULL);
      }

      auto int module_generic_string_prepass_externcall(
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanExternCallView extern_view;
        long long dst = read_int(ins, "dst");
        if (!read_lowering_plan_extern_call_view(entry, &extern_view)) return 0;
        return module_generic_string_prepass_extern_call_view(&extern_view, dst);
      }

      auto int module_generic_string_prepass_active_function(void) {
        yyjson_val* params = fn0 ? yyjson_obj_get(fn0, "params") : NULL;
        if (!(params && yyjson_is_arr(params))) return 0;
        for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
          yyjson_val* param = yyjson_arr_get(params, pi);
          long long reg = param ? (long long)yyjson_get_sint(param) : 0;
          module_generic_string_publish_i64_origin(reg, ORG_STRING);
        }
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) return 0;
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            const char* op = read_str(ins, "op");
            if (!op) return 0;
            if (!strcmp(op, "const")) {
              long long dst = read_int(ins, "dst");
              const char* sval = NULL;
              long long ival = 0;
              if (module_generic_string_read_string_const(ins, &sval)) {
                put_str_const(dst, sval);
                put_known_string_len(dst, (long long)strlen(sval));
                module_generic_string_publish_i64_origin(dst, ORG_STRING);
              } else if (module_generic_string_read_i64_const(ins, &ival)) {
                put_const(dst, ival);
                set_type(dst, T_I64);
              } else if (module_generic_string_read_void_sentinel_const(ins)) {
                put_const(dst, 0);
                set_type(dst, T_I64);
              } else {
                return 0;
              }
            } else if (!strcmp(op, "copy")) {
              long long dst = read_int(ins, "dst");
              long long src = read_int(ins, "src");
              int src_ty = get_type(src);
              int src_org = get_origin(src);
              int src_scan = get_scan_origin(src);
              if (src) set_copy_alias(dst, resolve_copy_source_reg(src));
              if (src_ty != T_NONE) set_type(dst, src_ty);
              if (src_org != ORG_NONE) set_origin(dst, src_org);
              if (src_scan != ORG_NONE) set_scan_origin(dst, src_scan);
            } else if (!strcmp(op, "newbox")) {
              long long dst = read_int(ins, "dst");
              const char* tname = read_str(ins, "type");
              if (tname && !strcmp(tname, "ArrayBox")) {
                module_generic_string_publish_i64_origin(dst, ORG_ARRAY_BIRTH);
              } else if (tname && !strcmp(tname, "MapBox")) {
                module_generic_string_publish_i64_origin(dst, ORG_MAP_BIRTH);
              } else {
                return 0;
              }
            } else if (!strcmp(op, "compare")) {
              set_type(read_int(ins, "dst"), T_I1);
            } else if (!strcmp(op, "unop")) {
              const char* kind = read_str(ins, "op_kind");
              if (!kind) kind = read_str(ins, "operation");
              if (!(kind && (!strcmp(kind, "not") || !strcmp(kind, "Not")))) return 0;
              set_type(read_int(ins, "dst"), T_I1);
            } else if (!strcmp(op, "phi")) {
              if (!module_generic_string_record_phi(ins, block.id)) return 0;
            } else if (!strcmp(op, "binop")) {
              long long dst = read_int(ins, "dst");
              long long lhs = read_int(ins, "lhs");
              long long rhs = read_int(ins, "rhs");
              const char* kind = read_str(ins, "op_kind");
              if (!kind) kind = read_str(ins, "operation");
              if (!module_generic_string_binop_declares_i64(ins) &&
                  kind &&
                  (!strcmp(kind, "+") || !strcmp(kind, "Add")) &&
                  (module_generic_string_value_is_string(lhs) ||
                   module_generic_string_value_is_string(rhs))) {
                module_generic_string_publish_i64_origin(dst, ORG_STRING);
              } else {
                set_type(dst, T_I64);
              }
            } else if (!strcmp(op, "mir_call") ||
                       !strcmp(op, "call") ||
                       !strcmp(op, "boxcall")) {
              if (!module_generic_string_prepass_call_instruction(
                      op,
                      block.id,
                      ii,
                      ins)) return 0;
            } else if (!strcmp(op, "externcall")) {
              if (!module_generic_string_prepass_externcall(block.id, ii, ins)) return 0;
            } else if (!strcmp(op, "branch") ||
                       !strcmp(op, "jump") ||
                       !strcmp(op, "ret") ||
                       !strcmp(op, "select") ||
                       !strcmp(op, "keepalive") ||
                       !strcmp(op, "release_strong")) {
              if (!strcmp(op, "select")) {
                long long dst = read_int(ins, "dst");
                long long then_val = read_int(ins, "then_val");
                long long else_val = read_int(ins, "else_val");
                int then_ty = get_type(then_val);
                int else_ty = get_type(else_val);
                int then_org = get_origin(then_val);
                int else_org = get_origin(else_val);
                int then_scan = get_scan_origin(then_val);
                int else_scan = get_scan_origin(else_val);
                if (then_ty == T_I1 && else_ty == T_I1) {
                  set_type(dst, T_I1);
                } else if (then_ty != T_NONE && then_ty == else_ty) {
                  set_type(dst, then_ty);
                } else if (then_ty != T_NONE && else_ty == T_NONE) {
                  set_type(dst, then_ty);
                } else if (else_ty != T_NONE && then_ty == T_NONE) {
                  set_type(dst, else_ty);
                } else {
                  set_type(dst, T_I64);
                }
                if (then_org != ORG_NONE && then_org == else_org) set_origin(dst, then_org);
                if (then_scan != ORG_NONE && then_scan == else_scan) set_scan_origin(dst, then_scan);
              }
              continue;
            } else {
              return 0;
            }
          }
        }
        module_generic_string_refine_phi_types();
        return generic_capacity_ok();
      }

      auto void module_generic_string_emit_const_globals(FILE* f, size_t fn_index) {
        for (size_t sci = 0; sci < str_consts_n; sci++) {
          size_t slen = strlen(str_consts[sci].val);
          fprintf(f, "@.fnstr_%zu_%lld = private unnamed_addr constant [%zu x i8] c\"",
                  fn_index,
                  str_consts[sci].reg,
                  slen + 1);
          hako_llvmc_emit_llvm_byte_string(f, str_consts[sci].val, (long long)slen);
          fprintf(f, "\\00\", align 1\n");
        }
      }

      auto int module_generic_string_emit_const_box(
          FILE* f,
          size_t fn_index,
          long long dst) {
        const char* sval = get_str_const(dst);
        size_t slen = sval ? strlen(sval) : 0;
        if (!sval) return 0;
        EMIT("  %%r%lld = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.fnstr_%zu_%lld, i64 0, i64 0))\n",
             dst,
             slen + 1,
             fn_index,
             dst);
        mark_string_const_box_emitted(dst);
        module_generic_string_publish_i64_origin(dst, ORG_STRING);
        return 1;
      }

      auto int module_generic_string_emit_copy(long long dst, long long src) {
        int src_ty = get_type(src);
        int src_org = get_origin(src);
        int src_scan = get_scan_origin(src);
        if (src == 0) {
          EMIT("  %%r%lld = add i64 %%r0, 0\n", dst);
        } else {
          set_copy_alias(dst, resolve_copy_source_reg(src));
        }
        if (src_ty != T_NONE) set_type(dst, src_ty);
        if (src_org != ORG_NONE) set_origin(dst, src_org);
        if (src_scan != ORG_NONE) set_scan_origin(dst, src_scan);
        return 1;
      }

      auto int module_generic_string_emit_newbox(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        const char* tname = read_str(ins, "type");
        if (tname && !strcmp(tname, "ArrayBox")) {
          module_generic_string_emit_i64_birth_call(
              dst,
              "nyash.array.birth_h",
              ORG_ARRAY_BIRTH);
          return 1;
        }
        if (tname && !strcmp(tname, "MapBox")) {
          module_generic_string_emit_i64_birth_call(
              dst,
              "nyash.map.birth_h",
              ORG_MAP_BIRTH);
          return 1;
        }
        return 0;
      }

      auto int module_generic_string_emit_unop(yyjson_val* ins) {
        const char* kind = read_str(ins, "op_kind");
        long long dst = read_int(ins, "dst");
        long long src = read_int(ins, "src");
        long long base = 0;
        long long cv = 0;
        int has_cv = 0;
        int src_ty = T_NONE;
        if (!kind) kind = read_str(ins, "operation");
        if (!src) src = read_int(ins, "operand");
        if (!(kind && (!strcmp(kind, "not") || !strcmp(kind, "Not")))) return 0;
        if (!(dst && src)) return 0;
        base = resolve_copy_source_reg(src);
        has_cv = has_const(base, &cv);
        src_ty = get_type(base);
        if (src_ty == T_I1) {
          if (has_cv) {
            EMIT("  %%r%lld = xor i1 %s, true\n", dst, cv ? "true" : "false");
          } else {
            EMIT("  %%r%lld = xor i1 %%r%lld, true\n", dst, base);
          }
        } else if (has_cv) {
          EMIT("  %%r%lld = icmp eq i64 %lld, 0\n", dst, cv);
        } else {
          EMIT("  %%r%lld = icmp eq i64 %%r%lld, 0\n", dst, base);
        }
        set_type(dst, T_I1);
        return 1;
      }

      auto int module_generic_string_emit_binop(yyjson_val* ins) {
        const char* kind = read_str(ins, "op_kind");
        const char* irop = NULL;
        long long dst = read_int(ins, "dst");
        long long lhs = read_int(ins, "lhs");
        long long rhs = read_int(ins, "rhs");
        long long lhs_base = resolve_copy_source_reg(lhs);
        long long rhs_base = resolve_copy_source_reg(rhs);
        long long chain_a = 0;
        long long chain_b = 0;
        long long chain_c = 0;
        int dst_declares_i64 = module_generic_string_binop_declares_i64(ins);
        int lhs_is_concat_chain =
            find_string_concat_pair(lhs_base, &chain_a, &chain_b) ||
            find_string_concat_triple(lhs_base, &chain_a, &chain_b, &chain_c);
        int rhs_is_concat_chain =
            find_string_concat_pair(rhs_base, &chain_a, &chain_b) ||
            find_string_concat_triple(rhs_base, &chain_a, &chain_b, &chain_c);
        char lhs_ref[64];
        char rhs_ref[64];
        if (!kind) kind = read_str(ins, "operation");
        if ((!dst_declares_i64 || lhs_is_concat_chain || rhs_is_concat_chain) &&
            kind && (!strcmp(kind, "+") || !strcmp(kind, "Add")) &&
            (module_generic_string_value_is_string(lhs) ||
             module_generic_string_value_is_string(rhs))) {
          if (!try_emit_string_concat3_chain_call(f, dst, lhs_base, rhs_base)) {
            emit_string_concat_hh_call(f, dst, lhs_base, rhs_base);
          }
          return 1;
        }
        if (kind) {
          if (!strcmp(kind, "+") || !strcmp(kind, "Add")) irop = "add";
          else if (!strcmp(kind, "-") || !strcmp(kind, "Sub")) irop = "sub";
          else if (!strcmp(kind, "*") || !strcmp(kind, "Mul")) irop = "mul";
          else if (!strcmp(kind, "/") || !strcmp(kind, "Div")) irop = "sdiv";
          else if (!strcmp(kind, "%") || !strcmp(kind, "Mod")) irop = "srem";
        }
        if (!irop) return 0;
        module_generic_string_format_i64_ref(lhs_ref, sizeof(lhs_ref), lhs);
        module_generic_string_format_i64_ref(rhs_ref, sizeof(rhs_ref), rhs);
        EMIT("  %%r%lld = %s i64 %s, %s\n", dst, irop, lhs_ref, rhs_ref);
        set_type(dst, T_I64);
        return 1;
      }

      auto int module_generic_string_emit_compare(yyjson_val* ins) {
        const char* pred = read_str(ins, "cmp");
        const char* p2 = NULL;
        long long dst = read_int(ins, "dst");
        long long lhs = read_int(ins, "lhs");
        long long rhs = read_int(ins, "rhs");
        if (!lhs) lhs = read_int(ins, "left");
        if (!rhs) rhs = read_int(ins, "right");
        if (!pred) pred = read_str(ins, "operation");
        if (pred) {
          if (!strcmp(pred, "Lt") || !strcmp(pred, "lt") || !strcmp(pred, "<")) p2 = "slt";
          else if (!strcmp(pred, "Le") || !strcmp(pred, "le") || !strcmp(pred, "<=")) p2 = "sle";
          else if (!strcmp(pred, "Eq") || !strcmp(pred, "eq") || !strcmp(pred, "==")) p2 = "eq";
          else if (!strcmp(pred, "Ne") || !strcmp(pred, "ne") || !strcmp(pred, "!=")) p2 = "ne";
          else if (!strcmp(pred, "Ge") || !strcmp(pred, "ge") || !strcmp(pred, ">=")) p2 = "sge";
          else if (!strcmp(pred, "Gt") || !strcmp(pred, "gt") || !strcmp(pred, ">")) p2 = "sgt";
        }
        if (!p2) return 0;
        if (should_emit_string_compare(ins, lhs, rhs)) {
          emit_string_compare(dst, p2, lhs, rhs);
        } else {
          emit_icmp(dst, p2, lhs, rhs);
        }
        return 1;
      }

      auto int module_generic_string_emit_select(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        long long cond = read_int(ins, "cond");
        long long then_val = read_int(ins, "then_val");
        long long else_val = read_int(ins, "else_val");
        long long cond_cv = 0;
        long long then_cv = 0;
        long long else_cv = 0;
        int cond_ty = get_type(cond);
        int then_ty = get_type(then_val);
        int else_ty = get_type(else_val);
        int val_ty = T_I64;
        long long cond_base = resolve_copy_source_reg(cond);
        long long then_base = resolve_copy_source_reg(then_val);
        long long else_base = resolve_copy_source_reg(else_val);
        int cond_const = has_const(cond_base, &cond_cv);
        int then_const = has_const(then_base, &then_cv);
        int else_const = has_const(else_base, &else_cv);
        int then_org = get_origin(then_val);
        int else_org = get_origin(else_val);
        int then_scan = get_scan_origin(then_val);
        int else_scan = get_scan_origin(else_val);
        char cond_ref[64];
        char then_ref[64];
        char else_ref[64];

        if (!(dst && cond && then_val && else_val)) return 0;
        if (then_ty == T_I1 && else_ty == T_I1) val_ty = T_I1;
        else if (then_ty != T_NONE && then_ty == else_ty) val_ty = then_ty;
        else if (then_ty != T_NONE && else_ty == T_NONE) val_ty = then_ty;
        else if (else_ty != T_NONE && then_ty == T_NONE) val_ty = else_ty;

        if (cond_ty == T_I1) {
          if (cond_const) snprintf(cond_ref, sizeof(cond_ref), "%s", cond_cv ? "true" : "false");
          else snprintf(cond_ref, sizeof(cond_ref), "%%r%lld", cond_base);
        } else if (cond_const) {
          EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", dst, cond_cv);
          snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
        } else {
          EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", dst, cond_base);
          snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
        }

        if (val_ty == T_I1) {
          if (then_const) snprintf(then_ref, sizeof(then_ref), "%s", then_cv ? "true" : "false");
          else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
          if (else_const) snprintf(else_ref, sizeof(else_ref), "%s", else_cv ? "true" : "false");
          else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
          EMIT("  %%r%lld = select i1 %s, i1 %s, i1 %s\n",
               dst,
               cond_ref,
               then_ref,
               else_ref);
          set_type(dst, T_I1);
        } else {
          if (then_const) snprintf(then_ref, sizeof(then_ref), "%lld", then_cv);
          else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
          if (else_const) snprintf(else_ref, sizeof(else_ref), "%lld", else_cv);
          else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
          EMIT("  %%r%lld = select i1 %s, i64 %s, i64 %s\n",
               dst,
               cond_ref,
               then_ref,
               else_ref);
          set_type(dst, T_I64);
        }
        if (then_org != ORG_NONE && then_org == else_org) set_origin(dst, then_org);
        if (then_scan != ORG_NONE && then_scan == else_scan) set_scan_origin(dst, then_scan);
        return 1;
      }

      auto int module_generic_string_emit_method_len_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv) {
        char ab[256];
        int is_string_len = 0;
        int is_array_len = 0;
        const char* helper_symbol = NULL;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (method_view) {
          is_string_len = module_generic_string_method_view_is_direct_string_len(method_view);
          is_array_len = module_generic_string_method_view_is_direct_array_len(method_view);
        }
        if (!(is_string_len || is_array_len)) {
          return 0;
        }
        helper_symbol = is_array_len ? "nyash.array.slot_len_h" : "nyash.string.len_fast_h";
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        module_generic_string_emit_i64_call(dst, helper_symbol, ab);
        if (dst) {
          if (is_string_len) remember_string_length_call(dst, recv);
        }
        return 1;
      }

      auto int module_generic_string_emit_method_substring_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        char recv_ab[128];
        long long start = 0;
        long long end = 0;
        long long recorded_end = 0;
        size_t arg_count = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        (void)f;
        ab[0] = '\0';
        recv_ab[0] = '\0';
        if (!(method_view &&
              module_generic_string_method_view_is_direct_string_substring(method_view))) {
          return 0;
        }
        if (!(arg_count == 1 || arg_count == 2)) return 0;
        if (!module_generic_string_read_arg_reg(args, 0, &start)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), start, 0);
        if (arg_count == 2) {
          if (!module_generic_string_read_arg_reg(args, 1, &end)) return 0;
          append_i64_arg_ref(ab, sizeof(ab), end, 0);
          recorded_end = end;
        } else {
          size_t used = strlen(ab);
          append_i64_arg_ref(recv_ab, sizeof(recv_ab), recv, 1);
          EMIT("  %%substr_end_%lld_%zu = call i64 @\"nyash.string.len_fast_h\"(%s)\n", bid, ii, recv_ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(
              ab + used,
              sizeof(ab) - used,
              ", i64 %%substr_end_%lld_%zu",
              bid,
              ii);
        }
        module_generic_string_emit_i64_call(dst, "nyash.string.substring_hii", ab);
        if (dst) {
          module_generic_string_publish_origin(dst, ORG_STRING);
          remember_string_substring_call(dst, recv, start, recorded_end);
        }
        return 1;
      }

      auto int module_generic_string_emit_method_push_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        long long value = 0;
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              module_generic_string_method_view_is_direct_array_push(method_view))) {
          return 0;
        }
        if (!module_generic_string_read_array_push_value(args, &value)) return 0;
        module_generic_string_emit_array_slot_append_call(dst, recv, value);
        module_generic_string_promote_array_string_push(recv, value);
        return 1;
      }

      auto int module_generic_string_emit_method_get_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        struct ModuleGenericStringGetRouteView route;
        const char* helper_symbol = NULL;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!module_generic_string_read_get_route_view(method_view, &route)) return 0;
        helper_symbol = route.is_array_load
                            ? "nyash.array.slot_load_hi"
                            : "nyash.runtime_data.get_hh";
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1)) return 0;
        if (!module_generic_string_read_arg_reg(args, 0, &key)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        module_generic_string_emit_i64_call(dst, helper_symbol, ab);
        if (dst) {
          if (route.is_module_function_array_item_get) {
            module_generic_string_publish_origin(dst, ORG_MAP_BIRTH);
          }
          if (route.is_phi_incoming_array_item_get) {
            module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
          }
          if (route.publishes_map_get_origin) {
            set_origin(dst, ORG_MAP_GET);
          }
          if (route.is_inst_field_get) {
            module_generic_string_publish_inst_field_origin(
                dst, method_view, 0);
          }
          if (route.is_function_field_get) {
            module_generic_string_publish_function_field_origin(
                dst, method_view);
          }
          if (route.is_module_field_get) {
            module_generic_string_publish_module_field_origin(
                dst, method_view);
          }
        }
        return 1;
      }

      auto int module_generic_string_emit_method_keys_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view &&
              module_generic_string_method_view_is_direct_mir_json_flags_keys(method_view))) {
          return 0;
        }
        if (args && yyjson_is_arr(args) && yyjson_arr_size(args) != 0) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        module_generic_string_emit_i64_call(dst, "nyash.map.keys_h", ab);
        if (dst) {
          module_generic_string_publish_origin(dst, ORG_ARRAY_BIRTH);
        }
        return 1;
      }

      auto int module_generic_string_emit_method_set_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        long long value = 0;
        size_t argc = 0;
        size_t key_index = 0;
        size_t value_index = 1;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view &&
              module_generic_string_method_view_is_direct_map_set(method_view))) {
          return 0;
        }
        if (!(args && yyjson_is_arr(args))) return 0;
        argc = yyjson_arr_size(args);
        if (argc == 3) {
          key_index = 1;
          value_index = 2;
        } else if (argc != 2) {
          return 0;
        }
        if (!module_generic_string_read_arg_reg(args, key_index, &key)) return 0;
        if (!module_generic_string_read_arg_reg(args, value_index, &value)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        append_i64_arg_ref(ab, sizeof(ab), value, 0);
        module_generic_string_emit_i64_call(dst, "nyash.map.slot_store_hhh", ab);
        return 1;
      }

      auto int module_generic_string_emit_method_indexof_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        long long needle = 0;
        long long start = 0;
        size_t arg_count = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        (void)f;
        if (!(method_view &&
              module_generic_string_method_view_is_direct_string_indexof(method_view))) {
          return 0;
        }
        if (!(arg_count == 1 || arg_count == 2)) return 0;
        if (arg_count == 1) {
          return module_generic_string_emit_string_needle_i64_call(
              dst,
              recv,
              args,
              "nyash.string.indexOf_hh");
        }
        if (!module_generic_string_read_arg_reg(args, 0, &needle)) return 0;
        if (!module_generic_string_read_arg_reg(args, 1, &start)) return 0;
        {
          char recv_ab[128];
          char sub_ab[256];
          char rel_ref[64];
          char start_ref[64];
          recv_ab[0] = '\0';
          sub_ab[0] = '\0';
          append_i64_arg_ref(recv_ab, sizeof(recv_ab), recv, 1);
          append_i64_arg_ref(sub_ab, sizeof(sub_ab), recv, 1);
          append_i64_arg_ref(sub_ab, sizeof(sub_ab), start, 0);
          EMIT("  %%indexof_len_%lld_%zu = call i64 @\"nyash.string.len_fast_h\"(%s)\n",
               bid,
               ii,
               recv_ab);
          {
            size_t used = strlen(sub_ab);
            if (used >= sizeof(sub_ab)) return 0;
            snprintf(
                sub_ab + used,
                sizeof(sub_ab) - used,
                ", i64 %%indexof_len_%lld_%zu",
                bid,
                ii);
          }
          EMIT("  %%indexof_tail_%lld_%zu = call i64 @\"nyash.string.substring_hii\"(%s)\n",
               bid,
               ii,
               sub_ab);
          module_generic_string_format_i64_ref(start_ref, sizeof(start_ref), start);
          snprintf(rel_ref, sizeof(rel_ref), "%%indexof_rel_%lld_%zu", bid, ii);
          EMIT("  %s = call i64 @\"nyash.string.indexOf_hh\"(i64 %%indexof_tail_%lld_%zu, i64 %%r%lld)\n",
               rel_ref,
               bid,
               ii,
               resolve_copy_source_reg(needle));
          if (dst) {
            EMIT("  %%indexof_miss_%lld_%zu = icmp slt i64 %s, 0\n", bid, ii, rel_ref);
            EMIT("  %%indexof_abs_%lld_%zu = add i64 %s, %s\n", bid, ii, rel_ref, start_ref);
            EMIT("  %%r%lld = select i1 %%indexof_miss_%lld_%zu, i64 -1, i64 %%indexof_abs_%lld_%zu\n",
                 dst,
                 bid,
                 ii,
                 bid,
                 ii);
            set_type(dst, T_I64);
          }
        }
        return 1;
      }

      auto int module_generic_string_emit_method_lastindexof_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              module_generic_string_method_view_is_direct_string_lastindexof(method_view))) {
          return 0;
        }
        return module_generic_string_emit_string_needle_i64_call(
            dst,
            recv,
            args,
            "nyash.string.lastIndexOf_hh");
      }

      auto int module_generic_string_emit_method_contains_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              module_generic_string_method_view_is_direct_string_contains(method_view))) {
          return 0;
        }
        return module_generic_string_emit_string_needle_i64_call(
            dst,
            recv,
            args,
            "nyash.string.contains_hh");
      }

      auto int module_generic_string_emit_generic_method_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        int rc = module_generic_string_emit_method_len_mir_call(
            f,
            bid,
            ii,
            method_view,
            dst,
            recv);
        if (rc != 1) {
          rc = module_generic_string_emit_method_push_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_get_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_keys_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_set_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_substring_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_indexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_lastindexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = module_generic_string_emit_method_contains_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        return rc;
      }

      auto int module_generic_string_emit_global_call_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst) {
        int rc = emit_global_call_lowering_plan_mir_call(f, bid, ii, dst, args);
        if (rc != 1) {
          long long a0 = 0;
          if (module_generic_string_read_arg_reg(args, 0, &a0)) {
            rc = emit_global_mir_call(
                f,
                classify_mir_call_global_surface(read_str(cal, "name")),
                bid,
                ii,
                a0);
          }
        }
        return rc;
      }

      auto int module_generic_string_emit_method_call_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const char* op,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst) {
        long long recv = module_generic_string_call_receiver(op, cal);
        const char* ctype = module_generic_string_call_type(op, cal);
        const char* mname = module_generic_string_call_name(op, cal);
        const char* bname = module_generic_string_call_box_name(op, cal);
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView method_view;
        const struct LoweringPlanGenericMethodView* method_view_ptr = NULL;
        if (module_generic_string_call_is_array_or_map_birth(
                ctype, mname, bname)) {
          if (dst) {
            put_const(dst, 0);
            set_type(dst, T_I64);
          }
          return 1;
        }
        if (read_lowering_plan_generic_method_view(entry, &method_view)) {
          method_view_ptr = &method_view;
        }
        return module_generic_string_emit_generic_method_mir_call(
            f,
            bid,
            ii,
            method_view_ptr,
            dst,
            recv,
            args);
      }

      auto int module_generic_string_emit_call_instruction(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        long long dst = read_int(ins, "dst");
        int rc = 0;
        if (!module_generic_string_read_call_payload(ins, &cal, &args)) return 0;
        ctype = module_generic_string_call_type(op, cal);
        rc = emit_extern_call_lowering_plan_mir_call(f, bid, ii, dst, args);
        if (rc != 0) {
          return rc;
        }
        if (ctype && !strcmp(ctype, "Extern")) {
          rc = 0;
        } else if (ctype && !strcmp(ctype, "Global")) {
          rc = module_generic_string_emit_global_call_mir_call(
              f,
              bid,
              ii,
              cal,
              args,
              dst);
        } else if (ctype && !strcmp(ctype, "Method")) {
          rc = module_generic_string_emit_method_call_mir_call(
              f,
              bid,
              ii,
              op,
              cal,
              args,
              dst);
        } else {
          return 0;
        }
        return rc == 1;
      }

      auto int module_generic_string_emit_externcall_instruction(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* args = yyjson_obj_get(ins, "args");
        int rc = emit_extern_call_lowering_plan_mir_call(
            f,
            bid,
            ii,
            read_int(ins, "dst"),
            args);
        return rc == 1;
      }

      auto int module_generic_string_emit_ret_instruction(
          long long bid,
          yyjson_val* ins) {
        long long v = read_int(ins, "value");
        long long cv = 0;
        long long base = resolve_copy_source_reg(v);
        char value_ref[64];
        if (has_const(base, &cv)) {
          EMIT("  ret i64 %lld\n", cv);
        } else if (get_type(base) == T_I1) {
          EMIT("  %%ret_zext_%lld_%lld = zext i1 %%r%lld to i64\n", bid, base, base);
          EMIT("  ret i64 %%ret_zext_%lld_%lld\n", bid, base);
        } else {
          module_generic_string_format_i64_ref(value_ref, sizeof(value_ref), v);
          EMIT("  ret i64 %s\n", value_ref);
        }
        return 1;
      }

      auto int module_generic_string_emit_active_body(FILE* f, size_t fn_index) {
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) return 0;
          emit_block_label(block.id);
          for (int pi = 0; pi < phi_n; pi++) {
            if (phis[pi].bid != block.id) continue;
            for (int ri = 0; ri < phis[pi].rec_n; ri++) {
              emit_phi(phis[pi].recs[ri].dst, phis[pi].recs[ri].in, phis[pi].recs[ri].in_n);
            }
          }
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            const char* op = read_str(ins, "op");
            if (!op) return 0;
            if (!strcmp(op, "phi")) continue;
            if (!strcmp(op, "const")) {
              const char* sval = NULL;
              long long ival = 0;
              long long dst = read_int(ins, "dst");
              if (module_generic_string_read_string_const(ins, &sval)) {
                if (!module_generic_string_emit_const_box(f, fn_index, dst)) {
                  note_pure_unsupported_shape(
                      block.id,
                      ii,
                      "const",
                      "module_generic_string_const_missing",
                      NULL);
                  return 0;
                }
              } else if (module_generic_string_read_i64_const(ins, &ival)) {
                put_const(dst, ival);
                set_type(dst, T_I64);
              } else if (module_generic_string_read_void_sentinel_const(ins)) {
                put_const(dst, 0);
                set_type(dst, T_I64);
              } else {
                return 0;
              }
              continue;
            }
            if (!strcmp(op, "copy")) {
              if (!module_generic_string_emit_copy(
                      read_int(ins, "dst"),
                      read_int(ins, "src"))) {
                return 0;
              }
              continue;
            }
            if (!strcmp(op, "newbox")) {
              if (!module_generic_string_emit_newbox(ins)) return 0;
              continue;
            }
            if (!strcmp(op, "compare")) {
              if (!module_generic_string_emit_compare(ins)) return 0;
              continue;
            }
            if (!strcmp(op, "unop")) {
              if (!module_generic_string_emit_unop(ins)) return 0;
              continue;
            }
            if (!strcmp(op, "select")) {
              if (!module_generic_string_emit_select(ins)) return 0;
              continue;
            }
            if (!strcmp(op, "binop")) {
              if (!module_generic_string_emit_binop(ins)) return 0;
              continue;
            }
            if (!strcmp(op, "branch")) {
              emit_branch(read_int(ins, "cond"), read_int(ins, "then"), read_int(ins, "else"));
              continue;
            }
            if (!strcmp(op, "jump")) {
              EMIT("  br label %%bb%lld\n", read_int(ins, "target"));
              continue;
            }
            if (!strcmp(op, "ret")) {
              if (!module_generic_string_emit_ret_instruction(block.id, ins)) return 0;
              continue;
            }
            if (!strcmp(op, "mir_call") || !strcmp(op, "call") || !strcmp(op, "boxcall")) {
              if (!module_generic_string_emit_call_instruction(f, block.id, ii, ins)) return 0;
              continue;
            }
            if (!strcmp(op, "externcall")) {
              if (!module_generic_string_emit_externcall_instruction(
                      f,
                      block.id,
                      ii,
                      ins)) return 0;
              continue;
            }
            if (!strcmp(op, "keepalive") || !strcmp(op, "release_strong")) {
              continue;
            }
            return 0;
          }
        }
        return 1;
      }

      auto int emit_generic_pure_string_function_definition(
          FILE* f,
          yyjson_val* fn,
          size_t fn_index) {
        hako_llvmc_generic_pure_function_view* saved_active_function = active_function;
        yyjson_val* saved_fn0 = fn0;
        yyjson_val* saved_fn_metadata = fn_metadata;
        yyjson_val* saved_blocks = blocks;
        size_t saved_generic_pure_block_count = generic_pure_block_count;
        hako_llvmc_entry_rune_selection saved_selection = selection;
        struct GenericPureFunctionLoweringState saved_lowering_state = lowering_state;
        hako_llvmc_generic_pure_function_view local_function;
        const char* name = fn ? read_str(fn, "name") : NULL;
        int rc = -1;
        if (!name ||
            !llvm_quoted_symbol_name_is_safe(name) ||
            !module_generic_symbol_will_be_defined(name) ||
            module_generic_symbol_was_emitted(name)) {
          return 0;
        }
        if (module_function_is_numeric_i64_leaf(fn) &&
            module_leaf_symbol_will_be_defined(name)) {
          return 0;
        }
        if (!hako_llvmc_read_generic_pure_function_view(
                fn,
                fn_index,
                0,
                &local_function) ||
            !local_function.metadata) {
          return -1;
        }

        active_function = &local_function;
        fn0 = local_function.fn;
        fn_metadata = local_function.metadata;
        blocks = local_function.blocks;
        generic_pure_block_count = local_function.block_count;
        reset_generic_pure_function_lowering_state(&lowering_state);
        reset_string_concat_tracking_state();

        if (!module_generic_string_prepass_active_function()) {
          note_pure_unsupported_shape_detail(
              -1,
              0,
              "module_function",
              "module_generic_prepass_failed",
              NULL,
              NULL,
              name,
              NULL);
          goto MODULE_GENERIC_STRING_RESTORE;
        }
        module_generic_string_emit_const_globals(f, fn_index);
        fprintf(f, "define i64 @\"%s\"(", name);
        {
          yyjson_val* params = yyjson_obj_get(fn, "params");
          size_t argc = params && yyjson_is_arr(params) ? yyjson_arr_size(params) : 0;
          for (size_t ai = 0; ai < argc; ai++) {
            yyjson_val* param = yyjson_arr_get(params, ai);
            long long reg = param ? (long long)yyjson_get_sint(param) : 0;
            fprintf(f, "%si64 %%r%lld", ai ? ", " : "", reg);
          }
        }
        fprintf(f, ") {\n");
        if (!module_generic_string_emit_active_body(f, fn_index)) {
          note_pure_unsupported_shape_detail(
              -1,
              0,
              "module_function",
              "module_generic_body_emit_failed",
              NULL,
              NULL,
              name,
              NULL);
          fprintf(f, "  unreachable\n");
          fprintf(f, "}\n\n");
          goto MODULE_GENERIC_STRING_RESTORE;
        }
        fprintf(f, "}\n\n");
        rc = remember_emitted_module_generic_string_symbol(name) ? 1 : -1;

      MODULE_GENERIC_STRING_RESTORE:
        free_owned_str_consts();
        reset_string_concat_tracking_state();
        lowering_state = saved_lowering_state;
        active_function = saved_active_function;
        fn0 = saved_fn0;
        fn_metadata = saved_fn_metadata;
        blocks = saved_blocks;
        generic_pure_block_count = saved_generic_pure_block_count;
        selection = saved_selection;
        return rc;
      }

      auto int emit_module_function_generic_string_definitions(FILE* f) {
        int emitted = 0;
        for (size_t i = 0; i < planned_module_generic_string_symbols_n; i++) {
          const char* name = planned_module_generic_string_symbols[i];
          size_t fn_index = 0;
          yyjson_val* fn = NULL;
          if (!module_generic_string_find_function_index(name, &fn_index)) continue;
          fn = yyjson_arr_get(program.functions, fn_index);
          if (!fn || fn == program.entry.fn) continue;
          int rc = emit_generic_pure_string_function_definition(f, fn, fn_index);
          if (rc < 0) return -1;
          if (rc > 0) emitted++;
        }
        reset_string_concat_tracking_state();
        return emitted;
      }
