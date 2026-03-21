# phase29ck_boundary smoke family

Boundary compile canaries for the phase29ck lane.

Layout:
- `entry/`: boundary entry / forwarder / compat-keep contract
- `string/`: pure `StringBox` shapes
- `runtime_data/`: pure `RuntimeDataBox` shapes, grouped by array / map method family

Suite:
- `tools/smokes/v2/suites/integration/phase29ck-boundary.txt`
