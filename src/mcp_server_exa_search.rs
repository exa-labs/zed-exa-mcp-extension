use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "exa-mcp-server";
const PACKAGE_VERSION: &str = "latest"; // Using latest version as recommended
const SERVER_PATH: &str = "node_modules/exa-mcp-server/build/index.js";

struct ExaSearchModelContextExtension;

#[derive(Debug, Deserialize)]
struct ExaSearchContextServerSettings {
    exa_api_key: String,
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
        let Some(settings) = settings.settings else {
            return Err("missing `exa_api_key` setting".into());
        };
        let settings: ExaSearchContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                "--tools=web_search,crawling".to_string(),
            ],
            env: vec![("EXA_API_KEY".into(), settings.exa_api_key)],
        })
    }
}

zed::register_extension!(ExaSearchModelContextExtension);