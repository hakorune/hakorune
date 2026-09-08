      /* A forward/backedge reference is fixed independently of alias cache
       * mutation. Only actual producers define map_original registers. */
      auto int static_v2_original_i64(long long value, char* out, size_t size) {
        const struct HakoStaticV2Index* index = &invocation->static_v2;
        int vi = hako_static_v2_value_find(index, fn0, (uint32_t)value);
        if (vi < 0) {
          value = resolve_copy_source_reg(value);
          vi = hako_static_v2_value_find(index, fn0, (uint32_t)value);
        }
        if (vi < 0) return 0;
        const hako_llvmc_value_projection_v2* row = &index->frame->values[vi];
        if (!row->flags) return -1;
        if (row->action == HAKO_LLVMC_MAP_ACTION_EXACT_BITS) {
          long long bits;
          if (!has_const(value, &bits)) return -1;
          snprintf(out, size, "%lld", bits);
        } else if (row->action == HAKO_LLVMC_MAP_ACTION_FORMAL ||
                   row->action == HAKO_LLVMC_MAP_ACTION_PHI) {
          if (get_type(value) != T_I64) {
            note_pure_unsupported_shape(0, 0, "phi", "static_v2_phi_original_i1_pending", NULL);
            return -1;
          }
          snprintf(out, size, "%%r%lld", value);
        } else {
          snprintf(out, size, "%%map_original_%lld", value);
        }
        return 1;
      }

      auto long long static_v2_phi_original_source(long long value) {
        const struct HakoStaticV2Index* index = &invocation->static_v2;
        int vi = hako_static_v2_value_find(index, fn0, (uint32_t)value);
        for (size_t depth = 0; vi >= 0 && depth < index->frame->value_count; depth++) {
          const hako_llvmc_value_projection_v2* row = &index->frame->values[vi];
          if (!hako_static_v2_is_copy(row->action)) break;
          uint32_t source;
          if (!hako_static_v2_copy_source(row, index->values[vi].instruction, &source)) break;
          value = source;
          vi = hako_static_v2_value_find(index, fn0, source);
        }
        return value;
      }

      auto int static_v2_validate_i1_phi(long long block_id, long long dst,
                                         struct Incoming* in, int in_n) {
        if (!invocation->static_v2.frame) return 1;
        for (int i = 0; i < in_n; i++) {
          long long value = in[i].val_reg;
          long long mapped = resolve_copy_source_reg(value);
          long long constant_bits;
          int direct = hako_static_v2_value_find(&invocation->static_v2, fn0, (uint32_t)value);
          if (direct >= 0 || has_const(mapped, &constant_bits) ||
              hako_static_v2_value_find(&invocation->static_v2, fn0, (uint32_t)mapped) >= 0) {
            long long bits, original = static_v2_phi_original_source(direct >= 0 ? value : mapped);
            long long base = resolve_copy_source_reg(original);
            int source_site = hako_static_v2_value_find(&invocation->static_v2, fn0, (uint32_t)original);
            if (source_site >= 0 && !invocation->static_v2.values[source_site].used) {
              yyjson_val* source = invocation->static_v2.values[source_site].instruction;
              /* Select chooses its original width after resolving arm aliases.
               * A prepass width is not a stable promise for a future Select. */
              if (hako_static_v2_text(yyjson_obj_get(source, "op"), "select")) {
                note_pure_unsupported_shape(block_id, 0, "phi", "static_v2_phi_original_select_pending", NULL);
                return -1;
              }
              if (hako_static_v2_text(yyjson_obj_get(source, "op"), "variant_tag") ||
                  hako_static_v2_text(yyjson_obj_get(source, "op"), "variant_project")) {
                note_pure_unsupported_shape(block_id, 0, "phi", "static_v2_phi_original_alias_pending", NULL);
                return -1;
              }
            }
            int phi_i1 = get_type(dst) == T_I1;
            int source_i1 = get_type(base) == T_I1;
            /* A future selected Operation has not updated the type table yet.
             * Its validated encoding is the emitting owner's physical width. */
            if (source_site >= 0) {
              const hako_llvmc_value_projection_v2* source_row =
                  &invocation->static_v2.frame->values[source_site];
              if (source_row->action == HAKO_LLVMC_MAP_ACTION_OPERATION)
                source_i1 = source_row->encoding == HAKO_LLVMC_MAP_ENCODING_BOOL_I1_ZEXT;
            }
            int constant = has_const(base, &bits);
            if ((constant && phi_i1 && bits != 0 && bits != 1) ||
                (!constant && (source_i1 != phi_i1))) {
              note_pure_unsupported_shape(block_id, 0, "phi", "static_v2_phi_original_width", NULL);
              return -1;
            }
          }
        }
        return 1;
      }

      /* Called only in the existing PHI group; all side values have fixed names
       * at their producer, including aliases on not-yet-emitted backedges. */
      auto int emit_static_v2_phi(long long block_id, long long dst, struct Incoming* in, int in_n) {
        int vi = hako_static_v2_value_find(&invocation->static_v2, fn0, (uint32_t)dst);
        if (vi < 0) return 0;
        const hako_llvmc_value_projection_v2* row = &invocation->static_v2.frame->values[vi];
        if (row->action != HAKO_LLVMC_MAP_ACTION_PHI) return -1;
        yyjson_val* ins = invocation->static_v2.values[vi].instruction;
        yyjson_val* source_inputs = hako_static_v2_phi_inputs(ins);
        if (yyjson_arr_size(source_inputs) != (size_t)in_n) return -1;
        for (int i = 0; i < in_n; i++) {
          uint32_t value, pred;
          if (!hako_static_v2_phi_input(yyjson_arr_get(source_inputs, (size_t)i), &value, &pred) ||
              in[i].val_reg != value || in[i].pred != pred) return -1;
        }
        for (int lane = 0; lane < 2; lane++) {
          EMIT("  %%map_%s_%lld = phi %s ", lane ? "payload" : "kind", dst, lane ? "i64" : "i32");
          for (int i = 0; i < in_n; i++) {
            char value[64], predecessor[128];
            int ok = lane ? static_v2_payload((uint32_t)in[i].val_reg, value, sizeof(value))
                : static_v2_kind((uint32_t)in[i].val_reg, value, sizeof(value));
            if (!ok) return -1;
            EMIT("%s[ %s, %s ]", i ? ", " : "", value,
                phi_predecessor_label(in[i].pred, block_id, predecessor, sizeof(predecessor)));
          }
          EMIT("\n");
        }
        if (!static_v2_consume_value(ins)) return -1;
        return row->flags ? 0 : 1;
      }

      auto int static_v2_select_map_only(yyjson_val* ins) {
        uint32_t dst;
        int vi = hako_static_v2_u32(yyjson_obj_get(ins, "dst"), &dst)
            ? hako_static_v2_value_find(&invocation->static_v2, fn0, dst) : -1;
        return vi >= 0 && invocation->static_v2.frame->values[vi].action == HAKO_LLVMC_MAP_ACTION_SELECT &&
            !invocation->static_v2.frame->values[vi].flags;
      }

      auto int emit_static_v2_select(yyjson_val* ins, const char* condition) {
        uint32_t dst;
        int vi = hako_static_v2_u32(yyjson_obj_get(ins, "dst"), &dst)
            ? hako_static_v2_value_find(&invocation->static_v2, fn0, dst) : -1;
        if (vi < 0) return 1;
        const hako_llvmc_value_projection_v2* row = &invocation->static_v2.frame->values[vi];
        if (row->action != HAKO_LLVMC_MAP_ACTION_SELECT) return 0;
        uint32_t inputs[16];
        if (hako_static_v2_value_inputs(row, ins, inputs) != 2) return 0;
        for (int lane = 0; lane < 2; lane++) {
          char refs[2][64];
          for (int i = 0; i < 2; i++) {
            int ok = lane ? static_v2_payload(inputs[i], refs[i], sizeof(refs[i]))
                : static_v2_kind(inputs[i], refs[i], sizeof(refs[i]));
            if (!ok) return 0;
          }
          const char* type = lane ? "i64" : "i32";
          EMIT("  %%map_%s_%u = select i1 %s, %s %s, %s %s\n",
              lane ? "payload" : "kind", dst, condition, type, refs[0], type, refs[1]);
        }
        return static_v2_consume_value(ins);
      }
