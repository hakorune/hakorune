      /* Same-module prepass: publish backend-local reg/origin facts from MIR-owned metadata. */

      auto int same_module_function_prepass_generic_method_view(
          const char* op,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst,
          const struct LoweringPlanGenericMethodView* method_view) {
        struct SameModuleFunctionGetRouteView get_route;
        if (!method_view) return 0;
        if (same_module_method_view_is_direct_string_len(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_array_len(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_array_push(method_view)) {
          long long recv = same_module_function_call_receiver(op, cal);
          long long value = 0;
          if (dst) set_type(dst, T_I64);
          if (!same_module_function_read_array_push_value(args, recv, &value)) return -1;
          same_module_function_promote_array_string_push(recv, value);
          return 1;
        }
        if (same_module_method_view_is_direct_mir_json_flags_keys(method_view) ||
            same_module_method_view_is_direct_map_keys(method_view)) {
          same_module_function_publish_i64_origin(dst, ORG_ARRAY_BIRTH);
          return 1;
        }
        if (same_module_function_read_get_route_view(method_view, &get_route)) {
          set_type(dst, T_I64);
          if (get_route.is_module_function_array_item_get) {
            same_module_function_publish_origin(dst, ORG_MAP_BIRTH);
          } else if (get_route.is_phi_incoming_array_item_get) {
            same_module_function_publish_origin(dst, ORG_ARRAY_BIRTH);
          } else if (get_route.prepass_publishes_map_get_origin) {
            set_origin(dst, ORG_MAP_GET);
          }
          if (!same_module_function_publish_result_origin_box(
                  dst, get_route.result_origin_box)) {
            return 0;
          }
          return 1;
        }
        if (same_module_method_view_is_direct_set(method_view)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_map_delete(method_view)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_has(method_view)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_string_substring(method_view)) {
          same_module_function_publish_i64_origin(dst, ORG_STRING);
          return 1;
        }
        if (same_module_method_view_is_direct_string_indexof(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_string_lastindexof(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (same_module_method_view_is_direct_string_contains(method_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        return 0;
      }

      auto int same_module_function_prepass_extern_call_view(
          const struct LoweringPlanExternCallView* extern_view,
          long long dst) {
        int origin_kind = ORG_NONE;
        if (!extern_view) return 0;
        origin_kind = lowering_plan_extern_call_view_result_origin_kind(extern_view);
        if (origin_kind != ORG_NONE) {
          same_module_function_publish_i64_origin(dst, origin_kind);
          return 1;
        }
        if (lowering_plan_extern_call_view_returns_scalar_i64(extern_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        if (lowering_plan_extern_call_view_returns_native_ptr_nullable_transport_i64(
                extern_view) ||
            lowering_plan_extern_call_view_returns_void_sentinel_i64_zero(
                extern_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        return 0;
      }

      auto int global_call_definition_prepass_view(
          long long dst,
          const struct LoweringPlanGlobalCallView* global_view) {
        if (global_view &&
            lowering_plan_global_call_view_is_stage1_emit_program_json(global_view)) {
          set_type(dst, T_I64);
          same_module_function_publish_result_box_or_reg_metadata(
              dst,
              global_view->target_result_box_name);
          mark_global_call_result_origin_from_view(global_view, dst);
          return 1;
        }
        if (global_view &&
            lowering_plan_global_call_view_requires_same_module_function_definition(global_view)) {
          set_type(dst, T_I64);
          same_module_function_publish_result_box_or_reg_metadata(
              dst,
              global_view->target_result_box_name);
          mark_global_call_result_origin_from_view(global_view, dst);
          return 1;
        }
        if (global_view &&
            lowering_plan_global_call_view_uses_leaf_i64_definition(global_view)) {
          set_type(dst, T_I64);
          return 1;
        }
        return 0;
      }

      auto int same_module_function_prepass_print_global_call_surface(
          const char* ctype,
          const char* mname,
          yyjson_val* args) {
        return ctype && !strcmp(ctype, "Global") &&
               classify_mir_call_global_surface(mname) ==
                   HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT &&
               args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1;
      }

      auto int same_module_function_prepass_global_call_view(
          const char* ctype,
          const char* mname,
          yyjson_val* args,
          long long dst,
          const struct LoweringPlanGlobalCallView* global_view) {
        if (global_call_definition_prepass_view(dst, global_view)) {
          return 1;
        }
        if (same_module_function_prepass_print_global_call_surface(
                ctype,
                mname,
                args)) {
          return 1;
        }
        return 0;
      }

      auto int same_module_function_prepass_call_instruction(
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
        struct LoweringPlanUserBoxMethodView user_box_method_view;
        const char* ctype = NULL;
        const char* mname = NULL;
        const char* bname = NULL;
        int has_method_view = 0;
        int has_extern_view = 0;
        int has_global_view = 0;
        int has_user_box_method_view = 0;
        int method_prepass_rc = 0;
        long long dst = read_int(ins, "dst");
        if (!same_module_function_read_call_payload(ins, &cal, &args)) return 0;
        ctype = same_module_function_call_type(op, cal);
        mname = same_module_function_call_name(op, cal);
        bname = same_module_function_call_box_name(op, cal);
        has_method_view = read_lowering_plan_generic_method_view(entry, &method_view);
        has_extern_view = read_lowering_plan_extern_call_view(entry, &extern_view);
        has_global_view = read_lowering_plan_global_call_view(entry, &global_view);
        has_user_box_method_view =
            read_lowering_plan_user_box_method_view(entry, &user_box_method_view);
        if (same_module_function_call_is_array_or_map_birth(
                ctype, mname, bname)) {
          if (dst) set_type(dst, T_I64);
          return 1;
        }
        if (has_user_box_method_view &&
            (lowering_plan_user_box_method_view_is_direct_void_sentinel_i64_zero(
                 &user_box_method_view) ||
             lowering_plan_user_box_method_view_is_direct_scalar_i64(
                 &user_box_method_view) ||
             lowering_plan_user_box_method_view_is_direct_runtime_i64_or_handle(
                 &user_box_method_view))) {
          if (dst) set_type(dst, T_I64);
          if (dst) {
            int origin_kind =
                lowering_plan_user_box_method_view_result_origin_kind(&user_box_method_view);
            int typed_plan_index = -1;
            if (user_box_method_view.target_result_box_name &&
                find_typed_object_plan_index(
                    user_box_method_view.target_result_box_name,
                    &typed_plan_index)) {
              set_origin(dst, ORG_TYPED_OBJECT);
              set_scan_origin(dst, ORG_TYPED_OBJECT);
              set_typed_object_binding(dst, typed_plan_index);
              return 1;
            }
            if (origin_kind != ORG_NONE) {
              set_origin(dst, origin_kind);
              set_scan_origin(dst, origin_kind);
            }
          }
          return 1;
        }
        if (has_method_view) {
          method_prepass_rc = same_module_function_prepass_generic_method_view(
              op,
              cal,
              args,
              dst,
              &method_view);
          if (method_prepass_rc < 0) return 0;
          if (method_prepass_rc > 0) return 1;
        }
        if (has_extern_view &&
            same_module_function_prepass_extern_call_view(&extern_view, dst)) {
          return 1;
        }
        return same_module_function_prepass_global_call_view(
            ctype,
            mname,
            args,
            dst,
            has_global_view ? &global_view : NULL);
      }

      auto int same_module_function_prepass_externcall(
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanExternCallView extern_view;
        long long dst = read_int(ins, "dst");
        if (!read_lowering_plan_extern_call_view(entry, &extern_view)) return 0;
        return same_module_function_prepass_extern_call_view(&extern_view, dst);
      }

      auto int same_module_function_prepass_instruction(
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        const char* op = read_str(ins, "op");
        if (!op) return 0;
        if (!strcmp(op, "const")) {
          long long dst = read_int(ins, "dst");
          const char* sval = NULL;
          long long ival = 0;
          if (same_module_function_read_string_const(ins, &sval)) {
            put_str_const(dst, sval);
            put_known_string_len(dst, (long long)strlen(sval));
            same_module_function_publish_i64_origin(dst, ORG_STRING);
            return 1;
          }
          if (same_module_function_read_i64_const(ins, &ival)) {
            put_const(dst, ival);
            set_type(dst, T_I64);
            return 1;
          }
          if (same_module_function_read_void_sentinel_const(ins)) {
            same_module_function_publish_void_sentinel_const(dst);
            return 1;
          }
          return 0;
        }
        if (!strcmp(op, "copy")) {
          long long dst = read_int(ins, "dst");
          long long src = read_int(ins, "src");
          int src_ty = get_type(src);
          int src_org = get_origin(src);
          int src_scan = get_scan_origin(src);
          if (src) set_copy_alias(dst, resolve_copy_source_reg(src));
          if (src_ty != T_NONE) set_type(dst, src_ty);
          if (src_org != ORG_NONE) set_origin(dst, src_org);
          if (src_scan != ORG_NONE) set_scan_origin(dst, src_scan);
          {
            int typed_plan_index = -1;
            if (get_typed_object_binding(src, &typed_plan_index)) {
              set_typed_object_binding(dst, typed_plan_index);
            }
            {
              struct GenericPureVariantBinding variant_binding;
              memset(&variant_binding, 0, sizeof(variant_binding));
              if (get_variant_binding(src, &variant_binding)) {
                set_variant_binding(
                    dst,
                    variant_binding.tag_const,
                    variant_binding.tag_reg,
                    variant_binding.payload_reg,
                    variant_binding.has_payload);
              }
            }
          }
          return 1;
        }
        if (!strcmp(op, "variant_make")) {
          long long dst = read_int(ins, "dst");
          long long payload = read_int(ins, "payload");
          long long tag = read_int(ins, "tag");
          if (payload) {
            long long payload_base = resolve_copy_source_reg(payload);
            set_copy_alias(dst, payload_base);
            set_variant_binding(dst, tag, 0, payload_base, 1);
          } else {
            put_const(dst, 0);
            set_variant_binding(dst, tag, 0, 0, 0);
          }
          set_type(dst, T_I64);
          return 1;
        }
        if (!strcmp(op, "variant_tag")) {
          set_type(read_int(ins, "dst"), T_I64);
          return 1;
        }
        if (!strcmp(op, "variant_project")) {
          long long dst = read_int(ins, "dst");
          long long value = read_int(ins, "value");
          struct GenericPureVariantBinding binding;
          if (get_variant_binding(value, &binding) && binding.has_payload) {
            set_copy_alias(dst, resolve_copy_source_reg(binding.payload_reg));
          }
          set_type(dst, T_I64);
          return 1;
        }
        if (!strcmp(op, "newbox")) {
          long long dst = read_int(ins, "dst");
          const char* tname = read_str(ins, "type");
          if (tname && !strcmp(tname, "ArrayBox")) {
            same_module_function_publish_i64_origin(dst, ORG_ARRAY_BIRTH);
            return 1;
          }
          if (tname && !strcmp(tname, "MapBox")) {
            same_module_function_publish_i64_origin(dst, ORG_MAP_BIRTH);
            return 1;
          }
          if (same_module_function_publish_typed_object_if_plan(dst, tname)) {
            return 1;
          }
          return 0;
        }
        if (!strcmp(op, "field_set")) {
          long long box_reg = read_int(ins, "box");
          const char* field = read_str(ins, "field");
          int typed_plan_index = -1;
          long long slot = -1;
          if (box_reg &&
              get_typed_object_binding(box_reg, &typed_plan_index) &&
              typed_object_plan_field_runtime_slot(typed_plan_index, field, &slot)) {
            (void)slot;
            return 1;
          }
          return 0;
        }
        if (!strcmp(op, "field_get")) {
          long long dst = read_int(ins, "dst");
          long long box_reg = read_int(ins, "box");
          const char* field = read_str(ins, "field");
          int typed_plan_index = -1;
          long long slot = -1;
          if (dst &&
              box_reg &&
              get_typed_object_binding(box_reg, &typed_plan_index) &&
              typed_object_plan_field_runtime_slot(typed_plan_index, field, &slot)) {
            (void)slot;
            if (!same_module_function_publish_reg_metadata(dst)) {
              set_type(dst, T_I64);
            }
            return 1;
          }
          return 0;
        }
        if (!strcmp(op, "compare")) {
          set_type(read_int(ins, "dst"), T_I1);
          return 1;
        }
        if (!strcmp(op, "unop")) {
          const char* kind = read_str(ins, "op_kind");
          if (!kind) kind = read_str(ins, "operation");
          if (!(kind && (!strcmp(kind, "not") || !strcmp(kind, "Not")))) return 0;
          set_type(read_int(ins, "dst"), T_I1);
          return 1;
        }
        if (!strcmp(op, "phi")) {
          return same_module_function_record_phi(ins, bid);
        }
        if (!strcmp(op, "binop")) {
          long long dst = read_int(ins, "dst");
          long long lhs = read_int(ins, "lhs");
          long long rhs = read_int(ins, "rhs");
          const char* kind = read_str(ins, "op_kind");
          if (!kind) kind = read_str(ins, "operation");
          if (kind &&
              (!strcmp(kind, "+") || !strcmp(kind, "Add")) &&
              (same_module_function_value_is_string(lhs) ||
               same_module_function_value_is_string(rhs))) {
            same_module_function_publish_i64_origin(dst, ORG_STRING);
          } else {
            set_type(dst, T_I64);
          }
          return 1;
        }
        if (!strcmp(op, "mir_call") ||
            !strcmp(op, "call") ||
            !strcmp(op, "boxcall")) {
          return same_module_function_prepass_call_instruction(
              op,
              bid,
              ii,
              ins);
        }
        if (!strcmp(op, "externcall")) {
          return same_module_function_prepass_externcall(bid, ii, ins);
        }
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
          return 1;
        }
        if (!strcmp(op, "branch") ||
            !strcmp(op, "jump") ||
            !strcmp(op, "ret") ||
            !strcmp(op, "keepalive") ||
            !strcmp(op, "release_strong")) {
          return 1;
        }
        return 0;
      }
