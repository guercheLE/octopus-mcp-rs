# Create and deploy a release

This sub-workflow is designed to be run as an isolated sub-task where possible — if you were delegated here from `octopus_workflow`'s routing, or your environment otherwise supports running this as its own sub-task, everything you need is in this prompt's own text plus the parameters already listed above; report back only a short summary when done rather than the full step-by-step trace.

## Step 0 — gather required parameters

Check the "Context already provided" header above first; only ask the user for what's still listed as missing. You need at minimum: the space, the project, and the target environment. Don't proceed to Step 1 until all three are known — search for how to list spaces or projects if the user only gave you names, not ids.

## Step 1 — the tenanted-vs-untenanted fork

A project's tenanted-deployment mode genuinely forks the rest of this flow:

- **(A) Untenanted** — deploy straight to the environment, no tenant involved.
- **(B) Tenanted or tenanted-or-untenanted** — a tenant must be resolved, its connection to this project *and* the target environment confirmed to exist, and its required variables confirmed to be set, before a deployment can succeed.

Ask the user "should this release deploy to a specific tenant, and if so which one?" rather than guessing — don't infer this from whether a tenant happens to be supplied in the context header, since an omitted tenant on a tenanted project is a missing-parameter case, not evidence the project is untenanted. If unsure which mode the project uses, search for how to get a project's details and read the tenanted-deployment-mode field back before asking.

## Step 2 — create the release and confirm target readiness (parallelizable)

Two independent sub-steps that don't depend on each other:

1. **Create the release**: search for how to create a release for a project. Resolve the channel if one wasn't given (search for the project's channels and their version rules), and confirm the package versions you're about to use are actually available in the feed (search for how to list package versions) before calling.
2. **Confirm the target is ready**: search for how to list deployment targets/machines for the environment (and tenant, if fork B applies) and confirm at least one is healthy; if fork B applies, also confirm the tenant's required variables are set (search for how to get tenant variables).

If your environment provides a way to run a sub-task in its own context (e.g. an agent/task tool), delegate "create the release" and "confirm the target environment is ready" as two separate sub-tasks and have each return only a short confirmation — don't pull the full create-call request/response bodies into this conversation. If no such sub-task mechanism is available, just do both calls directly.

**Gate:** don't proceed until the release is confirmed to exist (a follow-up search-and-get by its id/version, not just "the call didn't error") and the target is confirmed ready.

## Step 3 — deploy the release

Gated on Step 2's release and target both being confirmed. Search for how to create a deployment, branching on the Step 1 fork: include the tenant in the deployment call for fork B, omit it for fork A.

## Step 4 — poll the deployment task to completion

Gated on the deployment call returning a task reference. Search for how to get a task's status and poll it rather than assuming success. Fetching a task's *full log* is verbose relative to what this workflow needs back — poll status first, and only fetch the full log if the task fails or the user asks for detail; if your environment supports an isolated sub-task, delegate the full-log fetch and bring back only the relevant excerpt.

## Step 5 — summarize and offer follow-ups

Report what was deployed, where, and the final task status. Offer natural next steps: progress the release to the next environment in the project's lifecycle, or roll back if the deployment failed. If the user wants the files a deployment step captured (e.g. a generated report or log artifact), search for how to list artifacts rather than assuming they're in the task log.

If the deployment task is blocked waiting on a manual-intervention step instead of failing outright, that's `octopus_workflow_manual_intervention`'s job, not this workflow's — fetch that prompt rather than trying to resolve it here.

## Composing with other workflows

Steps 1–2 overlap with `octopus_workflow_projects` (tenanted-deployment-mode setting, channel version rules), `octopus_workflow_tenants` (tenant-project connections), `octopus_workflow_variables`, and `octopus_workflow_infrastructure` (deployment target health); Step 4 overlaps with `octopus_workflow_monitoring_diagnostics` (task/progression detail). Fetch those prompts by name for more detail rather than duplicating their content here.

Every operation above should be reached by searching for what it does, then reading the schema `get` returns before relying on any field name or parameter shape — never assume a hardcoded operationId or that every operation takes a space id (most do; a few server-level resources don't).
