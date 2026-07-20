# Projects

Covers project lifecycle (create/update/list), project groups, editing a project's deployment process (steps/actions), and project triggers (e.g. auto-deploy on new release).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Ask for the space first if it isn't already known — projects are always space-scoped. Search for how to list/create/update a project, how to list/create a project group, how to get or update a project's deployment process, and how to list/create project triggers, as the user's request requires. Read the schema `get` returns for the operation you pick before relying on any field name — in particular, confirm the project's tenanted-deployment-mode field (relevant to `octopus_workflow_release_deployment`) rather than assuming it.

If the user is setting up a brand-new project end to end (create project → add deployment process steps → deploy), point them to `octopus_workflow_release_deployment` for the deploy portion once the process is in place, rather than duplicating that flow here.
