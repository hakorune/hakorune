# phase29ck_boundary smoke family

Boundary compile canaries for the phase29ck lane.

Layout:
- `entry/`: boundary owner anchors for daily / compat-keep / forwarder routing, plus core pure-first control-flow canaries such as the bool-phi branch pin
- `string/`: pure `StringBox` shapes plus string-owner canaries such as the concat3 extern boundary pin
- `runtime_data/`: pure `RuntimeDataBox` shapes, grouped by array / map method family

Suite:
- `tools/smokes/v2/suites/integration/phase29ck-boundary.txt`

Notes:
- keep this family focused on boundary owner coverage and narrow pure-first acceptance
- direct-emit app contracts stay outside this suite unless they become boundary-owner canaries
