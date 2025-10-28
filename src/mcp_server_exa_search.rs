use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "latest";
const DEFAULT_MCP_URL: &str = "https://mcp.exa.ai/mcp";

struct ExaSearchModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct ExaSearchContextServerSettings {
    #[serde(default)]
    exa_api_key: Option<String>,
}

impl zed::Extension for ExaSearchModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?;
        if version.is_none() {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-exa-search", project)?;
        let settings: ExaSearchContextServerSettings = if let Some(settings_value) = settings.settings {
            serde_json::from_value(settings_value).map_err(|e| e.to_string())?
        } else {
            ExaSearchContextServerSettings {
                exa_api_key: None,
            }
        };

        let mcp_url = DEFAULT_MCP_URL.to_string();

        let mut env_vars = Vec::new();
        if let Some(api_key) = settings.exa_api_key {
            env_vars.push(("EXA_API_KEY".into(), api_key));
        }

        let command = if cfg!(target_os = "windows") {
            "node_modules/.bin/mcp-remote.cmd".to_string()
        } else {
            let path = "node_modules/.bin/mcp-remote";
            zed::make_file_executable(path)?;
            path.to_string()
        };

        Ok(Command {
            command,
            args: vec![mcp_url],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ExaSearchContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ExaSearchModelContextExtension);
