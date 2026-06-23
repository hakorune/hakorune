          if (strcmp(op, "variant_make")==0) {
            long long dst = read_int(ins, "dst");
            long long payload = read_int(ins, "payload");
            long long tag = read_int(ins, "tag");
            const char* enum_name = read_str(ins, "enum");
            const char* payload_storage = read_str(ins, "boxed_sum_payload_storage");
            int boxed_plan_index = -1;
            boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
            int boxed_emit = emit_boxed_sum_variant_make_lowering(
                bid, ii, op, dst, tag, payload, enum_name, payload_storage, boxed_plan_index);
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
            int boxed_plan_index = -1;
            boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
            int boxed_emit = emit_boxed_sum_variant_tag_lowering(
                bid,
                ii,
                op,
                dst,
                value,
                enum_name,
                boxed_plan_index,
                "local_variant_tag_binding_missing",
                1);
            if (boxed_emit < 0) {
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (boxed_emit > 0) {
              continue;
            }
            continue;
          }
          if (strcmp(op, "variant_project")==0) {
            long long dst = read_int(ins, "dst");
            long long value = read_int(ins, "value");
            long long tag = read_int(ins, "tag");
            const char* enum_name = read_str(ins, "enum");
            const char* payload_storage = read_str(ins, "boxed_sum_payload_storage");
            int boxed_plan_index = -1;
            boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
            int boxed_emit = emit_boxed_sum_variant_project_lowering(
                bid,
                ii,
                op,
                dst,
                value,
                tag,
                enum_name,
                payload_storage,
                boxed_plan_index,
                "local_variant_project_binding_missing",
                1);
            if (boxed_emit < 0) {
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (boxed_emit > 0) {
              continue;
            }
            continue;
          }
