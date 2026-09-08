/* PHI registration remains owned by the generic prepass lexical scope. */
            struct BlockPhi* pb = ensure_phi_block(bid); if (!pb) { goto GEN_END; }
            struct PhiRec pr; pr.dst = read_int(ins, "dst"); pr.in_n=0; pr.explicit_type=0;
            yyjson_val* dst_type = yyjson_obj_get(ins, "dst_type");
            const char* dst_type_s = read_str(ins, "dst_type");
            int explicit_phi_ty = T_NONE;
            int explicit_phi_metadata = 0;
            yyjson_val* vals = yyjson_obj_get(ins, "values"); if (!vals) vals = yyjson_obj_get(ins, "incoming");
            if (dst_type_s) {
              if (!strcmp(dst_type_s, "i1") || !strcmp(dst_type_s, "bool") ||
                  !strcmp(dst_type_s, "Bool")) {
                explicit_phi_ty = T_I1;
              } else if (!strcmp(dst_type_s, "i64")) {
                explicit_phi_ty = T_I64;
              }
            }
            if (vals && yyjson_is_arr(vals)) {
              size_t vn = yyjson_arr_size(vals);
              for (size_t vi=0; vi<vn && pr.in_n<16; vi++) {
                yyjson_val* ent = yyjson_arr_get(vals, vi);
                long long pred = 0;
                long long vin = 0;
                if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 2) {
                  yyjson_val* ent0 = yyjson_arr_get(ent, 0);
                  yyjson_val* ent1 = yyjson_arr_get(ent, 1);
                  vin = ent0 ? (long long)yyjson_get_sint(ent0) : 0;
                  pred = ent1 ? (long long)yyjson_get_sint(ent1) : 0;
                } else {
                  pred = read_int(ent, "pred"); if (!pred) pred = read_int(ent, "block");
                  vin = read_int(ent, "value");
                }
                pr.in[pr.in_n].pred = pred; pr.in[pr.in_n].val_reg = vin; pr.in_n++;
              }
            } else {
              long long pred = read_int(ins, "pred"); long long vin = read_int(ins, "value");
              if (vin) { pr.in[pr.in_n].pred=pred; pr.in[pr.in_n].val_reg=vin; pr.in_n++; }
            }
            if (pb->rec_n < 16) {
              int phi_bool_ok = 1;
              int phi_bool_seen = 0;
              int scan_first = ORG_NONE;
              int scan_same = 1;
              int origin_first = ORG_NONE;
              int origin_same = 1;
              for (int i = 0; i < pr.in_n; i++) {
                int s = pr.in[i].val_reg ? get_scan_origin(pr.in[i].val_reg) : ORG_NONE;
                int o = pr.in[i].val_reg ? get_origin(pr.in[i].val_reg) : ORG_NONE;
                if (is_bool_like_phi_input(pr.in[i].val_reg)) {
                  phi_bool_seen = 1;
                } else {
                  phi_bool_ok = 0;
                }
                if (s != ORG_NONE) {
                  if (scan_first == ORG_NONE) scan_first = s;
                  else if (scan_first != s) scan_same = 0;
                }
                if (o != ORG_NONE) {
                  if (origin_first == ORG_NONE) origin_first = o;
                  else if (origin_first != o) origin_same = 0;
                }
              }
              pb->recs[pb->rec_n++] = pr;
              if (dst_type && yyjson_is_obj(dst_type)) {
                explicit_phi_metadata =
                    generic_pure_publish_value_type_metadata(pr.dst, dst_type);
              } else {
                explicit_phi_metadata = generic_pure_publish_reg_metadata(pr.dst);
              }
              if (all_phi_inputs_are_local_variants(&pr)) {
                set_variant_binding(pr.dst, 0, pr.dst, pr.dst, 1);
                set_type(pr.dst, T_I64);
              }
              if (explicit_phi_ty != T_NONE) {
                set_type(pr.dst, explicit_phi_ty);
              } else if (!explicit_phi_metadata) {
                set_type(pr.dst, (phi_bool_ok && phi_bool_seen) ? T_I1 : T_I64);
              }
              if (scan_same && scan_first != ORG_NONE) {
                set_scan_origin(pr.dst, scan_first);
              }
              if (origin_same && origin_first != ORG_NONE) {
                set_origin(pr.dst, origin_first);
              }
            }
