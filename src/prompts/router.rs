use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{PromptMessage, Role};
use rmcp::{prompt, prompt_router};

use crate::core::mcp_server::McpifyServer;
use crate::prompts::{
    ConfigAsCodeWorkflowArgs, ManualInterventionWorkflowArgs, MasterWorkflowArgs,
    ReleaseDeploymentWorkflowArgs, render_context_header,
};

#[prompt_router(vis = "pub(crate)")]
impl McpifyServer {
    #[prompt(
        name = "octopus_workflow",
        description = "Start here. Presents the available Octopus Deploy management workflows, \
                        routes to the right guided sub-workflow based on the user's goal, \
                        and — where the environment supports it — delegates that whole \
                        sub-workflow to an isolated sub-task to spare this conversation's \
                        context window."
    )]
    async fn octopus_workflow_prompt(
        &self,
        Parameters(args): Parameters<MasterWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[("goal", args.goal.as_deref())]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/master.md")),
        )]
    }

    #[prompt(
        name = "octopus_workflow_release_deployment",
        description = "Guided create-and-deploy-a-release flow, including the \
                        tenanted-vs-untenanted fork and deployment task polling."
    )]
    async fn octopus_workflow_release_deployment_prompt(
        &self,
        Parameters(args): Parameters<ReleaseDeploymentWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("space_id", args.space_id.as_deref()),
            ("project_id", args.project_id.as_deref()),
            ("environment_id", args.environment_id.as_deref()),
            ("tenant_id", args.tenant_id.as_deref()),
            ("channel_id", args.channel_id.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/release_deployment.md")
            ),
        )]
    }

    #[prompt(
        name = "octopus_workflow_projects",
        description = "Project lifecycle (create/update/list), project groups, deployment \
                        process (steps/actions) editing, and project triggers."
    )]
    async fn octopus_workflow_projects_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/projects.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_environments_lifecycles",
        description = "Environments, lifecycles (phases, retention policies), and channels \
                        (version rules) — the structure a release progresses through."
    )]
    async fn octopus_workflow_environments_lifecycles_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/environments_lifecycles.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_tenants",
        description = "Tenant lifecycle, tenant-project connections, tenant variables, and tag sets."
    )]
    async fn octopus_workflow_tenants_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/tenants.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_variables",
        description = "Project variables (scoped to environment/tenant/channel/role) and \
                        library variable sets."
    )]
    async fn octopus_workflow_variables_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/variables.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_runbooks",
        description = "Runbook lifecycle: create runbook, edit its process, publish a \
                        snapshot, and run it against an environment/tenant."
    )]
    async fn octopus_workflow_runbooks_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/runbooks.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_infrastructure",
        description = "Deployment targets/machines, workers, worker pools, machine policies, \
                        and the accounts/certificates that back target credentials."
    )]
    async fn octopus_workflow_infrastructure_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/infrastructure.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_packages_feeds",
        description = "Package feeds (repository connections), packages, and build information."
    )]
    async fn octopus_workflow_packages_feeds_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/packages_feeds.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_users_teams",
        description = "Users, teams, scoped user roles, and team membership/permissions."
    )]
    async fn octopus_workflow_users_teams_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/users_teams.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_manual_intervention",
        description = "Find a pending interruption blocking a deployment or runbook run, \
                        take responsibility for it, and submit the response that unblocks it."
    )]
    async fn octopus_workflow_manual_intervention_prompt(
        &self,
        Parameters(args): Parameters<ManualInterventionWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("space_id", args.space_id.as_deref()),
            ("interruption_id", args.interruption_id.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!(
                "{header}\n\n{}",
                include_str!("content/manual_intervention.md")
            ),
        )]
    }

    #[prompt(
        name = "octopus_workflow_config_as_code",
        description = "Connect a project to a git repository (Config As Code): set up a git \
                        credential, connect the project, and know when to address deployment \
                        settings/process \"in git\" vs. \"in database\"."
    )]
    async fn octopus_workflow_config_as_code_prompt(
        &self,
        Parameters(args): Parameters<ConfigAsCodeWorkflowArgs>,
    ) -> Vec<PromptMessage> {
        let header = render_context_header(&[
            ("space_id", args.space_id.as_deref()),
            ("project_id", args.project_id.as_deref()),
        ]);
        vec![PromptMessage::new_text(
            Role::User,
            format!("{header}\n\n{}", include_str!("content/config_as_code.md")),
        )]
    }

    #[prompt(
        name = "octopus_workflow_server_administration",
        description = "Server-wide admin operations: SMTP, authentication providers, Octopus \
                        Server nodes (HA), proxies, scheduler, dynamic-extensions feature \
                        flags, webhook subscriptions, and general server configuration."
    )]
    async fn octopus_workflow_server_administration_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/server_administration.md"),
        )]
    }

    #[prompt(
        name = "octopus_workflow_monitoring_diagnostics",
        description = "Thin pointer to the right read-only signal: dashboard, tasks, events, \
                        deployment/runbook progression, interruptions, and server status."
    )]
    async fn octopus_workflow_monitoring_diagnostics_prompt(&self) -> Vec<PromptMessage> {
        vec![PromptMessage::new_text(
            Role::User,
            include_str!("content/monitoring_diagnostics.md"),
        )]
    }
}
