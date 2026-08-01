# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Version headers are derived from this repo's `chore(release): bump version to X.Y.Z`
commits and their matching `vX.Y.Z` git tags.

## [0.7.8] - 2026-07-26

### Fixed

- Build the multi-arch (amd64+arm64) GHCR image on native runners instead of QEMU emulation: a matrix job now builds `linux/amd64` on `ubuntu-24.04` and `linux/arm64` natively on `ubuntu-24.04-arm`, pushing each by digest, then a merge job combines both into one multi-arch manifest with `docker buildx imagetools create`.

## [0.7.7] - 2026-07-26

### Fixed

- Publish a real multi-arch (amd64+arm64) GHCR image — `docker/build-push-action` never set `platforms:`, so the release workflow only ever built and pushed `linux/amd64`, which failed to pull with "no matching manifest" on arm64 hosts (Apple Silicon, arm64 CI runners).

## [0.7.6] - 2026-07-26

### Fixed

- Fully inline `$ref` in catalog input/output schemas where possible, instead of leaving a reference to a locally embedded `$defs` block. Confirmed remaining `$ref` occurrences are either genuine reference cycles (e.g. `ActivityLogTreeNode` nesting itself) or dangling references to components that don't exist anywhere in the official Octopus Deploy OpenAPI spec (an upstream spec bug, left as-is).

## [0.7.5] - 2026-07-21

_Internal only: stabilized a flaky MCP protocol test. No user-facing changes._

## [0.7.4] - 2026-07-21

### Fixed

- Regenerated the store and schemas with fully-expanded `$ref` (via `mcpify add-version --set-default --force`, mcpify 0.11.5): every operation's stored `input_schema`/`output_schema` now embeds its own `$defs` map, since a `get`-tool response could previously contain a `$ref` pointing at nothing in the returned snippet.

## [0.7.3] - 2026-07-21

### Fixed

- Capped `release.yml`'s build job (45 min) and `release`/`ci.yml`'s test job (20 min) with `timeout-minutes`, so a hang fails fast instead of falling back to GitHub Actions' 6-hour job ceiling.

## [0.7.2] - 2026-07-21

### Fixed

- Recover from mutex poisoning on the shared embedded-store connection instead of propagating it: a panic while holding the process-wide `cached_store_connection` lock previously poisoned it permanently, so every later `search`/`get`/`call` (and CLI invocation) sharing that connection would panic too. The MCP protocol test now also has a 30s timeout so a future hang fails fast.

## [0.7.1] - 2026-07-20

### Fixed

- Renamed every MCP prompt from `octopus_workflow[_domain]` to `octopus[-domain]` (the master prompt is now exactly `octopus`; every sub-workflow is `octopus-<domain>`, e.g. `octopus-release-deployment`), aligning the naming convention with idiomatic MCP identifiers.

### Documentation

- Updated the MCP prompts workflow plan's inventory table, code snippets, and verification section to match the renamed prompt names.

## [0.7.0] - 2026-07-20

### Added

- Three new guided MCP prompts closing gaps found by re-scanning the full operation catalog: `octopus-manual-intervention` (take responsibility for and submit a response to a pending interruption blocking a deployment/runbook run), `octopus-config-as-code` (connect a project to a git repository for its deployment process/settings), and `octopus-server-administration` (SMTP, authentication providers, HA server nodes, proxies, scheduler, dynamic-extensions, subscriptions). Also cross-references artifacts and community action templates from the existing release-deployment and projects prompts, and documents the Workflows feature in the README.

### Documentation

- Extended the MCP prompts workflow plan with the round-2 workflow-coverage review and design rationale.

## [0.6.0] - 2026-07-20

### Added

- MCP **prompts** capability alongside the existing `search`/`get`/`call` tools: a master `octopus` menu prompt plus 10 guided sub-workflow prompts (release & deployment, projects, environments & lifecycles, tenants, variables, runbooks, infrastructure, packages & feeds, users & teams, monitoring/diagnostics) that walk a calling LLM through multi-step Octopus Deploy tasks with explicit gates, forks, and sub-task delegation guidance.

### Documentation

- Added `docs/mcp-prompts-workflow-plan.md`, the design record for the guided-workflow prompts feature.

## [0.5.11] - 2026-07-19

### Changed

