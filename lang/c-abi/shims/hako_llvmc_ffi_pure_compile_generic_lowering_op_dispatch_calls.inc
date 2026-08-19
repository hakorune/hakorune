          if (strcmp(op, "call")==0) {
            yyjson_val* cal = yyjson_obj_get(ins, "callee");
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal
                ? (read_str(cal, "box_name")
                    ? read_str(cal, "box_name")
                    : read_str(cal, "box_type"))
                : NULL;
            const char* mname = NULL;
            long long dst = read_int(ins, "dst");
            long long recv = cal ? read_int(cal, "receiver") : 0;
            yyjson_val* args = yyjson_obj_get(ins, "args");
            long long a0 = 0, a1 = 0;
            if (!cal) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "legacy_call_missing_structured_callee",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (ctype && (!strcmp(ctype, "Global") || !strcmp(ctype, "Extern"))) {
              mname = read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Method")) {
              mname = read_str(cal, "method")
                  ? read_str(cal, "method")
                  : read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Constructor")) {
              mname = read_str(cal, "name");
            }
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) {
                a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              }
              if (yyjson_arr_size(args) >= 2) {
                a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
              }
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              if (ctype && !strcmp(ctype, "Global")) {
                if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                  note_pure_unsupported_shape_detail(
                      bid,
                      ii,
                      op,
                      "missing_multi_function_emitter",
                      "?",
                      "structured_call_global_user_target",
                      mname,
                      "same_module_global_call_requires_uniform_emitter");
                }
              } else {
                note_pure_unsupported_shape_route_detail(
                    bid,
                    ii,
                    op,
                    "structured_call_no_route",
                    NULL,
                    ctype,
                    mname,
                    NULL,
                    mname,
                    bname,
                    NULL,
                    NULL,
                    "check_callee_route_or_receiver_origin");
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
          if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
            const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            long long dst = read_int(ins, "dst");
            long long recv = read_int(cal, "receiver");
            yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
            long long a0 = 0, a1 = 0;
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                note_pure_unsupported_shape_route_detail(
                    bid,
                    ii,
                    op,
                    "mir_call_no_route",
                    NULL,
                    ctype,
                    NULL,
                    NULL,
                    mname,
                    bname,
                    NULL,
                    NULL,
                    "check_callee_route_or_receiver_origin");
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
