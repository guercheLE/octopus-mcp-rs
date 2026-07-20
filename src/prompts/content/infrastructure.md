# Infrastructure

Covers deployment targets/machines, workers, worker pools, machine policies, and the accounts/certificates that back target credentials (e.g. cloud accounts, SSH/TLS certificates used to connect to a target).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to list/create/update a deployment target (machine), how to list workers and worker pools, how to list/create machine policies, and how to list/create accounts or certificates. Read the schema `get` returns for the operation you pick — target-creation payloads vary a lot by target type (e.g. SSH endpoint vs. cloud region target), so confirm the exact shape rather than assuming one.

If the user's real goal is "is this environment ready to deploy to," that's a read-only health check (list targets for the environment/tenant, check health status) — a candidate for step-level delegation if it returns a long list, per `octopus-monitoring-diagnostics`'s guidance. It also overlaps with `octopus-release-deployment`'s Step 2 readiness check — point there if the user is mid-deployment.
