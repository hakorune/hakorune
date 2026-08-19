        if (ptfc_selected &&
            hako_llvmc_ptfc_provenance_base_block_v1(bid, 0) != 0) {
          note_pure_unsupported_shape(
              bid, 0, "block", "pinned_text_provenance_failed", NULL);
          yyjson_doc_free(d);
          goto GEN_ABORT;
        }
        emit_block_label(bid);
