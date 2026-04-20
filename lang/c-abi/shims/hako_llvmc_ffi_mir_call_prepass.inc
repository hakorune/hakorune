      auto void mark_array_string_indexof_need_flags(
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long a0,
          long long dst,
          int scan_org,
          struct GenericPureNeedFlags* needs) {
        struct ArrayTextObserverRoutePlan observer_route;

        (void)blocks;
        (void)ilen;
        (void)insts;
        (void)bname;
        (void)mname;
        (void)recv;
        (void)a0;
        (void)scan_org;
        if (!needs) return;
        if (match_array_text_observer_route_plan(bid, ii, &observer_route)) {
          needs->arr_string_indexof = 1;
        }
        needs->arr_get = 1;
        needs->arr_slot_load = 1;
        if (dst) set_scan_origin(dst, ORG_STRING);
      }

      auto void scan_mir_call_need_flags(
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          yyjson_val* ins,
          struct GenericPureNeedFlags* needs) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;
        const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
        const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;

        if (!needs) return;
        if (ctype && !strcmp(ctype, "Constructor")) {
          long long dst = read_int(ins, "dst");
          enum MirCallNeedKind need_kind =
              classify_mir_call_constructor_need_kind(bname);
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH) {
            if (dst) set_scan_origin(dst, ORG_MAP_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH) {
            if (dst) set_scan_origin(dst, ORG_ARRAY_BIRTH);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          long long recv = cal ? read_int(cal, "receiver") : 0;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          long long a0 = 0;
          long long a1 = 0;
          struct GenericMethodRouteState route;
          enum MirCallReceiverFamilyKind recv_family;
          enum MirCallMethodSurfaceKind method_surface;
          enum MirCallNeedKind need_kind;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }
          route = classify_generic_method_route(bid, ii, bname, mname, recv);
          recv_family = classify_mir_call_receiver_family(route.recv_org);
          method_surface = classify_mir_call_method_surface(mname);
          need_kind = classify_mir_call_method_need_kind(bname, mname, recv_family);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_NONE &&
              route.runtime_string) {
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF) {
              need_kind = HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF;
            } else if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING) {
              need_kind = HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
            }
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF) {
            mark_array_string_indexof_need_flags(
                blocks, bid, ii, ilen, insts, bname, mname, recv, a0,
                read_int(ins, "dst"), route.recv_org, needs);
            return;
          }
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              a0 && get_scan_origin(a0) == ORG_STRING) {
            promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              a1 && get_scan_origin(a1) == ORG_STRING) {
            promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING) {
            if (read_int(ins, "dst")) set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Global")) {
          apply_mir_call_need_kind(needs, classify_mir_call_global_need_kind(mname));
          return;
        }

        if (ctype && !strcmp(ctype, "Extern") && mname) {
          enum MirCallNeedKind need_kind =
              classify_mir_call_extern_need_kind(mname);
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT &&
              read_int(ins, "dst")) {
            set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
        }
      }
