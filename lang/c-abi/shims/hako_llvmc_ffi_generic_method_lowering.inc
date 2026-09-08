#include "hako_llvmc_ffi_generic_method_match.inc"
#include "hako_llvmc_ffi_map_lookup_fusion_metadata.inc"
#include "hako_llvmc_ffi_generic_method_has_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_window.inc"
#include "hako_llvmc_ffi_generic_method_len_policy.inc"
#include "hako_llvmc_ffi_generic_method_push_policy.inc"
#include "hako_llvmc_ffi_generic_method_substring_policy.inc"

#include "hako_llvmc_ffi_generic_direct_array_emit.inc"
#include "hako_llvmc_ffi_generic_array_string_slot_store.inc"

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
        if (!classify_generic_method_emit_plan(
                bid,
                ii,
                bname,
                mname,
                route,
                emit_kind,
                a0,
                a1,
                &plan)) {
          return 0;
        }

        switch (emit_kind) {
          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SET: {
            if (a1) (void)generic_pure_publish_reg_metadata(a1);
            int array_set_plain_i64 = a1 && is_plain_i64_value(a1);
            int array_set_string_handle = plan.value_is_string_handle;
            enum GenericMethodSetRouteKind set_route = plan.set_route;
            int map_set_plain_i64 =
                a1 && is_plain_i64_value(a1) &&
                (set_route == HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_I64 ||
                 set_route == HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY);
            (void)array_set_plain_i64;
            (void)array_set_string_handle;
            ab[0] = '\0';
            if (recv) append_i64_arg_ref(ab, ab_cap, recv, 1);
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (map_set_plain_i64) {
              char value_ref[64];
              long long value_const = 0;
              long long value_base = resolve_copy_source_reg(a1);
              size_t used = strlen(ab);
              if (has_const(value_base, &value_const)) {
                snprintf(value_ref, sizeof(value_ref), "%lld", value_const);
              } else {
                snprintf(value_ref, sizeof(value_ref), "%%r%lld", value_base);
              }
              fprintf(f,
                      "  %%map_set_i64_value_box_%lld_%zu = call i64 @\"nyash.box.from_i64\"(i64 %s)\n",
                      bid,
                      ii,
                      value_ref);
              if (used < ab_cap) {
                snprintf(
                    ab + used,
                    ab_cap - used,
                    ", i64 %%map_set_i64_value_box_%lld_%zu",
                    bid,
                    ii);
              }
            } else if (a1) {
              append_i64_arg_ref(ab, ab_cap, a1, 0);
            }
            switch (set_route) {
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_I64:
                if (dst) {
                  fprintf(f, "  %%r%lld = call i64 @\"nyash.map.slot_store_hih\"(%s)\n", dst, ab);
                  set_type(dst, T_I64);
                } else {
                  fprintf(f, "  call i64 @\"nyash.map.slot_store_hih\"(%s)\n", ab);
                }
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY:
                /* owner route: `MapStoreAny`
                 * intended canonical MIR reading: `store.map.value`
                 * phase-151x visibility lock keeps the current concrete lowering
                 * visible instead of promoting this symbol to authority.
                 */
                if (dst) {
                  fprintf(f, "  %%r%lld = call i64 @\"nyash.map.slot_store_hhh\"(%s)\n", dst, ab);
                  set_type(dst, T_I64);
                } else {
                  fprintf(f, "  call i64 @\"nyash.map.slot_store_hhh\"(%s)\n", ab);
                }
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64:
                if (!(hako_llvmc_emit_direct_array_i64_store_range_index(
                          f,
                          bid,
                          ii,
                          dst,
                          recv,
                          a0,
                          a1) ||
                      hako_llvmc_emit_direct_array_i64_store_unchecked(
                          f,
                          bid,
                          ii,
                          dst,
                          recv,
                          a0,
                          a1) ||
                      hako_llvmc_emit_direct_array_i64_store(
                        f,
                        bid,
                        ii,
                        dst,
                        recv,
                        a0,
                        a1))) {
                  hako_llvmc_emit_array_slot_store(
                      f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_I64);
                }
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING:
                /* owner route: `ArrayStoreString`
                 * intended canonical MIR reading: `store.array.str`
                 * lifecycle visibility carried above Rust:
                 *   - source_preserve
                 *   - identity_demand
                 *   - publication_demand
                 * current carrier:
                 *   - GenericMethodRouteState
                 *   - GenericMethodEmitPlan
                 * phase-151x visibility lock keeps the current concrete lowering
                 * visible instead of promoting this symbol to authority.
                 */
                if (emit_array_string_kernel_slot_store_from_piecewise_subrange(
                        f, ii, ilen, insts, dst, recv, a0, a1)) {
                  break;
                }
                if (emit_array_string_kernel_slot_store_from_insert_mid(
                        f, ii, ilen, insts, dst, recv, a0, a1)) {
                  break;
                }
                if (emit_array_string_kernel_slot_store_from_const_suffix(
                        f, ii, ilen, insts, dst, recv, a0, a1)) {
                  break;
                }
                hako_llvmc_emit_array_slot_store(
                    f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_PUBLIC);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY:
                if (!(hako_llvmc_emit_direct_array_i64_store_range_index(
                          f,
                          bid,
                          ii,
                          dst,
                          recv,
                          a0,
                          a1) ||
                      hako_llvmc_emit_direct_array_i64_store_unchecked(
                          f,
                          bid,
                          ii,
                          dst,
                          recv,
                          a0,
                          a1) ||
                      hako_llvmc_emit_direct_array_i64_store(
                        f,
                        bid,
                        ii,
                        dst,
                        recv,
                        a0,
                        a1))) {
                  hako_llvmc_emit_array_slot_store(
                      f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_HANDLE);
                }
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
                plan.recv_org, plan.runtime_array_get, plan.runtime_array_string,
                &plan, ab, ab_cap, skip_inst);
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
                    f, bname, dst, recv, a0, a1, &plan, ab, ab_cap)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING:
          {
            int handled = emit_generic_method_substring_by_policy(
                f, bid, ii, ilen, insts, recv, dst, a0, a1, skip_inst, ab, ab_cap);
            if (handled < 0) {
              return -1;
            }
            if (!handled) {
              return -1;
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS:
          {
            int handled = emit_generic_method_has_by_policy(
                f, bid, ii, bname, dst, recv, a0, &plan, ab, ab_cap);
            if (handled < 0) {
              return -1;
            }
            if (!handled) {
              return -1;
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_KEYS:
          {
            char keys_ab[128];
            keys_ab[0] = '\0';
            append_i64_arg_ref(keys_ab, sizeof(keys_ab), recv, 1);
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.keys_h\"(%s)\n", dst, keys_ab);
              set_type(dst, T_I64);
              set_origin(dst, ORG_ARRAY_BIRTH);
              set_scan_origin(dst, ORG_ARRAY_BIRTH);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.keys_h\"(%s)\n", keys_ab);
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE:
          default:
            return 0;
        }
      }
