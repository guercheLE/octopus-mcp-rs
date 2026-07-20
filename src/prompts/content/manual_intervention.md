# Resolve a pending manual intervention

A deployment or runbook run can pause on a manual-intervention step and wait for a human decision (approve/reject, or a custom form) before it continues. This sub-workflow finds one, claims it, and submits the response that lets the blocked task proceed.

This sub-workflow is designed to be run as an isolated sub-task where possible — if you were delegated here, or your environment otherwise supports its own sub-task context, everything you need is this prompt's text plus the parameters already listed above; report back only a short summary when done.

## Step 0 — gather required parameters

You need the space and, ideally, the interruption id. If the user only knows "my deployment is stuck," search for how to list interruptions in the space and match it to the deployment/task the user described rather than guessing an id.

## Step 1 — take responsibility for the interruption

Search for how to take responsibility for an interruption. This is gated by team membership — only a user in one of the interruption's responsible teams can claim it, so a failure here is a real permissions answer to relay to the user, not something to retry blindly. **Gate:** confirm responsibility actually landed on the calling user (search for how to get the interruption's responsible user) before moving on — a successful-looking call can still not have granted responsibility if someone else claimed it first.

## Step 2 — understand why the interruption exists

Every interruption originates from a specific deployment or runbook-run task. If the user wants context before deciding, fetch that task's detail — reuse `octopus-release-deployment`'s or `octopus-runbooks`' task-polling guidance for how to read a task's status/log rather than re-deriving it here; don't fetch a full log speculatively, only if the user needs the detail to decide.

## Step 3 — submit the response

Search for how to get the interruption's own detail first — its form fields are defined by the interruption itself (a plain approve/reject, or a richer custom form), not a fixed schema, so read that shape before submitting rather than assuming field names like "approved". Then search for how to submit the interruption's response with the values the user chose.

## Step 4 — confirm the blocked task resumed

**Gate:** poll the originating deployment/runbook-run task's status (again, per `octopus-release-deployment`'s Step 4 pattern) to confirm it actually continued and reached a final state, rather than assuming the submit call's success implies the task is unblocked.

## Composing with other workflows

Step 2 and Step 4 deliberately reuse `octopus-release-deployment`'s and `octopus-runbooks`' task-polling steps rather than duplicating them — fetch those prompts for the full detail.

Every operation above should be reached by searching for what it does, then reading the schema `get` returns before relying on any field name — this Octopus Server version supports interruption take-responsibility/submit actions, but a search-first approach is what keeps this prompt correct if it's ever run against a different, older API version this server might facade.
