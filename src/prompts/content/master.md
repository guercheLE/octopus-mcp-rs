# Octopus Deploy management workflows

Route the user's goal to one of the guided sub-workflows below, then fetch that prompt (`prompts/get`) for its full step-by-step instructions. If your environment provides a way to run a sub-task/agent in an isolated context, delegate the *entire* matched sub-workflow to it — hand it the sub-workflow's prompt name and whatever parameters are already known, let it fetch the prompt itself and carry out every step (including its own `search`/`get`/`call` traffic) in its own context, and have it report back only a short summary. Only run a sub-workflow's steps directly in this conversation if no such delegation mechanism is available.

- `octopus_workflow_release_deployment` — create a release and deploy it to an environment (or tenant), including polling the deployment to completion. Use for "deploy X to Y" / "cut a release" requests.
- `octopus_workflow_runbooks` — create/edit a runbook, publish a snapshot, and run it against an environment (or tenant). Use for operational procedures that aren't tied to a release (failover, maintenance, one-off scripts).
- `octopus_workflow_projects` — project lifecycle, project groups, deployment process editing, project triggers. Use for "set up a new project" or "change how this project deploys."
- `octopus_workflow_environments_lifecycles` — environments, lifecycles (phases/retention), channels (version rules). Use for "add an environment" or "change the deployment sequence/version rules."
- `octopus_workflow_tenants` — tenant lifecycle, tenant-project connections, tenant variables, tag sets. Use for multi-customer/tenanted setups.
- `octopus_workflow_variables` — project variables and library variable sets. Use for "set/change a config value" requests.
- `octopus_workflow_infrastructure` — deployment targets/machines, workers, worker pools, machine policies, accounts, certificates. Use for "register a new target" or target-health questions.
- `octopus_workflow_packages_feeds` — package feeds, packages, build information. Use for "where does this package come from" or feed-connection questions.
- `octopus_workflow_users_teams` — users, teams, scoped user roles, permissions. Use for access/permission questions.
- `octopus_workflow_monitoring_diagnostics` — dashboard, tasks, events, progression, interruptions, server status. Use for read-only "what's the current state of X" questions.

If the user's goal spans more than one of these (e.g. "set up a new tenanted project and deploy it"), route to each relevant sub-workflow in the order its dependencies require, and prefer delegating each as its own sub-task rather than interleaving their steps in this conversation.

Ask the user which Octopus Space they're working in if it isn't already known — nearly every operation above is scoped to a space (a few, like users/teams and server configuration, are server-level instead; the sub-workflow you route to will call that out where it matters).
