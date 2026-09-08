/* Generic direct Array physical predicates and emission; lexical compile context. */
      auto void hako_llvmc_format_i64_operand(char* out, size_t out_cap, long long reg) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        if (has_const(base, &cv)) snprintf(out, out_cap, "%lld", cv);
        else snprintf(out, out_cap, "%%r%lld", base);
      }

      auto int hako_llvmc_direct_array_i64_exact_enabled(void) {
        return (allocation_config.runtime_flags &
                HAKO_LLVMC_RUNTIME_MODE_REQUIRES_DIRECT_ARRAY_I64_EXACT) != 0;
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
