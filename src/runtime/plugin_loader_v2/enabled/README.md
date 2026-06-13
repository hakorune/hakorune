# Plugin Loader Runtime Binding Boundary

This directory is the enabled PluginLoader v2 runtime implementation.

## Layer Split

```text
PluginLoader route resolver / provider exports:
  plugin metadata and callable ids

BoxCallableRegistry:
  callable truth snapshot

RoutePlan:
  selected execution shape

runtime_invoke_boundary:
  runtime function pointer binding and compat-shim policy
```

## Runtime Invoke Boundary

`runtime_invoke_boundary.rs` is deliberately narrow.

Allowed:

```text
read PluginLoaderV2 function pointer table
bind invoke_box / invoke_shim function pointers
read compat fallback policy
```

Forbidden:

```text
own method_id / birth_id / fini_id truth
select routes
read TypeAbiCatalog / BoxDescriptorCatalog
generate RoutePlan
```

Consumers must enter this boundary only after a registry-derived route plan has
selected plugin execution.
