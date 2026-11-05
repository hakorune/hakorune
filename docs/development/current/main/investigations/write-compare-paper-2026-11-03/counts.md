## Hakorune

### t1_match_borrow

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 66 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | 0 | 0 |

### t2_chain_borrow

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 91 | 2 | 0 | 0 | 0 | 5 | 0 | 3 | 0 | 0 |
| COMMON | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| required chain | 18 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 0 | 0 |
| nullable chain | 50 | 0 | 0 | 0 | 0 | 2 | 0 | 3 | 0 | 0 |
| temporary root | 13 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 10 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t3_map_update

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 51 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 3 |
| COMMON | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| update with mut capture (natural form) | 19 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 2 |
| update without capture (3a form, decided rules) | 20 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| no update: get is read-only | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4a_take_rewrite

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 93 | 9 | 4 | 4 | 0 | 0 | 1 | 4 | 1 | 0 |
| COMMON | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 |
| nullable children | 43 | 5 | 4 | 4 | 0 | 0 | 0 | 2 | 0 | 0 |
| array children | 26 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |
| caller | 13 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4b_required_child

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 203 | 38 | 12 | 2 | 0 | 1 | 0 | 13 | 10 | 0 |
| COMMON | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| COMMON box AST | 61 | 0 | 0 | 0 | 0 | 1 | 0 | 4 | 3 | 0 |
| V1 candidate node.body = fold(move node.body) | 42 | 10 | 4 | 2 | 0 | 0 | 0 | 2 | 2 | 0 |
| V2 decided rules read to fresh rebuild | 44 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 5 | 0 |
| V3 enum payload layout with match move | 47 | 24 | 8 | 0 | 0 | 0 | 0 | 5 | 0 | 0 |
| caller | 9 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4c_mut_parent

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 69 | 1 | 2 | 1 | 0 | 0 | 6 | 3 | 0 | 0 |
| COMMON | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
| nullable through mut | 15 | 1 | 2 | 1 | 0 | 0 | 1 | 2 | 0 | 0 |
| required through mut | 14 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 0 |
| whole node through mut | 5 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| enum child through mut | 18 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| caller | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t5_scope_lookup

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 99 | 2 | 2 | 3 | 0 | 4 | 0 | 7 | 0 | 3 |
| COMMON | 10 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| V1 from a, b | 16 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| V2 single support with one owner chain | 41 | 2 | 2 | 3 | 0 | 2 | 0 | 5 | 0 | 3 |
| V3 single support with two independent owners | 18 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t6_bundle

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 81 | 8 | 4 | 1 | 0 | 0 | 0 | 1 | 3 | 0 |
| COMMON | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| V1a box bundle required field (natural first draft) | 19 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 0 |
| V1b box bundle nullable field (decided rules) | 16 | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 1 | 0 |
| V2 record holding the box (undecided option B) | 14 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| V3 one-variant enum (no new permission) | 13 | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t7_shared_config

| section | loc | move | take_decl | take_expr | share | from_ | mut | nullable | copy | unspelled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 58 | 2 | 1 | 0 | 6 | 1 | 0 | 0 | 0 | 0 |

## Rust (rough token counts)

### t1_match_borrow

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 38 | 6 | 0 | 6 | 0 | 3 | 0 | 0 | 0 | 0 |

### t2_chain_borrow

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 60 | 14 | 0 | 6 | 1 | 3 | 0 | 0 | 0 | 0 |
| COMMON | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| required chain | 22 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| nullable chain | 21 | 5 | 0 | 4 | 1 | 3 | 0 | 0 | 0 | 0 |
| temporary root | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 7 | 1 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |

### t3_map_update

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 42 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4a_take_rewrite

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 71 | 2 | 3 | 7 | 3 | 4 | 4 | 0 | 0 | 0 |
| COMMON | 14 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 0 | 0 |
| nullable children | 36 | 2 | 0 | 3 | 3 | 2 | 4 | 0 | 0 | 0 |
| array children | 7 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 14 | 0 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4b_required_child

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 94 | 0 | 0 | 23 | 16 | 2 | 0 | 0 | 0 | 0 |
| COMMON | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| V1 struct nodes, field re-store (the candidate's Rust equivalent) | 52 | 0 | 0 | 9 | 9 | 1 | 0 | 0 | 0 | 0 |
| V3 enum payload layout | 24 | 0 | 0 | 8 | 7 | 1 | 0 | 0 | 0 | 0 |
| caller | 17 | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 0 |

### t4c_mut_parent

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 57 | 0 | 15 | 8 | 4 | 1 | 1 | 0 | 1 | 0 |
| COMMON | 13 | 0 | 0 | 2 | 0 | 1 | 0 | 0 | 0 | 0 |
| nullable through mut | 3 | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 0 |
| required through mut | 13 | 0 | 6 | 1 | 2 | 0 | 0 | 0 | 1 | 0 |
| whole node through mut | 3 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| enum child through mut | 10 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| caller | 15 | 0 | 6 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |

### t5_scope_lookup

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 84 | 24 | 7 | 2 | 1 | 7 | 1 | 0 | 0 | 9 |
| COMMON | 15 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| V1 two supports | 9 | 7 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 4 |
| V2 owner chain | 37 | 7 | 7 | 2 | 1 | 3 | 1 | 0 | 0 | 0 |
| V2b borrowed parent chain (first choice in Rust, not writable in Hakorune) | 9 | 4 | 0 | 0 | 0 | 2 | 0 | 0 | 0 | 5 |
| caller | 14 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### t6_bundle

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 30 | 2 | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |

### t7_shared_config

| section | loc | ref | ref_mut | box_ | deref | option | take_ | clone | replace | lifetime |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TOTAL | 51 | 14 | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 |

