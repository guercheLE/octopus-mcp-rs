# Users & teams

Covers users (create/update/list, server-level, *not* space-scoped), teams, scoped user roles (which permissions a team has, optionally restricted to specific spaces/projects/environments/tenants), and team membership.

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to list/create a user, how to list/create a team, how to list/create scoped user roles, and how to add/remove team members. Read the schema `get` returns before relying on any field name — unlike most other domains here, users/teams are server-level resources, so don't assume a space id is needed for these operations; check the operation's own parameters.

If the user's real ask is "why can't this person deploy to X," that's a scoped-user-role question (is their team's role actually scoped to include that space/project/environment/tenant), not a user-creation problem — confirm the role's scope before assuming account setup is the issue.
