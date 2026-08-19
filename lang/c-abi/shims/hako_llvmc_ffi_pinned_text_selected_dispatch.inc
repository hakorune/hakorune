          if (ptfc_selected) {
            int ptfc_emit = hako_llvmc_ptfc_try_emit_selected_op(
                f, ins, op, &ptfc_candidate, &ptfc_draft);
            if (ptfc_emit < 0) {
              note_pure_unsupported_shape(
                  bid, ii, op, "pinned_text_selected_lowering_failed", NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (ptfc_emit > 0) {
              if (!strcmp(op, "pinned_text_op")) {
                long long plan_index = read_int(ins, "plan");
                long long dst = read_int(ins, "dst");
                set_type(
                    dst,
                    ptfc_candidate.plans[plan_index].kind ==
                            HAKO_PTFC_ACCESS_SCALAR_EQ_TEXT
                        ? T_I1
                        : T_I64);
              }
              continue;
            }
          }
