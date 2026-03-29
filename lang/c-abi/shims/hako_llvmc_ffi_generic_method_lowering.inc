#include "hako_llvmc_ffi_generic_method_match.inc"
#include "hako_llvmc_ffi_generic_method_has_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_window.inc"
#include "hako_llvmc_ffi_generic_method_len_policy.inc"
#include "hako_llvmc_ffi_generic_method_push_policy.inc"
#include "hako_llvmc_ffi_generic_method_substring_policy.inc"

#include "hako_llvmc_ffi_generic_method_get_lowering.inc"

      auto int try_emit_non_indexof_generic_method_call(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long dst,
          long long recv,
          long long a0,
          long long a1,
          enum GenericMethodEmitKind emit_kind,
          const struct GenericMethodRouteState* route,
          char* ab,
          size_t ab_cap,
          unsigned char* skip_inst) {
        struct GenericMethodEmitPlan plan;
        if (!classify_generic_method_emit_plan(bname, mname, route, a0, a1, &plan)) return 0;
        int recv_org = plan.recv_org;
        int runtime_array_len = plan.runtime_array_len;
        int runtime_array_get = plan.runtime_array_get;
        int runtime_array_push = plan.runtime_array_push;
        int runtime_array_has = plan.runtime_array_has;
        int runtime_array_string = plan.runtime_array_string;
        int runtime_map_get = plan.runtime_map_get;
        int runtime_map_has = plan.runtime_map_has;

        switch (emit_kind) {
          case HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO:
            if (dst) {
              fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
              set_type(dst, T_I64);
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SET: {
            int array_set_plain_i64 = a1 && is_plain_i64_value(a1);
            int array_set_string_handle = a1 && get_origin(a1) == ORG_STRING;
            enum GenericMethodSetRouteKind set_route =
                classify_generic_method_set_route(
                    bname, &plan, array_set_plain_i64, array_set_string_handle);
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
            switch (set_route) {
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY:
                fprintf(f, "  call i64 @\"nyash.map.slot_store_hhh\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64:
                fprintf(f, "  call i64 @\"nyash.array.slot_store_hii\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING:
                fprintf(f, "  call i64 @\"nyash.array.set_his\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY:
                fprintf(f, "  call i64 @\"nyash.array.set_hih\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE:
              default:
                return -1;
            }
            if (should_promote_array_string_birth_after_set(&plan, array_set_string_handle)) {
              promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              if (recv) {
                set_origin(recv, ORG_ARRAY_STRING_BIRTH);
                set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
              }
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_GET: {
            int handled = emit_generic_method_get_by_window_or_policy(
                f, blocks, bid, ii, ilen, insts, bname, mname, dst, recv, a0,
                recv_org, runtime_array_get, runtime_array_string, &plan, ab,
                ab_cap, skip_inst);
            if (handled < 0) {
              return -1;
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN:
            if (!emit_generic_method_len_by_policy(f, bname, dst, recv, &plan, ab)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH:
            if (!emit_generic_method_push_by_policy(
                    f, bname, dst, recv, a0, &plan, ab, ab_cap)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING:
            if (!emit_generic_method_substring_by_policy(
                    f, ii, ilen, insts, recv, dst, a0, a1, skip_inst, ab, ab_cap)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS:
            if (!emit_generic_method_has_by_policy(
                    f, bname, dst, a0, &plan, ab, ab_cap)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE:
          default:
            return 0;
        }
      }
