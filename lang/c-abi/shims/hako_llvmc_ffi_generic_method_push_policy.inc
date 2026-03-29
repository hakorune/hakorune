      enum GenericMethodPushRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY,
      };

      auto enum GenericMethodPushRouteKind classify_generic_method_push_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if ((bname && !strcmp(bname, "ArrayBox")) ||
            (plan && (plan->runtime_array_push || plan->runtime_array_string))) {
          return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_NONE;
      }

      auto int should_promote_array_string_birth_after_push(
          const struct GenericMethodEmitPlan* plan,
          int value_is_string_handle) {
        if (!plan || !value_is_string_handle) return 0;
        return plan->runtime_array_push || plan->runtime_array_string;
      }

      auto int emit_generic_method_push_by_policy(
          FILE* f,
          const char* bname,
          long long dst,
          long long recv,
          long long a0,
          const struct GenericMethodEmitPlan* plan,
          char* ab,
          size_t ab_cap) {
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        switch (classify_generic_method_push_route(bname, plan)) {
          case HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_ARRAY_APPEND_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_append_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_append_hh\"(%s)\n", ab);
            }
            if (should_promote_array_string_birth_after_push(
                    plan, a0 && get_origin(a0) == ORG_STRING)) {
              promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              if (recv) {
                set_origin(recv, ORG_ARRAY_STRING_BIRTH);
                set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
              }
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_NONE:
          default:
            return 0;
        }
      }
