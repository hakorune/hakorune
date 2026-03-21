# Ring1 Providers

This directory hosts the `ring1_*_provider` smoke pins split out of `integration/apps/`.
It keeps the provider probes together so the remaining `apps` bucket can continue shrinking by semantic domain.

Contained scripts:

- `ring1_array_provider_vm.sh`
- `ring1_console_provider_vm.sh`
- `ring1_map_provider_vm.sh`
- `ring1_path_provider_vm.sh`

Contract:

- keep these pins under the `integration` profile only
- keep them live and executable by `run.sh`
- keep the family small and evidence-oriented
