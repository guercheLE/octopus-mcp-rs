# Create and run a runbook

Runbooks are operational procedures (e.g. database failover, scheduled maintenance) that run through the same deployment-process machinery as a release, but without a release/version attached. This sub-workflow covers create runbook → edit its process → publish a snapshot → run it.

This sub-workflow is designed to be run as an isolated sub-task where possible — if you were delegated here from `octopus`'s routing, or your environment otherwise supports running this as its own sub-task, everything you need is in this prompt's own text plus the parameters already listed above; report back only a short summary when done rather than the full step-by-step trace.

## Step 0 — gather required parameters

You need at minimum: the space and the project the runbook belongs to. If the user is running an *existing* runbook rather than creating one, you also need the runbook itself and the target environment (and tenant, if applicable — see Step 3's fork, which mirrors `octopus-release-deployment`'s). Don't proceed until these are known; search for how to list runbooks on a project if the user only has a name.

## Step 1 — create the runbook (skip if running an existing one)

Search for how to create a runbook for a project. A new runbook starts with an empty deployment process — it isn't runnable yet.

## Step 2 — edit the runbook's process, then publish a snapshot

Gated: don't attempt to run a runbook whose process has no steps, or whose latest edits haven't been published. Search for how to get/update a runbook's deployment process (adding steps/actions), then search for how to publish a runbook snapshot once the process is in the shape you want. A run always executes a *published snapshot*, not the live in-progress process — confirm a snapshot was actually published (via a follow-up get, not just "the publish call didn't error") before moving to Step 3.

## Step 3 — the tenanted-vs-untenanted fork

Same fork as `octopus-release-deployment`'s Step 1: a project's tenanted-deployment mode determines whether the run needs a tenant. Ask "should this run target a specific tenant, and if so which one?" rather than guessing from the parameters supplied. If tenanted, confirm the tenant is connected to this project and the target environment (see `octopus-tenants`) before proceeding.

## Step 4 — run the snapshot

Search for how to run a published runbook snapshot against an environment (and tenant, per the Step 3 fork). Confirm the target environment/tenant is ready first (deployment targets healthy) — this is the same readiness check `octopus-release-deployment`'s Step 2 performs, so reuse that logic rather than reinventing it; delegate it to an isolated sub-task if your environment supports one and the target list is long.

## Step 5 — poll the run to completion

Runbook runs produce a task reference just like deployments do — reuse `octopus-release-deployment`'s Step 4 task-polling guidance directly: poll status first, only fetch the full log (delegated, if possible) if the run fails or the user asks for detail.

## Step 6 — summarize

Report what ran, against which environment/tenant, and the final status.

## Composing with other workflows

Step 0/3 overlap with `octopus-projects` (tenanted-deployment-mode) and `octopus-tenants`; Step 4's readiness check and Step 5's polling reuse `octopus-release-deployment`'s Steps 2 and 4 rather than duplicating them — fetch that prompt for the full detail on either.

Every operation above should be reached by searching for what it does, then reading the schema `get` returns before relying on any field name or parameter shape.
