# Environments, lifecycles & channels

Covers the structure a release progresses through: environments (deployment targets are grouped by environment), lifecycles (the ordered phases an environment sequence must follow, plus retention policies), and channels (per-project version-numbering/branching rules that pick which lifecycle applies).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to list/create/update an environment, how to list/create/update a lifecycle (including its phases and any automatic-deployment or retention settings), and how to list/create/update a channel for a project (including its version rules and which lifecycle it uses). Read the schema `get` returns for the operation you pick before relying on any field name.

A lifecycle change affects every project using it — if the user asks to modify an existing lifecycle's phases, confirm which projects/channels reference it before changing retention or phase order, rather than assuming it's safe to edit freely.
