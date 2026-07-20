# Connect a project to version control (Config As Code)

A project's deployment process, deployment settings, and some variables can live in a connected git repository instead of Octopus's own database. This is a distinct persistence model, not just a setting — once connected, the operations you'd otherwise call against the project directly are addressed via a `gitRef` (branch, tag, or commit) instead.

This sub-workflow is designed to be run as an isolated sub-task where possible — if you were delegated here, or your environment otherwise supports its own sub-task context, everything you need is this prompt's text plus the parameters already listed above; report back only a short summary when done.

**Version caveat:** Config As Code is a newer addition to the Octopus Server API than most operations the other workflows here use. Search first rather than assuming these operations exist — if the search for git-credential or git-backed deployment-settings operations comes back empty, tell the user this Octopus Server version may not support Config As Code rather than guessing at a substitute.

## Step 0 — gather required parameters

You need the space and the project. Confirm with the user whether the project is already version-controlled or this is a first-time connection — the rest of this workflow forks on that.

## Step 1 — set up a git credential (skip if reusing an existing one)

Search for how to create a git credential in the space. Note this stores repository *authentication*, not the repository URL — the URL is supplied separately when connecting a project.

## Step 2 — connect the project to the repository

Search for how to connect/convert a project to version control, supplying the repository URL and the credential from Step 1. **Gate:** don't assume success from the call alone — confirm the project's persistence/connection settings actually reflect a git-backed project afterward (a follow-up get, not just "the call didn't error").

## Step 3 — know the fork going forward

Once connected, deployment-process and deployment-settings operations split into two families: the plain (database) form and a `gitRef`-parameterized form. Ask the user which branch they mean before calling the git form — don't assume `main`/`master` or any other default. If the project *isn't* connected to version control, use the plain database-backed operations instead (see `octopus_workflow_projects`, which also covers the database form of deployment settings — release-creation strategy, connectivity policy, default channel).

## Step 4 — a worked example

Search for how to get or modify deployment settings for a project "in git" — the operation takes the project id and a `gitRef` rather than just the project id. Read the schema `get` returns before relying on any field name; the git and database forms are similar but not identical.

## Step 5 — stale-data troubleshooting

If the user reports that git-sourced data looks out of date after a push to the repository, search for how to clear the server's local git cache rather than assuming the connection is broken.

## Composing with other workflows

Everything about the project that *doesn't* depend on this fork (project groups, triggers, tenanted-deployment mode) is unchanged — see `octopus_workflow_projects`. The deployment itself, once a release is cut, proceeds exactly as `octopus_workflow_release_deployment` describes regardless of whether the project is version-controlled.