- Reverted the Windows Defender build-path exclusion added in 0.5.10 — the measured CI time gain over the prebuilt cargo-dist installer alone was modest (19m56s vs 21m41s) and not worth the added risk for a step that couldn't be validated locally.

## [0.5.10] - 2026-07-19

### Changed

- Excluded the cargo registry and checkout directory from Windows Defender real-time scanning in CI, targeting the disproportionately slow Windows `Verify`/`Build archives` steps (later reverted in 0.5.11).

## [0.5.9] - 2026-07-19

### Changed

- Install `cargo-dist` from its prebuilt-binary installer script in CI instead of compiling it from source — cut the "Install cargo-dist" CI step from up to 7m08s (Windows) to about 8 seconds.

## [0.5.8] - 2026-07-19

### Fixed

- Retry the store-extraction rename a few times with a short backoff on transient Windows failures: separate OS processes (not just threads within one process) can race to extract/rename the embedded SQLite store for the same API version, which Windows can reject outright unlike POSIX. Fixes both a CI flake and the same latent race for real concurrent Windows deployments.

## [0.5.7] - 2026-07-19

### Fixed

- Stopped the `setup` CLI smoke test from hanging indefinitely on GitHub Actions Windows runners — `inquire`'s Windows backend reads raw console input rather than treating closed/non-console stdin as immediate EOF the way Unix does. Skipped via a runtime check scoped to GitHub Actions specifically, so a real Windows machine still gets full test coverage.

## [0.5.6] - 2026-07-19

### Documentation

- Added a sponsorship callout to the README and a `FUNDING.yml`.

## [0.5.5] - 2026-07-19

### Fixed

- Stopped the stdio smoke test from hanging on GitHub Actions Windows runners (previously surfaced as a run needing manual cancellation after ~5 hours): Windows' background blocking-thread stdin reader doesn't observe an immediately-EOF-closed child stdin the way Unix does. Skipped via a runtime check scoped to GitHub Actions, not a blanket Windows skip.

## [0.5.4] - 2026-07-19

### Fixed

- Relaxed a Windows-fragile stdio-transport test assertion that checked for exact `rmcp` error text — Windows doesn't surface the same wording as Linux/macOS when a server never receives an `initialize` request. The platform-independent property (handshake never completes, process exits non-zero) is still asserted everywhere.

## [0.5.3] - 2026-07-19

_Internal only: closed a remaining production test-coverage gap (85.49% → 88.68%). No user-facing changes._

## [0.5.2] - 2026-07-19

### Fixed

- Eliminated a Windows-only race in `resolve_store_path`: the embedded SQLite store is now extracted and cached once per process (per API version, behind a mutex) instead of being re-extracted and renamed on every tool call, which could make Windows reject the rename outright and made unrelated MCP-protocol tests intermittently fail in CI.

## [0.5.1] - 2026-07-19

### Fixed

- `dist host --steps=create` (the actual release-tag trigger) refused to run against this repo's intentionally simplified, hand-written `release.yml`, flagging it "out of date" against cargo-dist's own auto-generated multi-job shape. Added `allow-dirty` to tell dist the divergence is intentional; confirmed against this repo's own v0.5.0 release run.

## [0.5.0] - 2026-07-19

### Changed

- Re-synced against mcpify's current Rust code-generation templates: the embedded store is now zstd-compressed (11.5MB raw → 4.3MB compressed); hardened auth/validation/schema handling that was previously hand-patched directly into this repo (including the `api_key` auth strategy using the API's real declared header name, `X-Octopus-ApiKey`, instead of a hardcoded `X-Api-Key`) is now generated correctly by default; release workflow simplified; coverage and profiling scaffolding added.

### Fixed

- Reformatted an over-long empty-body guard clause to satisfy `cargo fmt --check`.

### Documentation

- Added a "Connect an MCP client" section to the README with real stdio/HTTP `mcpServers` configuration examples.

## [0.4.4] - 2026-07-17

### Fixed

- Completed the OAuth2 code exchange: the credentials cascade now falls back to the initial `authenticate()` exchange when there's no `refresh_token` to try, instead of erroring out on any credential blob missing both an access and refresh token.
- Send an explicit empty body with `Content-Length: 0` on body-less `PUT`/`POST`/`DELETE` requests — operations whose arguments are entirely query/path parameters previously sent no `Content-Length`, which strict APIs reject with `411 Length Required` before ever reaching auth or business logic.

## [0.4.3] - 2026-07-16

