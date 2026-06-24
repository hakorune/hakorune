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
        if (scan_mir_extern_call_need_flags_from_lowering_plan(
                bid, ii, ins, needs)) {
          return;
        }
        if (ctype && !strcmp(ctype, "Constructor")) {
          long long dst = read_int(ins, "dst");
          if (mark_constructor_call_from_lowering_plan(bid, ii, dst, needs)) {
            return;
          }
          switch (classify_mir_call_constructor_surface(
              cal ? (read_str(cal, "box_name")
                  ? read_str(cal, "box_name")
                  : read_str(cal, "box_type"))
                  : NULL)) {
            case HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_MAP_BOX:
              apply_mir_call_need_kind(needs, HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH);
              if (dst) {
                set_scan_origin(dst, ORG_MAP_BIRTH);
                set_origin(dst, ORG_MAP_BIRTH);
              }
              return;
            case HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_ARRAY_BOX:
              apply_mir_call_need_kind(needs, HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH);
              if (dst) {
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
                set_origin(dst, ORG_ARRAY_BIRTH);
              }
              return;
            case HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_NONE:
            default:
              break;
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          scan_mir_method_call_need_flags_from_lowering_plan(
              bid, ii, ins, cal, mc, needs);
          return;
        }

        if (ctype && !strcmp(ctype, "Global")) {
          scan_mir_global_call_need_flags_from_lowering_plan(
              bid, ii, ins, needs);
          return;
        }

      }
