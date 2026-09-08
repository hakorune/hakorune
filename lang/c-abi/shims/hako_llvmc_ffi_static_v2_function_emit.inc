      /* The signature and each exact incoming call read the same body-ordered
       * Formal layout. Original lanes remain independent from Map payloads. */
      auto int emit_static_v2_expanded_call(
          const hako_llvmc_published_static_method_call_v1* row, yyjson_val* args, long long dst) {
        struct HakoStaticV2Index* index = &invocation->static_v2;
        int ei = hako_static_v2_expanded_target(index, row->target_symbol);
        if (ei < 0) return 0;
        yyjson_val* target = index->expanded[ei].function;
        yyjson_val* params = yyjson_obj_get(target, "params");
        for (size_t ai = 0; ai < row->arity; ai++) {
          uint32_t actual, formal;
          if (!hako_static_v2_u32(yyjson_arr_get(args, ai), &actual) ||
              !hako_static_v2_u32(yyjson_arr_get(params, ai), &formal)) return -1;
          int vi = hako_static_v2_value_find(index, target, formal);
          int original = vi < 0 || index->frame->values[vi].flags;
          long long base = resolve_copy_source_reg(actual), bits;
          if (original && !has_const(base, &bits) && get_type(base) == T_I1)
            EMIT("  %%map_actual_%u_%u_%zu = zext i1 %%r%lld to i64\n",
                row->block_id, row->instruction_index, ai, base);
        }
        if (dst) EMIT("  %%r%lld = ", dst);
        else EMIT("  ");
        EMIT("call i64 @\"%s\"(", index->frame->expanded_functions[ei].internal_target);
        int first = 1;
        for (size_t ai = 0; ai < row->arity; ai++) {
          uint32_t actual, formal;
          if (!hako_static_v2_u32(yyjson_arr_get(args, ai), &actual) ||
              !hako_static_v2_u32(yyjson_arr_get(params, ai), &formal)) return -1;
          int vi = hako_static_v2_value_find(index, target, formal);
          if (vi >= 0) {
            char kind[64], payload[64];
            if (!static_v2_kind(actual, kind, sizeof(kind)) ||
                !static_v2_payload(actual, payload, sizeof(payload))) return -1;
            EMIT("%si32 %s, i64 %s", first ? "" : ", ", kind, payload);
            first = 0;
          }
          if (vi < 0 || index->frame->values[vi].flags) {
            long long base = resolve_copy_source_reg(actual), bits;
            EMIT("%si64 ", first ? "" : ", ");
            if (has_const(base, &bits)) EMIT("%lld", bits);
            else if (get_type(base) == T_I1)
              EMIT("%%map_actual_%u_%u_%zu", row->block_id, row->instruction_index, ai);
            else EMIT("%%r%lld", base);
            first = 0;
          }
        }
        EMIT(")\n");
        if (dst) set_type(dst, T_I64);
        return 1;
      }

      auto int emit_static_v2_expanded_signature(yyjson_val* fn) {
        struct HakoStaticV2Index* index = &invocation->static_v2;
        int ei = hako_static_v2_expanded_find(index, fn);
        if (ei < 0) return 0;
        if (fn == program->entry.fn || index->expanded[ei].used) return -1;
        yyjson_val* params = yyjson_obj_get(fn, "params");
        EMIT("define internal i64 @\"%s\"(", index->frame->expanded_functions[ei].internal_target);
        int first = 1;
        for (size_t ai = 0; ai < yyjson_arr_size(params); ai++) {
          uint32_t formal;
          if (!hako_static_v2_u32(yyjson_arr_get(params, ai), &formal)) return -1;
          int vi = hako_static_v2_value_find(index, fn, formal);
          if (vi >= 0) {
            EMIT("%si32 %%map_kind_%u, i64 %%map_payload_%u", first ? "" : ", ", formal, formal);
            first = 0;
            index->values[vi].used = 1;
          }
          if (vi < 0 || index->frame->values[vi].flags) {
            EMIT("%si64 %%r%u", first ? "" : ", ", formal);
            first = 0;
          }
        }
        EMIT(") alwaysinline {\n");
        index->expanded[ei].used = 1;
        return 1;
      }
