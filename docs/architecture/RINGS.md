# Runtime Rings Architecture (ring0 / ring1 / ring2)

Purpose: clarify responsibilities and naming for core runtime layers, and make provider selection and diagnostics consistent without large code moves.

Detailed SSOT:
- `docs/development/current/main/design/ring2-provider-link-abi-lifecycle-ssot.md`
- `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`

Overview
- ring0 (Kernel): Box-unaware and language-unaware host API substrate (memory, I/O, time, log, filesystem, threads). No Box semantics or provider policy.
- ring1 (Core Providers): Minimal, trusted, always-available providers (static). Example: FileBox Core-RO (open/read/close). Small, reproducible, and safe to enable under fail-fast.
- ring2 (Extensions): Featureful, application/ecosystem, swappable providers. A ring2 provider may be dynamically loaded or statically embedded; static linkage does not promote it into ring1.

Mapping (current repo)
- ring0: `src/runtime/ring0/` owns the current `Ring0Context`; `src/ring0/` remains a facade/guard anchor.
- ring1: src/providers/ring1/ (facade + guard). Concrete code still lives where it is; ring1 hosts documentation and future home for static providers.
- ring2: plugins/ (dynamic shared libraries, as before).

Selection Policy (current Auto compatibility mode)
- Global: `HAKO_PROVIDER_POLICY=strict-plugin-first|safe-core-first|static-preferred`
  - strict-plugin-first (default): order the dynamic/plugin candidate before ring1 where the current compatibility owner permits alternatives.
  - safe-core-first/static-preferred: order the ring1 candidate before the plugin candidate where the current compatibility owner permits alternatives.
- Per box (example: FileBox)
  - `NYASH_FILEBOX_MODE=auto|ring1|plugin-only`
  - `NYASH_FILEBOX_ALLOW_FALLBACK=0|1` (narrow legacy/dev compatibility override)

The current FileBox Auto fallback is migration inventory, not the target
TypedFast call contract. Provider/transport selection must eventually finish
before an effect-bearing init, birth, or method call. Once execution starts,
failure must not retry the same operation through another provider or
transport.

Diagnostics (stderr, quiet when JSON_ONLY=1)
- Selection: `[provider/select:<Box> ring=<0|1|plugin> src=<static|dynamic>]`
- Fail-Fast block: `[failfast/provider/<box>:<reason>]`

Design Invariants
- ring0 must not depend on ring2. ring1 contains only minimal stable capabilities.
- Ring classification records responsibility/trust. Provider residency (`dynamic|embedded`), ABI transport (`BID-TLV|TypedFast`), dispatch binding (`generic|table|method-pointer|direct-symbol`), and optimization outcome are independent axes.
- Application configuration may select an embedded ring2 provider but must not grant ring1 status.
- Post-effect fallback/retry is forbidden. Any temporary preselection compatibility override must remain named, narrow, and fail-fast by default.
- `JSON_ONLY` changes diagnostic output only; it never authorizes a provider or transport fallback.

Migration Plan (small steps)
1) Add facades and guards (this change).
2) Keep existing code paths; introduce provider policy (done for FileBox).
3) Gradually move minimal static providers under `src/providers/ring1/` (no behavior change).
4) Add canaries to assert selection tags under different policies.

Notes on Naming
- Historical names like "builtin" referred to in-tree providers. To avoid confusion, use ring terms: ring0 (kernel), ring1 (core providers), ring2 (plugins).
