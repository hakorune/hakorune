from instructions import field_access_helpers_common as _common
from instructions import field_access_helpers_typed as _typed

# Compatibility facade for the historical field_access_helpers module.  The
# call sites import underscore-prefixed helpers explicitly, so star imports
# would hide the exact names this facade must preserve.
for _module in (_common, _typed):
    for _name in dir(_module):
        if not _name.startswith("__"):
            globals()[_name] = getattr(_module, _name)
