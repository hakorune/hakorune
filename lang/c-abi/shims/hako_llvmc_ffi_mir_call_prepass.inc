      auto void scan_mir_call_need_flags(
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          yyjson_val* ins,
          struct MirCallNeedFlags* needs) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;

        (void)blocks;
        (void)ilen;
        (void)insts;
        if (!needs) return;
        {
          enum MirCallNeedKind plan_extern_need =
              classify_mir_call_extern_need_kind_from_lowering_plan(bid, ii);
          if (plan_extern_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            apply_mir_call_need_kind(needs, plan_extern_need);
            mark_extern_call_result_origin_from_lowering_plan(
                bid, ii, read_int(ins, "dst"));
            return;
          }
        }
        if (ctype && !strcmp(ctype, "Constructor")) {
          long long dst = read_int(ins, "dst");
          if (mark_constructor_call_from_lowering_plan(bid, ii, dst, needs)) {
            return;
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          scan_mir_method_call_need_flags_from_lowering_plan(
              bid, ii, ins, cal, mc, needs);
          return;
        }

        if (ctype && !strcmp(ctype, "Global")) {
          enum MirCallNeedKind plan_need =
              classify_mir_call_global_need_kind_from_lowering_plan(bid, ii);
          mark_global_call_result_origin_from_lowering_plan(
              bid, ii, read_int(ins, "dst"));
          if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            apply_mir_call_need_kind(needs, plan_need);
          }
          return;
        }

      }
