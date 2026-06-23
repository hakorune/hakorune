          if (strcmp(op, "variant_make")==0) {
            long long dst = read_int(ins, "dst");
            long long payload = read_int(ins, "payload");
            long long tag = read_int(ins, "tag");
            const char* enum_name = read_str(ins, "enum");
            int boxed_emit = payload
                ? emit_boxed_sum_handle_variant_make(bid, ii, op, dst, tag, payload, enum_name)
                : emit_boxed_sum_unit_variant_make(bid, ii, op, dst, tag, enum_name);
            if (boxed_emit < 0) {
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (boxed_emit > 0) {
              continue;
            }
            continue;
          }
          if (strcmp(op, "variant_tag")==0) {
            long long dst = read_int(ins, "dst");
            long long value = read_int(ins, "value");
            const char* enum_name = read_str(ins, "enum");
            struct GenericPureVariantBinding binding;
            memset(&binding, 0, sizeof(binding));
            if (!get_variant_binding(value, &binding)) {
              int boxed_emit = emit_boxed_sum_tag_read(bid, ii, op, dst, value, enum_name);
              if (boxed_emit < 0) {
                yyjson_doc_free(d);
                goto GEN_ABORT;
              }
              if (boxed_emit > 0) {
                continue;
              }
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "local_variant_tag_binding_missing",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (binding.tag_reg) {
              EMIT("  %%r%lld = add i64 %%r%lld_tag, 0\n", dst, binding.tag_reg);
            } else {
              put_const(dst, binding.tag_const);
            }
            set_type(dst, T_I64);
            continue;
          }
          if (strcmp(op, "variant_project")==0) {
            long long dst = read_int(ins, "dst");
            long long value = read_int(ins, "value");
            long long tag = read_int(ins, "tag");
            const char* enum_name = read_str(ins, "enum");
            struct GenericPureVariantBinding binding;
            memset(&binding, 0, sizeof(binding));
            if (!get_variant_binding(value, &binding) || !binding.has_payload) {
              int boxed_emit =
                  emit_boxed_sum_project_handle(bid, ii, op, dst, value, tag, enum_name);
              if (boxed_emit < 0) {
                yyjson_doc_free(d);
                goto GEN_ABORT;
              }
              if (boxed_emit > 0) {
                continue;
              }
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "local_variant_project_binding_missing",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            set_copy_alias(dst, resolve_copy_source_reg(binding.payload_reg));
            set_type(dst, T_I64);
            continue;
          }
