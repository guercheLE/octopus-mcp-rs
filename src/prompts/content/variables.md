# Variables

Covers project variables (scoped to environment/tenant/channel/deployment-target-role combinations) and library variable sets (shared variable collections a project can include).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to get/update a project's variables and how to list/create a library variable set, then read the schema `get` returns — the scope model (which combination of environment/tenant/channel/role a value applies to) is expressed as fields on each variable value, not a separate operation, so inspect an existing variable's shape before adding a new one.

When a value should differ per environment or per tenant, add a scoped value rather than a single unscoped one — ask the user which scope(s) they actually need rather than guessing. If a tenanted deployment is failing because a required variable isn't set for that tenant, that's this workflow's concern even if the user framed it as a deployment problem — cross-reference `octopus-release-deployment`'s Step 2 readiness check.
