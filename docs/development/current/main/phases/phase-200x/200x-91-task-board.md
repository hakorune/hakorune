# 200x-91 Task Board

Status: Landed

1. Add a dedicated DCE memory seam instead of widening `local_fields.rs`
2. Prune dead `Load` on definitely private `RefNew` carriers
3. Keep same-carrier `Store` and carrier escapes as blockers on the first cut
4. Move pointers from B1 to B2
