use std::sync::Arc;

use octopus_mcp::auth::auth_manager::AuthManager;
use octopus_mcp::core::config_schema::{AuthMethod, Config};
use octopus_mcp::core::mcp_server::McpifyServer;
use rmcp::ServiceExt;
use rmcp::model::{ContentBlock, GetPromptRequestParams};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "apiKey"
    }))
    .unwrap();
    McpifyServer::new(
        "2023.4.82.90".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::ApiKey))),
    )
}

fn message_text(result: &rmcp::model::GetPromptResult) -> String {
    result
        .messages
        .iter()
        .map(|message| match &message.content {
            ContentBlock::Text(text) => text.text.clone(),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[tokio::test]
async fn prompts_list_advertises_the_full_workflow_surface() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let prompts = client.list_all_prompts().await.unwrap();
    let mut names: Vec<&str> = prompts.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();

    let mut expected = vec![
        "octopus_workflow",
        "octopus_workflow_release_deployment",
        "octopus_workflow_projects",
        "octopus_workflow_environments_lifecycles",
        "octopus_workflow_tenants",
        "octopus_workflow_variables",
        "octopus_workflow_runbooks",
        "octopus_workflow_infrastructure",
        "octopus_workflow_packages_feeds",
        "octopus_workflow_users_teams",
        "octopus_workflow_monitoring_diagnostics",
        "octopus_workflow_manual_intervention",
        "octopus_workflow_config_as_code",
        "octopus_workflow_server_administration",
    ];
    expected.sort_unstable();
    assert_eq!(names, expected);
    assert!(
        names
            .iter()
            .all(|name| name.starts_with("octopus_workflow")),
        "every advertised prompt should share the octopus_workflow* prefix, got {names:?}"
    );

    let release_deployment = prompts
        .iter()
        .find(|p| p.name == "octopus_workflow_release_deployment")
        .expect("release_deployment prompt should be advertised");
    let arguments = release_deployment
        .arguments
        .as_ref()
        .expect("release_deployment prompt should declare arguments");
    for expected in [
        "space_id",
        "project_id",
        "environment_id",
        "tenant_id",
        "channel_id",
    ] {
        let arg = arguments
            .iter()
            .find(|a| a.name == expected)
            .unwrap_or_else(|| panic!("missing argument {expected}"));
        assert_eq!(arg.required, Some(false), "{expected} should be optional");
    }

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn master_prompt_links_to_the_release_deployment_workflow() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(GetPromptRequestParams::new("octopus_workflow"))
        .await
        .unwrap();
    assert!(message_text(&result).contains("octopus_workflow_release_deployment"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn release_deployment_prompt_echoes_supplied_and_lists_missing_context() {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await.unwrap();

    let result = client
        .get_prompt(
            GetPromptRequestParams::new("octopus_workflow_release_deployment").with_arguments(
                serde_json::json!({ "space_id": "Spaces-1", "project_id": "Projects-1" })
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
        .await
        .unwrap();
    let text = message_text(&result);

    assert!(text.contains("space_id: Spaces-1"));
    assert!(text.contains("project_id: Projects-1"));
    assert!(text.contains("environment_id"));
    assert!(text.contains("tenant_id"));
    assert!(text.contains("channel_id"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[test]
fn server_info_advertises_the_prompts_capability() {
    use rmcp::ServerHandler;
    let info = server().get_info();
    assert!(info.capabilities.prompts.is_some());
    assert!(info.instructions.unwrap().contains("octopus_workflow"));
}
