/* Allocation emission, lexically owned by the existing active instruction walk. */
          if (strcmp(op, "newbox")==0) {
            long long dst = read_int(ins, "dst");
            int intrinsic = hako_llvmc_published_intrinsic_array_take_v1(
                fn0 ? read_str(fn0, "name") : NULL, bid, ii, ins);
            if (intrinsic < 0) {
              note_pure_unsupported_shape(bid, ii, op, "intrinsic_array_contract", NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            if (intrinsic > 0) {
              const char* symbol = hako_llvmc_direct_array_i64_exact_enabled()
                  ? "nyash.array.direct_i64.birth_h" : "nyash.array.birth_h";
              EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, symbol);
              set_type(dst, T_I64);
              set_origin(dst, ORG_ARRAY_BIRTH);
              set_scan_origin(dst, ORG_ARRAY_BIRTH);
              continue;
            }

            const char* tname = read_str(ins, "type");
            struct GenericPureTypedObjectNewPlanView typed_new_plan;
            enum GenericPureTypedObjectNewPlanStatus typed_new_status =
                GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
            if (tname && strcmp(tname, "DirectArrayI64") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.array.direct_i64.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.array.direct_i64.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "ArrayBox") == 0) {
              const char* birth_symbol = hako_llvmc_direct_array_i64_exact_enabled()
                                             ? "nyash.array.direct_i64.birth_h"
                                             : "nyash.array.birth_h";
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, birth_symbol);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"%s\"()\n", birth_symbol);
              }
              continue;
            }
            if (tname && strcmp(tname, "MapBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_MAP_BIRTH);
                set_scan_origin(dst, ORG_MAP_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.map.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "FileBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_FILEBOX);
                set_scan_origin(dst, ORG_FILEBOX);
              } else {
                EMIT("  call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "StringBox") == 0) {
              yyjson_val* nb_args = yyjson_obj_get(ins, "args");
              yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
              long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
              if (dst && arg0) {
                if (should_skip_hoisted_string_const_box(arg0) &&
                    !string_const_box_was_emitted(arg0) &&
                    !emit_materialized_string_const_box_result(f, arg0)) {
                  note_pure_unsupported_shape(
                      bid,
                      ii,
                      op,
                      "string_const_materialization_failed",
                      NULL);
                  yyjson_doc_free(d);
                  goto GEN_ABORT;
                }
                set_copy_alias(dst, resolve_copy_source_reg(arg0));
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                continue;
              }
            }
            typed_new_status = generic_pure_validate_typed_object_new_plan(
                dst,
                tname,
                &typed_new_plan);
            if (typed_new_status == GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK) {
              EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 %lld)\n",
                   dst,
                   typed_new_plan.type_id,
                   typed_new_plan.field_count);
              generic_pure_publish_typed_object_plan_index(
                  dst,
                  typed_new_plan.typed_plan_index);
              continue;
            }
            if (typed_new_status == GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "typed_object_plan_invalid_new",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            note_pure_unsupported_shape(bid, ii, op, "unsupported_newbox_type", NULL);
            yyjson_doc_free(d); goto GEN_ABORT;
          }