### Fixed

- Extract `mcp_store.db` atomically (write to a uniquely-named sibling file, then rename into place) instead of via a direct, non-atomic write on every tool call. Concurrent MCP tool calls running as separate tokio tasks could previously read the store file mid-truncate and hit spurious "no such table: endpoints" errors.

## [0.4.2] - 2026-07-16

### Fixed

- `call` no longer rejects an otherwise-successful call when the live API response doesn't match its documented output schema — it now logs a warning and still returns the response, since upstream OpenAPI specs are frequently wrong about response shape. Input validation is unchanged and still rejects invalid caller-supplied arguments. `McpifyError::Validation`'s error text now includes the actual jsonschema violation details instead of a generic message.

## [0.4.1] - 2026-07-16

### Changed

- Applied `cargo fmt` across the codebase (formatting only, no functional changes).

## [0.4.0] - 2026-07-15

### Fixed

- Send the correct `X-Octopus-ApiKey` header for `api_key` auth over stdio transport instead of a hardcoded `X-Api-Key`, which Octopus Deploy was rejecting/ignoring (the root cause of a QA-reported 401).
- Fall back to the encrypted credential file when the OS keychain cleanly reports "not found," not only on a hard error.
- Resolve the home directory via `HOME` then `USERPROFILE` on Windows.
- Read `OCTOPUS_MCP_TOKEN`/`OCTOPUS_MCP_API_KEY` (or username/password) from the environment before falling back to the stored credential.
- The setup wizard now prompts for global vs. local config persistence and writes YAML the config loader actually reads back.
- `call`'s `arguments` field now defaults to `{}` instead of `null`, matching its advertised schema.
- `populate-embeddings` now populates and verifies every API version's store by default (row-count parity between endpoints and semantic index), failing loudly instead of silently under-populating; `search` now warns when a store is incomplete.
- Documented that `OCTOPUS_MCP_URL` must include the `/api` suffix.

## [0.3.0] - 2026-07-14

### Changed

- Adopted current mcpify-generated Rust parity/templates.

### Documentation

- Clarified container transport usage.

## [0.2.5] - 2026-07-13

### Documentation

- Documented real `call` tool arguments.

## [0.2.4] - 2026-07-10

### Fixed

- Fixed release builds broken by ONNX Runtime (`ort`) cross-platform issues: dropped the `x86_64-apple-darwin` target (no prebuilt ONNX Runtime binary exists for it since ORT 1.24.1 dropped x64 macOS support — Intel Mac users can still build from source); pinned `x86_64-unknown-linux-gnu` to `ubuntu-24.04` (glibc 2.39) since the default `ubuntu-22.04` runner's glibc was too old for pyke's prebuilt ONNX Runtime; disabled static MSVC CRT linking on Windows, which is incompatible with fastembed/ort's prebuilt ONNX Runtime.

## [0.2.3] - 2026-07-10

### Added

- Distribute prebuilt binaries for macOS (x86_64/arm64), Linux (x86_64), and Windows via `cargo-dist`, published to GitHub Releases with shell/PowerShell installer scripts, triggered by pushing a version tag. Split crates.io publishing and the GHCR image push into their own workflows.

## [0.2.2] - 2026-07-08

### Documentation

- Replaced a fabricated, off-domain Usage example (a "create an issue" search unrelated to Octopus Deploy's API) and an invalid `--some-arg` `call` syntax with real, source-grounded examples. Added Configuration, Docker, Observability & Resilience, and License sections to the README — previously omitted entirely — including noting that `OCTOPUS_MCP_API_KEY` was documented in `.env.example` but not actually read by any code path at the time (credentials only came from the OS keychain populated by `setup`).

## [0.2.1] - 2026-07-08

### Fixed

- Normalize credentials before use in the authentication cascade.

## [0.2.0] - 2026-07-06

> Bumped in commit `2af849f`, but never tagged — superseded two days later by `v0.2.1`. Kept here for completeness since real, released changes shipped under this version.

### Changed

- Enhanced the release workflow, logging, and embedded-store management, and updated dependencies.

## [0.1.1] - 2026-07-06

### Added

- Initial generated Octopus Deploy Server API MCP server, exposing exactly 3 tools — `search`, `get`, `call` — backed by an embedded semantic database.

### Fixed

- Installed `rustls` as the default crypto provider (prevents a startup panic) and updated dependencies.
