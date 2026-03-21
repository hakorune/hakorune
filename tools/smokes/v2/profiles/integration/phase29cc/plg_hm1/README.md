# phase29cc/plg_hm1

PLG-HM1 contract locks for plugin execution mode and route-skip behavior.

- `contract_tests`: consolidated cargo-test contract pin for min1..min4.
- `min1`: plugin exec mode accepts only supported values.
- `min2`: `module_first` skips dynamic route for Core4.
- `min3`: `module_first` skips dynamic route for FileBox and PathBox.
- `min4`: Math and Net stay on the dynamic compat lane.
