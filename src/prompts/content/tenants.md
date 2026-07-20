# Tenants

Covers tenant lifecycle (create/update/list), tenant-project connections (which environments a tenant is enabled for on a given project), tenant variables, and tag sets (used to group/filter tenants).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to list/create/update a tenant, how to connect a tenant to a project and environment, how to get/set a tenant's variables, and how to list/create tag sets and tags. Read the schema `get` returns before relying on any field name.

A tenant must be connected to both the target project *and* the target environment before a tenanted deployment can succeed — if the user is preparing a tenant for a deployment, confirm the connection exists (don't just create the tenant and assume it's ready). See `octopus-release-deployment` for how this fits into an actual deployment.
