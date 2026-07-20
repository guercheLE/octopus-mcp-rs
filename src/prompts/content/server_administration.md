# Server administration

Covers server-wide operations that aren't scoped to any space: SMTP (email notification settings), authentication providers (login configuration), Octopus Server nodes (health/management of a high-availability cluster), proxies (network egress used to reach deployment targets/feeds), the built-in scheduler (status, enable/disable, trigger-now, and logs for maintenance jobs), dynamic-extensions feature flags, webhook/event subscriptions, and general server configuration sections (including certificate configurations and the default retention policy new projects inherit).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

These operations are independent of each other and of the space-scoped domains covered elsewhere — search for the one that matches the user's request rather than following a fixed sequence:

- **Email notifications not arriving?** Search for how to get/update the SMTP configuration, and how to check whether SMTP is configured, before assuming a deeper bug.
- **Login/SSO questions?** Search for how to list authentication providers.
- **HA cluster health?** Search for how to list Octopus Server nodes and ping one; a node's task-count detail is a separate operation from its summary listing.
- **A deployment target unreachable through a corporate network?** Search for how to list/create proxies — this is the network-egress configuration, distinct from the target's own credentials (see `octopus_workflow_infrastructure`).
- **A scheduled/maintenance job not running?** Search for the scheduler's status, and its enable/trigger-now/logs operations — don't assume a job is broken before checking whether it (or the scheduler as a whole) is simply disabled.
- **Feature flags / experimental settings?** Search for the dynamic-extensions feature metadata and values operations.
- **Notify an external system on Octopus events?** Search for how to list/create a subscription (webhook).
- **General server config, certificates, or default retention for new projects?** Search for the configuration section list, then the specific section by id.

Read the schema `get` returns for whichever operation you pick before relying on any field name — several of these (subscriptions, configuration sections) have per-type payload shapes. If a search for one of these areas comes back empty, tell the user this Octopus Server version may not expose that particular administrative feature rather than assuming it's a bug in this server.
