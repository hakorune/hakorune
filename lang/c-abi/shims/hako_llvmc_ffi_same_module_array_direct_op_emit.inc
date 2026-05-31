      struct SameModuleArraySlotDirectOpPlan {
        int active;
        size_t get_index;
        size_t set_index;
        long long get_dst;
        long long src_handle;
        long long src_idx;
        long long dst_handle;
        long long store_value;
        long long set_dst;
        long long carrier_dsts[16];
        size_t carrier_count;
      };

      auto int same_module_function_is_selected_array_direct_op_target(long long bid) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               !strcmp(name, "HakoAllocPageModel.acquire_usize/1") &&
               bid == 45;
      }

      auto int same_module_function_instruction_is_array_method(
          yyjson_val* ins,
          const char* expected_method,
          yyjson_val** args_out,
          long long* receiver_out) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* bname = NULL;
        const char* mname = NULL;
        long long recv = 0;
        if (!same_module_function_read_call_payload(ins, &cal, &args)) return 0;
        ctype = same_module_function_call_type(op, cal);
        bname = same_module_function_call_box_name(op, cal);
        mname = same_module_function_call_name(op, cal);
        recv = same_module_function_call_receiver(op, cal);
        if (!(ctype && !strcmp(ctype, "Method") &&
              bname && !strcmp(bname, "ArrayBox") &&
              mname && expected_method && !strcmp(mname, expected_method))) {
          return 0;
        }
        if (args_out) *args_out = args;
        if (receiver_out) *receiver_out = recv;
        return 1;
      }

      auto int same_module_function_array_arg_at(
          yyjson_val* args,
          size_t index,
          long long* out) {
        yyjson_val* val = args && yyjson_is_arr(args) && yyjson_arr_size(args) > index
                            ? yyjson_arr_get(args, index)
                            : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int same_module_function_plan_contains_carrier(
          const struct SameModuleArraySlotDirectOpPlan* plan,
          long long reg) {
        if (!plan || !reg) return 0;
        for (size_t i = 0; i < plan->carrier_count; i++) {
          if (plan->carrier_dsts[i] == reg) return 1;
        }
        return 0;
      }

      auto int same_module_function_plan_add_carrier(
          struct SameModuleArraySlotDirectOpPlan* plan,
          long long reg) {
        if (!plan || !reg) return 0;
        if (same_module_function_plan_contains_carrier(plan, reg)) return 1;
        if (plan->carrier_count >= ARR_LEN(plan->carrier_dsts)) return 0;
        plan->carrier_dsts[plan->carrier_count++] = reg;
        return 1;
      }

      auto int same_module_function_match_array_slot_direct_op_plan(
          long long bid,
          yyjson_val* insts,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleArraySlotDirectOpPlan* plan) {
        if (!plan) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!(insts && skip_inst)) return 0;
        if (!same_module_function_is_selected_array_direct_op_target(bid)) return 0;
        for (size_t gi = 0; gi < ilen; gi++) {
          yyjson_val* get_ins = yyjson_arr_get(insts, gi);
          yyjson_val* get_args = NULL;
          long long get_recv = 0;
          long long get_arg0 = 0;
          long long get_idx = 0;
          long long get_dst = read_int(get_ins, "dst");
          if (!get_dst) continue;
          if (!same_module_function_instruction_is_array_method(
                  get_ins,
                  "get",
                  &get_args,
                  &get_recv)) {
            continue;
          }
          if (!same_module_function_array_arg_at(get_args, 0, &get_arg0)) continue;
          if (yyjson_arr_size(get_args) >= 2) {
            if (!same_module_function_array_arg_at(get_args, 1, &get_idx)) continue;
          } else {
            get_idx = get_arg0;
          }
          memset(skip_inst, 0, ilen);
          plan->get_index = gi;
          plan->get_dst = get_dst;
          plan->src_handle = get_recv ? get_recv : get_arg0;
          plan->src_idx = get_idx;
          plan->carrier_count = 0;
          if (!same_module_function_plan_add_carrier(plan, get_dst)) return 0;

          for (size_t si = gi + 1; si < ilen; si++) {
            yyjson_val* ins = yyjson_arr_get(insts, si);
            const char* op = read_str(ins, "op");
            if (op && !strcmp(op, "copy")) {
              long long src = read_int(ins, "src");
              long long dst = read_int(ins, "dst");
              if (same_module_function_plan_contains_carrier(plan, src)) {
                if (!same_module_function_plan_add_carrier(plan, dst)) return 0;
                skip_inst[si] = 1;
              }
              continue;
            }
            if (op && !strcmp(op, "mir_call")) {
              yyjson_val* set_args = NULL;
              long long set_recv = 0;
              long long set_arg0 = 0;
              long long set_key = 0;
              long long set_value = 0;
              size_t argc = 0;
              if (!same_module_function_instruction_is_array_method(
                      ins,
                      "set",
                      &set_args,
                      &set_recv)) {
                continue;
              }
              argc = set_args && yyjson_is_arr(set_args) ? yyjson_arr_size(set_args) : 0;
              if (argc == 3) {
                if (!same_module_function_array_arg_at(set_args, 0, &set_arg0)) return 0;
                if (!same_module_function_array_arg_at(set_args, 1, &set_key)) return 0;
                if (!same_module_function_array_arg_at(set_args, 2, &set_value)) return 0;
              } else if (argc == 2) {
                if (!same_module_function_array_arg_at(set_args, 0, &set_key)) return 0;
                if (!same_module_function_array_arg_at(set_args, 1, &set_value)) return 0;
              } else {
                return 0;
              }
              if (!same_module_function_plan_contains_carrier(plan, set_key)) {
                continue;
              }
              plan->set_index = si;
              plan->dst_handle = set_recv ? set_recv : set_arg0;
              plan->store_value = set_value;
              plan->set_dst = read_int(ins, "dst");
              plan->active = 1;
              skip_inst[gi] = 1;
              return 1;
            }
          }
        }
        memset(plan, 0, sizeof(*plan));
        memset(skip_inst, 0, ilen);
        return 0;
      }

      auto int same_module_function_emit_array_slot_direct_op(
          const struct SameModuleArraySlotDirectOpPlan* plan) {
        char ab[512];
        if (!(plan && plan->active)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), plan->src_handle, 1);
        append_i64_arg_ref(ab, sizeof(ab), plan->src_idx, 0);
        append_i64_arg_ref(ab, sizeof(ab), plan->dst_handle, 0);
        append_i64_arg_ref(ab, sizeof(ab), plan->store_value, 0);
        same_module_function_emit_i64_call(
            plan->get_dst,
            "nyash.array.slot_load_store_i64_hihi",
            ab);
        for (size_t i = 0; i < plan->carrier_count; i++) {
          long long dst = plan->carrier_dsts[i];
          if (dst == plan->get_dst) continue;
          set_copy_alias(dst, resolve_copy_source_reg(plan->get_dst));
          set_type(dst, T_I64);
        }
        if (plan->set_dst) {
          put_const(plan->set_dst, 1);
          set_type(plan->set_dst, T_I64);
        }
        return 1;
      }
