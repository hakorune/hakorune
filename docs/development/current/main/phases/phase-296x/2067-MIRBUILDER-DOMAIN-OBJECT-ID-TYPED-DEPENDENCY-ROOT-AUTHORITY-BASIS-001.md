# 2067 - MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001

## Token

```text
MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001
```

## Purpose

Define typed dependency-root authority for unresolved non-ID DomainObject/Id
subaxes. This card does not select PlanRecipe, MIR, AST, Context/Span, or
Other.

## Rule

```text
selector_rule:
  DomainObjectIdTypedDependencyRootAuthorityV1

extends:
  DomainObjectIdSubaxisMechanicalSelectorV1.dependency_root_authority

edge_direction:
  dependent_subaxis_requires_prerequisite_subaxis

select root only if:
  exactly_one dependency root exists
  ambiguous_edge_count = 0
  forbidden_edge_source_count = 0
  cycle_count = 0
```

Isolated candidates are unranked: they are not selected, but they also do not
block a unique dependency root.

## Accepted Edge Evidence

```text
ReturnTypeFieldReference
ParameterTypeReference
ConstructedDomainObjectReference
VerifierInputContractReference
PolicyDecisionPayloadReference
FixtureDeclaredSemanticResourceDependency
```

## Forbidden Edge Evidence

```text
RowCount
OwnerName
SourcePath
RouteMembershipAlone
CoveragePercentage
ImplementationConvenience
LexicalOrder
HardcodedSubaxisPriority
```

## Result

```text
decision:
  SelectDomainObjectIdSubaxisPriorityRerun003

reason_token:
  DefineDomainObjectIdTypedDependencyRootAuthorityBeforeSubaxisSelection

selected_domain_subaxis:
  null

selected_next_card:
  MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-003
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_domain_object_id_typed_dependency_root_authority_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_domain_object_id_typed_dependency_root_authority_basis_guard.sh
```

## Recovery For Rerun

```text
NoMachineDerivedDomainObjectIdTypedDependencyRootAuthority
MultipleDomainObjectIdTypedDependencyRootCandidates
DomainObjectIdSubaxisDependencyCycleUnresolved
```

## Non-Claims

```text
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
manual_subaxis_selection = 0
hardcoded_subaxis_priority = 0
row_count_as_proof = 0
domain_object_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
convenience_as_proof = 0
```
