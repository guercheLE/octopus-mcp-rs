use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

pub mod router;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MasterWorkflowArgs {
    /// What the user is trying to accomplish, in their own words
    pub goal: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReleaseDeploymentWorkflowArgs {
    /// The Octopus Space id (e.g. "Spaces-1") the project lives in
    pub space_id: Option<String>,
    /// The project id or name to create/deploy a release for
    pub project_id: Option<String>,
    /// The target environment id or name to deploy to
    pub environment_id: Option<String>,
    /// The tenant id or name, for a tenanted deployment
    pub tenant_id: Option<String>,
    /// The channel id or name to create the release under
    pub channel_id: Option<String>,
}

/// Renders a short "Context already provided" / "Still unknown" header from
/// the prompt's own arguments, prepended to each prompt's static markdown
/// body — this is the only per-invocation content a prompt renders, so the
/// markdown itself never needs a template-substitution engine.
pub fn render_context_header(fields: &[(&str, Option<&str>)]) -> String {
    if fields.is_empty() {
        return String::new();
    }

    let (known, missing): (Vec<_>, Vec<_>) = fields.iter().partition(|(_, value)| value.is_some());

    let mut header = String::from("## Context already provided\n");
    if known.is_empty() {
        header.push_str("(none)\n");
    } else {
        for (name, value) in &known {
            header.push_str(&format!("- {name}: {}\n", value.unwrap()));
        }
    }

    header.push_str("\n## Still unknown — ask the user\n");
    if missing.is_empty() {
        header.push_str("(none)\n");
    } else {
        for (name, _) in &missing {
            header.push_str(&format!("- {name}\n"));
        }
    }

    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_renders_no_sections() {
        assert_eq!(render_context_header(&[]), "");
    }

    #[test]
    fn all_supplied_lists_no_missing_fields() {
        let header = render_context_header(&[
            ("space_id", Some("Spaces-1")),
            ("project_id", Some("Projects-1")),
        ]);
        assert!(header.contains("- space_id: Spaces-1"));
        assert!(header.contains("- project_id: Projects-1"));
        assert!(header.contains("## Still unknown — ask the user\n(none)"));
    }

    #[test]
    fn all_missing_lists_no_known_fields() {
        let header = render_context_header(&[("space_id", None), ("project_id", None)]);
        assert!(header.contains("## Context already provided\n(none)"));
        assert!(header.contains("- space_id"));
        assert!(header.contains("- project_id"));
    }

    #[test]
    fn mixed_supplied_and_missing_are_split_correctly() {
        let header =
            render_context_header(&[("space_id", Some("Spaces-1")), ("environment_id", None)]);
        assert!(header.contains("## Context already provided\n- space_id: Spaces-1"));
        assert!(header.contains("## Still unknown — ask the user\n- environment_id"));
    }
}
