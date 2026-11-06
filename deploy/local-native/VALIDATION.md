# Local validation: missing saved pin UUIDs

Validated on Linux AMD64 with Rust 1.98.0, Node 24.22.0 and the v5.3.0-alpha
runtime. Tests used standalone containers, temporary directories and disposable
PostgreSQL databases on an isolated internal network. No production data was
seeded. No user designs are included in this branch.

Passed:

- Full native workspace: 1,413 tests passed; two repository-defined tests ignored.
- Native formatting, Clippy with warnings denied, generated Rust bindings,
  contract generation/checks, dependency licences/sources, and Rust hygiene.
- Generator determinism and pins across the committed fixtures. All 519
  portable member hashes outside the semantic index and manifest are unchanged.
- Contract examples and all 259 versioned golden documents validate. Parity
  runner tests (18), its self-test, and all 33 fixture input checks pass.
- Frontend: 2,218 tests across 297 files, lint, React Doctor baseline gate,
  production build and provider-panel build. Lint retains an unrelated existing
  unused-variable warning; no frontend behavior change was made for this fix.
- Backend bundle/render suite against disposable PostgreSQL: 218 tests and
  105 subtests passed. Two tests skipped: a developer-only checkout and a
  Windows-only file-sharing behavior. The real KiCad render integration ran.
- Analytic WASM rebuilt with the pinned toolchain and wasm-bindgen 0.2.127.
  Semantic viewer tests (269), harness tests (4), and served-asset regeneration
  pass. The frozen a2 schema remains available for stored bundles.
- Standard source Compose and proxy-overlay configuration checks, plus agent
  documentation link checks.

The original failure was reproduced in isolation. The patched generator
accepts the affected project's bundle without changing its semantic content
apart from the schema version. A copy of an existing a2 bundle still verifies.
After local deployment, the affected project's real parse, board and models
stages completed, and the public reverse-proxy readiness endpoint passed.

The native runtime image is based on implementation commit `6bd80b35`; later
commits update render test fixtures, regenerated viewer assets and this report.
The complete backend suite and GitHub's cross-platform CI were not run locally.
These scoped results do not replace the upstream pull-request quality gate.
