use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "exa-mcp-server";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/exa-mcp-server/.smithery/stdio/index.cjs";

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
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-exa-search", project)?;
        let settings: ExaSearchContextServerSettings = if let Some(settings_value) = settings.settings {
            serde_json::from_value(settings_value).map_err(|e| e.to_string())?
        } else {
            ExaSearchContextServerSettings {
                exa_api_key: None,
            }
        };

        let mut env_vars = Vec::new();
        if let Some(api_key) = settings.exa_api_key {
            env_vars.push(("EXA_API_KEY".into(), api_key));
        }

        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path],
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
