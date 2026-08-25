      auto int pure_prepass_validate_extern_call(
          long long bid,
          size_t ii,
          const char* op,
          const char* callee_name,
          yyjson_val* args,
          long long dst,
          yyjson_val* plan_entry) {
        struct LoweringPlanExternCallView extern_view;
        if (!read_lowering_plan_extern_call_view(plan_entry, &extern_view)) {
          pure_prepass_note_extern_reject(
              bid,
              ii,
              op,
              "extern_call_missing_plan",
              callee_name,
              NULL,
              "emit a Rust-issued extern_call_routes lowering plan");
          return 0;
        }
        return pure_prepass_extern_call_contract(
            bid,
            ii,
            op,
            callee_name,
            args,
            dst,
            &extern_view);
      }
