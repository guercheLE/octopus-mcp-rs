# Packages & feeds

Covers package feeds (repository connections — NuGet, Docker, Maven, etc. — that Octopus pulls packages from), the packages themselves, and build information (metadata linking a package version back to the CI build/commit that produced it).

This sub-workflow is designed to be run as an isolated sub-task where possible — if delegated here, or if your environment supports its own sub-task context, everything you need is this prompt's text plus any parameters already supplied; report back only a short summary.

Search for how to list/create a feed, how to list packages (and package versions) within a feed, and how to push or query build information. Read the schema `get` returns before relying on any field name — feed credentials/URL shape varies by feed type.

If the user is troubleshooting "why isn't my expected package version showing up," that's usually a feed/package listing question, not a release-creation problem — confirm the version actually exists in the feed before assuming the release-creation call (`octopus-release-deployment`, Step 2) is at fault. Listing packages in a large feed can return a long result — if your environment supports an isolated sub-task, delegate the listing/filtering and bring back only the matching version(s).
