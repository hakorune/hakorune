#include "hako_llvmc_ffi_generic_method_match.inc"
#include "hako_llvmc_ffi_map_lookup_fusion_metadata.inc"
#include "hako_llvmc_ffi_generic_method_has_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_window.inc"
#include "hako_llvmc_ffi_generic_method_len_policy.inc"
#include "hako_llvmc_ffi_generic_method_push_policy.inc"
#include "hako_llvmc_ffi_generic_method_substring_policy.inc"

      auto void hako_llvmc_format_i64_operand(char* out, size_t out_cap, long long reg) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        if (has_const(base, &cv)) snprintf(out, out_cap, "%lld", cv);
        else snprintf(out, out_cap, "%%r%lld", base);
      }

      auto int hako_llvmc_direct_array_i64_exact_enabled(void) {
        const char* value = getenv("HAKO_ARRAY_SLOT_STORE");
        return value && !strcmp(value, "direct_array_i64_exact");
      }

      auto int hako_llvmc_is_selected_direct_array_i64_store_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long value,
          long long dst) {
        return lowering_plan_direct_array_access_site_matches_checked_i64(
            bid,
            ii,
            "store",
            "direct_array_i64_store",
            recv,
            key,
            value,
            1,
            dst,
            dst ? 1 : 0);
      }

      auto int hako_llvmc_is_selected_direct_array_i64_get_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long dst) {
        return lowering_plan_direct_array_access_site_matches_checked_i64(
            bid,
            ii,
            "load",
            "direct_array_i64_load",
            recv,
            key,
            0,
            0,
            dst,
            1);
      }

      auto int hako_llvmc_is_selected_direct_array_i64_unchecked_get_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long dst) {
        return lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_load_i64(
            bid,
            ii,
            "direct_array_i64_load",
            recv,
            key,
            dst,
            1);
      }

      auto int hako_llvmc_is_selected_direct_array_i64_unchecked_store_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long value,
          long long dst) {
        if (lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_store_i64(
                bid,
                ii,
                "direct_array_i64_store",
                recv,
                key,
                value,
                dst,
                dst ? 1 : 0)) {
          return 1;
        }
        return lowering_plan_direct_array_access_site_matches_proved_unchecked_caller_precondition_store_i64(
            bid,
            ii,
            "direct_array_i64_store",
            recv,
            key,
            value,
            dst,
            dst ? 1 : 0);
      }

      auto int hako_llvmc_is_selected_direct_array_i64_range_index_store_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long value,
          long long dst) {
        (void)recv;
        (void)key;
        (void)value;
        (void)dst;
        return lowering_plan_direct_array_access_site_has_proved_unchecked_range_index_store_i64(
            bid,
            ii,
            "direct_array_i64_store");
      }

      auto int hako_llvmc_emit_direct_array_i64_store_range_index(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key,
          long long value) {
        char recv_ref[64];
        char key_ref[64];
        char value_ref[64];
        if (!hako_llvmc_direct_array_i64_exact_enabled()) return 0;
        if (!hako_llvmc_is_selected_direct_array_i64_range_index_store_target(
                bid, ii, recv, key, value, dst)) return 0;

        hako_llvmc_format_i64_operand(recv_ref, sizeof(recv_ref), recv);
        hako_llvmc_format_i64_operand(key_ref, sizeof(key_ref), key);
        hako_llvmc_format_i64_operand(value_ref, sizeof(value_ref), value);

        fprintf(f, "  %%direct_array_i64_range_index_base_%lld_%zu = and i64 %s, -4\n", bid, ii, recv_ref);
        fprintf(f, "  %%direct_array_i64_range_index_len_addr_%lld_%zu = add i64 %%direct_array_i64_range_index_base_%lld_%zu, 16\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_range_index_len_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_len_%lld_%zu = load i64, ptr %%direct_array_i64_range_index_len_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_byte_index_%lld_%zu = mul i64 %s, 8\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_range_index_data_base_%lld_%zu = add i64 %%direct_array_i64_range_index_base_%lld_%zu, 32\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_value_addr_%lld_%zu = add i64 %%direct_array_i64_range_index_data_base_%lld_%zu, %%direct_array_i64_range_index_byte_index_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_range_index_value_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  store i64 %s, ptr %%direct_array_i64_range_index_value_ptr_%lld_%zu\n", value_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_index_plus_one_%lld_%zu = add i64 %s, 1\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_range_index_len_grows_%lld_%zu = icmp ult i64 %%direct_array_i64_range_index_len_%lld_%zu, %%direct_array_i64_range_index_index_plus_one_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_range_index_next_len_%lld_%zu = select i1 %%direct_array_i64_range_index_len_grows_%lld_%zu, i64 %%direct_array_i64_range_index_index_plus_one_%lld_%zu, i64 %%direct_array_i64_range_index_len_%lld_%zu\n", bid, ii, bid, ii, bid, ii, bid, ii);
        fprintf(f, "  store i64 %%direct_array_i64_range_index_next_len_%lld_%zu, ptr %%direct_array_i64_range_index_len_ptr_%lld_%zu\n", bid, ii, bid, ii);
        if (dst) {
          fprintf(f, "  %%r%lld = add i64 1, 0\n", dst);
          set_type(dst, T_I64);
        }
        return 1;
      }

      auto int hako_llvmc_emit_direct_array_i64_get_unchecked(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key) {
        char recv_ref[64];
        char key_ref[64];
        if (!dst) return 0;
        if (!hako_llvmc_direct_array_i64_exact_enabled()) return 0;
        if (!hako_llvmc_is_selected_direct_array_i64_unchecked_get_target(
                bid, ii, recv, key, dst)) return 0;

        hako_llvmc_format_i64_operand(recv_ref, sizeof(recv_ref), recv);
        hako_llvmc_format_i64_operand(key_ref, sizeof(key_ref), key);

        fprintf(f, "  %%direct_array_i64_unchecked_get_base_%lld_%zu = and i64 %s, -4\n", bid, ii, recv_ref);
        fprintf(f, "  %%direct_array_i64_unchecked_get_byte_index_%lld_%zu = mul i64 %s, 8\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_unchecked_get_data_base_%lld_%zu = add i64 %%direct_array_i64_unchecked_get_base_%lld_%zu, 32\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_unchecked_get_value_addr_%lld_%zu = add i64 %%direct_array_i64_unchecked_get_data_base_%lld_%zu, %%direct_array_i64_unchecked_get_byte_index_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_unchecked_get_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_unchecked_get_value_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%r%lld = load i64, ptr %%direct_array_i64_unchecked_get_value_ptr_%lld_%zu\n", dst, bid, ii);
        set_type(dst, T_I64);
        return 1;
      }

      auto int hako_llvmc_emit_direct_array_i64_get(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key) {
        char recv_ref[64];
        char key_ref[64];
        if (!dst) return 0;
        if (!hako_llvmc_direct_array_i64_exact_enabled()) return 0;
        if (!hako_llvmc_is_selected_direct_array_i64_get_target(bid, ii, recv, key, dst)) return 0;

        hako_llvmc_format_i64_operand(recv_ref, sizeof(recv_ref), recv);
        hako_llvmc_format_i64_operand(key_ref, sizeof(key_ref), key);

        fprintf(f, "  %%direct_array_i64_get_base_%lld_%zu = and i64 %s, -4\n", bid, ii, recv_ref);
        fprintf(f, "  %%direct_array_i64_get_len_addr_%lld_%zu = add i64 %%direct_array_i64_get_base_%lld_%zu, 16\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_get_len_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_len_%lld_%zu = load i64, ptr %%direct_array_i64_get_len_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_nonneg_%lld_%zu = icmp sge i64 %s, 0\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_get_in_len_%lld_%zu = icmp ult i64 %s, %%direct_array_i64_get_len_%lld_%zu\n", bid, ii, key_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_can_load_%lld_%zu = and i1 %%direct_array_i64_get_nonneg_%lld_%zu, %%direct_array_i64_get_in_len_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  br i1 %%direct_array_i64_get_can_load_%lld_%zu, label %%direct_array_i64_get_load_%lld_%zu, label %%direct_array_i64_get_oob_%lld_%zu\n", bid, ii, bid, ii, bid, ii);

        fprintf(f, "direct_array_i64_get_load_%lld_%zu:\n", bid, ii);
        fprintf(f, "  %%direct_array_i64_get_byte_index_%lld_%zu = mul i64 %s, 8\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_get_data_base_%lld_%zu = add i64 %%direct_array_i64_get_base_%lld_%zu, 32\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_value_addr_%lld_%zu = add i64 %%direct_array_i64_get_data_base_%lld_%zu, %%direct_array_i64_get_byte_index_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_get_value_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_get_value_%lld_%zu = load i64, ptr %%direct_array_i64_get_value_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  br label %%direct_array_i64_get_done_%lld_%zu\n", bid, ii);

        fprintf(f, "direct_array_i64_get_oob_%lld_%zu:\n", bid, ii);
        fprintf(f, "  br label %%direct_array_i64_get_done_%lld_%zu\n", bid, ii);

        fprintf(f, "direct_array_i64_get_done_%lld_%zu:\n", bid, ii);
        fprintf(f, "  %%r%lld = phi i64 [ %%direct_array_i64_get_value_%lld_%zu, %%direct_array_i64_get_load_%lld_%zu ], [ 0, %%direct_array_i64_get_oob_%lld_%zu ]\n", dst, bid, ii, bid, ii, bid, ii);
        set_type(dst, T_I64);
        return 1;
      }

#include "hako_llvmc_ffi_generic_method_get_policy.inc"
#include "hako_llvmc_ffi_generic_method_get_lowering.inc"

      auto int hako_llvmc_emit_direct_array_i64_store_unchecked(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key,
          long long value) {
        char recv_ref[64];
        char key_ref[64];
        char value_ref[64];
        if (!hako_llvmc_direct_array_i64_exact_enabled()) return 0;
        if (!hako_llvmc_is_selected_direct_array_i64_unchecked_store_target(
                bid, ii, recv, key, value, dst)) return 0;

        hako_llvmc_format_i64_operand(recv_ref, sizeof(recv_ref), recv);
        hako_llvmc_format_i64_operand(key_ref, sizeof(key_ref), key);
        hako_llvmc_format_i64_operand(value_ref, sizeof(value_ref), value);

        fprintf(f, "  %%direct_array_i64_unchecked_base_%lld_%zu = and i64 %s, -4\n", bid, ii, recv_ref);
        fprintf(f, "  %%direct_array_i64_unchecked_byte_index_%lld_%zu = mul i64 %s, 8\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_unchecked_data_base_%lld_%zu = add i64 %%direct_array_i64_unchecked_base_%lld_%zu, 32\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_unchecked_value_addr_%lld_%zu = add i64 %%direct_array_i64_unchecked_data_base_%lld_%zu, %%direct_array_i64_unchecked_byte_index_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_unchecked_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_unchecked_value_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  store i64 %s, ptr %%direct_array_i64_unchecked_value_ptr_%lld_%zu\n", value_ref, bid, ii);
        if (dst) {
          fprintf(f, "  %%r%lld = add i64 1, 0\n", dst);
          set_type(dst, T_I64);
        }
        return 1;
      }

      auto int hako_llvmc_emit_direct_array_i64_store(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key,
          long long value) {
        char recv_ref[64];
        char key_ref[64];
        char value_ref[64];
        if (!hako_llvmc_direct_array_i64_exact_enabled()) return 0;
        if (!hako_llvmc_is_selected_direct_array_i64_store_target(bid, ii, recv, key, value, dst)) return 0;

        hako_llvmc_format_i64_operand(recv_ref, sizeof(recv_ref), recv);
        hako_llvmc_format_i64_operand(key_ref, sizeof(key_ref), key);
        hako_llvmc_format_i64_operand(value_ref, sizeof(value_ref), value);

        fprintf(f, "  %%direct_array_i64_base_%lld_%zu = and i64 %s, -4\n", bid, ii, recv_ref);
        fprintf(f, "  %%direct_array_i64_len_addr_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 16\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_len_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_len_%lld_%zu = load i64, ptr %%direct_array_i64_len_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_cap_addr_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 24\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_cap_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_cap_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_cap_%lld_%zu = load i64, ptr %%direct_array_i64_cap_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_nonneg_%lld_%zu = icmp sge i64 %s, 0\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_set_not_past_append_%lld_%zu = icmp ule i64 %s, %%direct_array_i64_len_%lld_%zu\n", bid, ii, key_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_within_cap_%lld_%zu = icmp ult i64 %s, %%direct_array_i64_cap_%lld_%zu\n", bid, ii, key_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_index_ok_%lld_%zu = and i1 %%direct_array_i64_set_nonneg_%lld_%zu, %%direct_array_i64_set_not_past_append_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_can_store_%lld_%zu = and i1 %%direct_array_i64_set_index_ok_%lld_%zu, %%direct_array_i64_set_within_cap_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  br i1 %%direct_array_i64_set_can_store_%lld_%zu, label %%direct_array_i64_set_store_%lld_%zu, label %%direct_array_i64_set_fail_%lld_%zu\n", bid, ii, bid, ii, bid, ii);

        fprintf(f, "direct_array_i64_set_store_%lld_%zu:\n", bid, ii);
        fprintf(f, "  %%direct_array_i64_set_byte_index_%lld_%zu = mul i64 %s, 8\n", bid, ii, key_ref);
        fprintf(f, "  %%direct_array_i64_set_data_base_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 32\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_value_addr_%lld_%zu = add i64 %%direct_array_i64_set_data_base_%lld_%zu, %%direct_array_i64_set_byte_index_%lld_%zu\n", bid, ii, bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_set_value_addr_%lld_%zu to ptr\n", bid, ii, bid, ii);
        fprintf(f, "  store i64 %s, ptr %%direct_array_i64_set_value_ptr_%lld_%zu\n", value_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_is_append_%lld_%zu = icmp eq i64 %s, %%direct_array_i64_len_%lld_%zu\n", bid, ii, key_ref, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_len_plus_one_%lld_%zu = add i64 %%direct_array_i64_len_%lld_%zu, 1\n", bid, ii, bid, ii);
        fprintf(f, "  %%direct_array_i64_set_next_len_%lld_%zu = select i1 %%direct_array_i64_set_is_append_%lld_%zu, i64 %%direct_array_i64_set_len_plus_one_%lld_%zu, i64 %%direct_array_i64_len_%lld_%zu\n", bid, ii, bid, ii, bid, ii, bid, ii);
        fprintf(f, "  store i64 %%direct_array_i64_set_next_len_%lld_%zu, ptr %%direct_array_i64_len_ptr_%lld_%zu\n", bid, ii, bid, ii);
        fprintf(f, "  br label %%direct_array_i64_set_done_%lld_%zu\n", bid, ii);

        fprintf(f, "direct_array_i64_set_fail_%lld_%zu:\n", bid, ii);
        fprintf(f, "  br label %%direct_array_i64_set_done_%lld_%zu\n", bid, ii);

        fprintf(f, "direct_array_i64_set_done_%lld_%zu:\n", bid, ii);
        if (dst) {
          fprintf(f, "  %%r%lld = phi i64 [ 1, %%direct_array_i64_set_store_%lld_%zu ], [ 0, %%direct_array_i64_set_fail_%lld_%zu ]\n", dst, bid, ii, bid, ii);
          set_type(dst, T_I64);
        }
        return 1;
      }

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
        char direct_store_ab[256];

        (void)ilen;
        (void)insts;
        if (!pair_root) return 0;
        if (!find_deferred_const_suffix_kernel_slot(pair_root, &lhs, &rhs)) return 0;
        rhs_const = get_str_const(rhs);
        if (!(rhs && rhs_const)) return 0;
        size_t rhs_len = strlen(rhs_const);
        int lhs_is_same_array_slot_source =
            find_array_string_get_source(lhs, &source_recv, &source_idx) &&
            resolve_copy_source_reg(source_recv) == resolve_copy_source_reg(recv) &&
            resolve_copy_source_reg(source_idx) == resolve_copy_source_reg(idx);

        if (lhs_is_same_array_slot_source) {
          direct_store_ab[0] = '\0';
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), recv, 1);
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), idx, 0);
          strncat(
              direct_store_ab,
              ", ptr getelementptr inbounds (",
              sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                rhs_len + 1,
                rhs);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          {
            char tail[64];
            snprintf(tail, sizeof(tail), ", i64 %zu", rhs_len);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.array.string_suffix_store_hisi\"(%s)\n",
                dst,
                direct_store_ab);
            set_type(dst, T_I64);
          } else {
            fprintf(
                f,
                "  call i64 @\"nyash.array.string_suffix_store_hisi\"(%s)\n",
                direct_store_ab);
          }
          return 1;
        }

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
        long long source_recv = 0;
        long long source_idx = 0;
        const char* middle_const = NULL;
        char slot_insert_ab[320];
        char slot_store_ab[128];
        char direct_store_ab[288];
        int source_is_array_slot = 0;
        int source_is_same_array_slot = 0;

        (void)ilen;
        (void)insts;
        if (!insert_root) return 0;
        if (!find_deferred_insert_mid_kernel_slot(insert_root, &source, &middle, &split)) return 0;
        middle_const = get_str_const(middle);
        if (!(source && middle && middle_const)) return 0;
        size_t middle_len = strlen(middle_const);
        source_is_array_slot = find_array_string_get_source(source, &source_recv, &source_idx);
        source_is_same_array_slot =
            source_is_array_slot &&
            resolve_copy_source_reg(source_recv) == resolve_copy_source_reg(recv) &&
            resolve_copy_source_reg(source_idx) == resolve_copy_source_reg(idx);

        if (source_is_same_array_slot) {
          direct_store_ab[0] = '\0';
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), recv, 1);
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), idx, 0);
          strncat(
              direct_store_ab,
              ", ptr getelementptr inbounds (",
              sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                middle_len + 1,
                middle);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          {
            char tail[64];
            snprintf(tail, sizeof(tail), ", i64 %zu", middle_len);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), split, 0);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.array.string_insert_mid_store_hisii\"(%s)\n",
                dst,
                direct_store_ab);
            set_type(dst, T_I64);
          } else {
            fprintf(
                f,
                "  call i64 @\"nyash.array.string_insert_mid_store_hisii\"(%s)\n",
                direct_store_ab);
          }
          return 1;
        }

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
        if (source_is_array_slot) {
          append_i64_arg_ref(slot_insert_ab, sizeof(slot_insert_ab), source_recv, 1);
          append_i64_arg_ref(slot_insert_ab, sizeof(slot_insert_ab), source_idx, 0);
        } else {
          append_i64_arg_ref(slot_insert_ab, sizeof(slot_insert_ab), source, 1);
        }
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
        if (source_is_array_slot) {
          fprintf(
              f,
              "  call i64 @\"nyash.array.kernel_slot_insert_hisi\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              insert_root,
              ii,
              slot_insert_ab);
        } else {
          fprintf(
              f,
              "  call i64 @\"nyash.string.kernel_slot_insert_hsi\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              insert_root,
              ii,
              slot_insert_ab);
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
        long long source_recv = 0;
        long long source_idx = 0;
        const char* middle_const = NULL;
        int source_is_array_slot = 0;
        int source_is_same_array_slot = 0;
        char slot_piecewise_ab[256];
        char slot_store_ab[128];
        char direct_store_ab[360];

        (void)ilen;
        if (!substring_root) return 0;
        if (!find_string_substring_call(substring_root, &piecewise_root, &start, &end)) return 0;
        piecewise_root = resolve_copy_chain_source_reg(insts, ii + 1, piecewise_root);
        start = resolve_copy_chain_source_reg(insts, ii + 1, start);
        end = resolve_copy_chain_source_reg(insts, ii + 1, end);
        if (!find_deferred_piecewise_subrange(piecewise_root, &source, &middle, &split)) return 0;
        middle_const = get_str_const(middle);
        if (!(source && middle && middle_const)) return 0;
        size_t middle_len = strlen(middle_const);
        source_is_array_slot = find_array_string_get_source(source, &source_recv, &source_idx);
        source_is_same_array_slot =
            source_is_array_slot &&
            resolve_copy_source_reg(source_recv) == resolve_copy_source_reg(recv) &&
            resolve_copy_source_reg(source_idx) == resolve_copy_source_reg(idx);

        if (source_is_same_array_slot) {
          direct_store_ab[0] = '\0';
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), recv, 1);
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), idx, 0);
          strncat(
              direct_store_ab,
              ", ptr getelementptr inbounds (",
              sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                middle_len + 1,
                middle);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          {
            char tail[64];
            snprintf(tail, sizeof(tail), ", i64 %zu", middle_len);
            strncat(
                direct_store_ab,
                tail,
                sizeof(direct_store_ab) - strlen(direct_store_ab) - 1);
          }
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), split, 0);
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), start, 0);
          append_i64_arg_ref(direct_store_ab, sizeof(direct_store_ab), end, 0);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.array.string_insert_mid_subrange_store_hisiiii\"(%s)\n",
                dst,
                direct_store_ab);
            set_type(dst, T_I64);
          } else {
            fprintf(
                f,
                "  call i64 @\"nyash.array.string_insert_mid_subrange_store_hisiiii\"(%s)\n",
                direct_store_ab);
          }
          return 1;
        }

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
        if (source_is_array_slot) {
          append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), source_recv, 1);
          append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), source_idx, 0);
        } else {
          append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), source, 1);
        }
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
        if (source_is_array_slot) {
          char slot_substring_ab[128];
          fprintf(
              f,
              "  call i64 @\"nyash.array.kernel_slot_insert_hisi\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              substring_root,
              ii,
              slot_piecewise_ab);
          slot_substring_ab[0] = '\0';
          append_i64_arg_ref(slot_substring_ab, sizeof(slot_substring_ab), start, 1);
          append_i64_arg_ref(slot_substring_ab, sizeof(slot_substring_ab), end, 0);
          fprintf(
              f,
              "  call i64 @\"nyash.string.kernel_slot_substring_hii_in_place\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              substring_root,
              ii,
              slot_substring_ab);
        } else {
          append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), start, 0);
          append_i64_arg_ref(slot_piecewise_ab, sizeof(slot_piecewise_ab), end, 0);
          fprintf(
              f,
              "  call i64 @\"nyash.string.kernel_slot_piecewise_subrange_hsiii\"(ptr %%arrstr.slot.%lld.%zu, %s)\n",
              substring_root,
              ii,
              slot_piecewise_ab);
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
