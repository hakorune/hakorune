# phase29ck_boundary smoke family

Boundary compile canaries for the phase29ck lane.

Layout:
- `entry/`: active boundary anchors for compat-keep / forwarder routing plus unflipped pure-first control-flow canaries
- `string/`: active pure `StringBox` shapes that still belong to the phase29ck boundary lane
- `runtime_data/`: pure `RuntimeDataBox` shapes, grouped by array / map method family

Suite:
- `tools/smokes/v2/suites/integration/phase29ck-boundary.txt`
- `tools/smokes/v2/suites/integration/phase29ck-boundary-legacy.txt`

Notes:
- keep `phase29ck-boundary.txt` focused on active boundary owner coverage and unflipped pure-first acceptance
- `ret const`, `bool phi/branch`, and `concat3 extern` are now legacy boundary locks; their daily owner proof moved to `phase29x/derust`
- `phase29ck-boundary-legacy.txt` is a temporary suite for those retired locks until compare/archive decisions are complete
- direct-emit app contracts stay outside this suite unless they become boundary-owner canaries
