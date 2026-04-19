#include "hako_llvmc_ffi_generic_method_match.inc"
#include "hako_llvmc_ffi_generic_method_has_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_window.inc"
#include "hako_llvmc_ffi_generic_method_len_policy.inc"
#include "hako_llvmc_ffi_generic_method_push_policy.inc"
#include "hako_llvmc_ffi_generic_method_substring_policy.inc"

#include "hako_llvmc_ffi_generic_method_get_lowering.inc"

      auto int emit_array_string_kernel_slot_store_from_const_suffix(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long recv,
          long long idx,
          long long value) {
        long long pair_root = resolve_copy_source_reg(value);
        long long lhs = 0;
        long long rhs = 0;
        long long source_recv = 0;
        long long source_idx = 0;
        const char* rhs_const = NULL;
        char slot_concat_ab[224];
        char slot_store_ab[128];

        (void)ilen;
        (void)insts;
        if (!pair_root) return 0;
        if (!find_deferred_const_suffix_kernel_slot(pair_root, &lhs, &rhs)) return 0;
        rhs_const = get_str_const(rhs);
        if (!(rhs && rhs_const)) return 0;
        int lhs_is_same_array_slot_source =
            find_array_string_get_source(lhs, &source_recv, &source_idx) &&
            resolve_copy_source_reg(source_recv) == resolve_copy_source_reg(recv) &&
            resolve_copy_source_reg(source_idx) == resolve_copy_source_reg(idx);

        fprintf(
            f,
            "  %%arrstr.slotsave.%lld.%zu = call ptr @llvm.stacksave()\n",
            pair_root,
            ii);
        fprintf(
            f,
            "  %%arrstr.slot.%lld.%zu = alloca { i8, ptr, i64, i64 }, align 8\n",
            pair_root,
            ii);
        fprintf(
            f,
            "  store { i8, ptr, i64, i64 } zeroinitializer, ptr %%arrstr.slot.%lld.%zu, align 8\n",
            pair_root,
            ii);

        slot_concat_ab[0] = '\0';
        if (lhs_is_same_array_slot_source) {
          append_i64_arg_ref(slot_concat_ab, sizeof(slot_concat_ab), source_recv, 1);
          append_i64_arg_ref(slot_concat_ab, sizeof(slot_concat_ab), source_idx, 0);
        } else {
          append_i64_arg_ref(slot_concat_ab, sizeof(slot_concat_ab), lhs, 1);
        }
        strncat(
            slot_concat_ab,
            ", ptr getelementptr inbounds (",
            sizeof(slot_concat_ab) - strlen(slot_concat_ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(rhs_const) + 1,
              rhs);
          strncat(
              slot_concat_ab,
              tail,
              sizeof(slot_concat_ab) - strlen(slot_concat_ab) - 1);
        }
        if (lhs_is_same_array_slot_source) {
          fprintf(
              f,
              "  call i64 @\"nyash.array.kernel_slot_concat_his\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              pair_root,
              ii,
              slot_concat_ab);
        } else {
          fprintf(
              f,
              "  call i64 @\"nyash.string.kernel_slot_concat_hs\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              pair_root,
              ii,
              slot_concat_ab);
        }

        slot_store_ab[0] = '\0';
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), recv, 1);
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), idx, 0);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              ", ptr %%arrstr.slot.%lld.%zu",
              pair_root,
              ii);
          strncat(
              slot_store_ab,
              tail,
              sizeof(slot_store_ab) - strlen(slot_store_ab) - 1);
        }

        hako_llvmc_emit_array_slot_store(
            f,
            dst,
            slot_store_ab,
            HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_KERNEL);
        fprintf(
            f,
            "  call void @llvm.stackrestore(ptr %%arrstr.slotsave.%lld.%zu)\n",
            pair_root,
            ii);
        return 1;
      }

      auto int emit_array_string_kernel_slot_store_from_insert_mid(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long recv,
          long long idx,
          long long value) {
        long long insert_root = resolve_copy_source_reg(value);
        long long source = 0;
        long long middle = 0;
        long long split = 0;
        const char* middle_const = NULL;
        char slot_insert_ab[256];
        char slot_store_ab[128];

        (void)ilen;
        (void)insts;
        if (!insert_root) return 0;
        if (!find_deferred_insert_mid_kernel_slot(insert_root, &source, &middle, &split)) return 0;
        middle_const = get_str_const(middle);
        if (!(source && middle && middle_const)) return 0;

        fprintf(
            f,
            "  %%arrstr.slotsave.%lld.%zu = call ptr @llvm.stacksave()\n",
            insert_root,
            ii);
        fprintf(
            f,
            "  %%arrstr.slot.%lld.%zu = alloca { i8, ptr, i64, i64 }, align 8\n",
            insert_root,
            ii);
        fprintf(
            f,
            "  store { i8, ptr, i64, i64 } zeroinitializer, ptr %%arrstr.slot.%lld.%zu, align 8\n",
            insert_root,
            ii);

        slot_insert_ab[0] = '\0';
        append_i64_arg_ref(slot_insert_ab, sizeof(slot_insert_ab), source, 1);
        strncat(
            slot_insert_ab,
            ", ptr getelementptr inbounds (",
            sizeof(slot_insert_ab) - strlen(slot_insert_ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle);
          strncat(
              slot_insert_ab,
              tail,
              sizeof(slot_insert_ab) - strlen(slot_insert_ab) - 1);
        }
        append_i64_arg_ref(slot_insert_ab, sizeof(slot_insert_ab), split, 0);
        fprintf(
            f,
            "  call i64 @\"nyash.string.kernel_slot_insert_hsi\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
            insert_root,
            ii,
            slot_insert_ab);

        slot_store_ab[0] = '\0';
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), recv, 1);
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), idx, 0);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              ", ptr %%arrstr.slot.%lld.%zu",
              insert_root,
              ii);
          strncat(
              slot_store_ab,
              tail,
              sizeof(slot_store_ab) - strlen(slot_store_ab) - 1);
        }

        hako_llvmc_emit_array_slot_store(
            f,
            dst,
            slot_store_ab,
            HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_KERNEL);
        fprintf(
            f,
            "  call void @llvm.stackrestore(ptr %%arrstr.slotsave.%lld.%zu)\n",
            insert_root,
            ii);
        return 1;
      }

      auto int emit_array_string_kernel_slot_store_from_piecewise_subrange(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long recv,
          long long idx,
          long long value) {
        long long substring_root = resolve_copy_source_reg(value);
        long long piecewise_root = 0;
        long long source = 0;
        long long middle = 0;
        long long split = 0;
        long long start = 0;
        long long end = 0;
        const char* middle_const = NULL;
        char slot_piecewise_ab[256];
        char slot_store_ab[128];

        (void)ilen;
        if (!substring_root) return 0;
        if (!find_string_substring_call(substring_root, &piecewise_root, &start, &end)) return 0;
        piecewise_root = resolve_copy_chain_source_reg(insts, ii + 1, piecewise_root);
        start = resolve_copy_chain_source_reg(insts, ii + 1, start);
        end = resolve_copy_chain_source_reg(insts, ii + 1, end);
        if (!find_deferred_piecewise_subrange(piecewise_root, &source, &middle, &split)) return 0;
        middle_const = get_str_const(middle);
        if (!(source && middle && middle_const)) return 0;

        fprintf(
            f,
            "  %%arrstr.slotsave.%lld.%zu = call ptr @llvm.stacksave()\n",
            substring_root,
            ii);
        fprintf(
            f,
            "  %%arrstr.slot.%lld.%zu = alloca { i8, ptr, i64, i64 }, align 8\n",
            substring_root,
            ii);
        fprintf(
            f,
            "  store { i8, ptr, i64, i64 } zeroinitializer, ptr %%arrstr.slot.%lld.%zu, align 8\n",
            substring_root,
            ii);

        slot_piecewise_ab[0] = '\0';
        append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), source, 1);
        strncat(
            slot_piecewise_ab,
            ", ptr getelementptr inbounds (",
            sizeof(slot_piecewise_ab) - strlen(slot_piecewise_ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle);
          strncat(
              slot_piecewise_ab,
              tail,
              sizeof(slot_piecewise_ab) - strlen(slot_piecewise_ab) - 1);
        }
        append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), split, 0);
        append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), start, 0);
        append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), end, 0);
        fprintf(
            f,
            "  call i64 @\"nyash.string.kernel_slot_piecewise_subrange_hsiii\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
            substring_root,
            ii,
            slot_piecewise_ab);

        slot_store_ab[0] = '\0';
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), recv, 1);
        append_i64_arg_ref(slot_store_ab, sizeof(slot_store_ab), idx, 0);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              ", ptr %%arrstr.slot.%lld.%zu",
              substring_root,
              ii);
          strncat(
              slot_store_ab,
              tail,
              sizeof(slot_store_ab) - strlen(slot_store_ab) - 1);
        }

        hako_llvmc_emit_array_slot_store(
            f,
            dst,
            slot_store_ab,
            HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_KERNEL);
        fprintf(
            f,
            "  call void @llvm.stackrestore(ptr %%arrstr.slotsave.%lld.%zu)\n",
            substring_root,
            ii);
        return 1;
      }

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
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY:
                if (dst) {
                  fprintf(f, "  %%r%lld = call i64 @\"nyash.runtime_data.set_hhh\"(%s)\n", dst, ab);
                  set_type(dst, T_I64);
                } else {
                  fprintf(f, "  call i64 @\"nyash.runtime_data.set_hhh\"(%s)\n", ab);
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
                hako_llvmc_emit_array_slot_store(
                    f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_I64);
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
                hako_llvmc_emit_array_slot_store(
                    f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_HANDLE);
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
