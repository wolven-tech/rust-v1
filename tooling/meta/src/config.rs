use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workspace: WorkspaceConfig,
    pub tools: HashMap<String, ToolConfig>,
    pub projects: HashMap<String, ProjectConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub root: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    pub enabled: bool,
    pub command: String,
    #[serde(default)]
    pub for_languages: Vec<String>,
    #[serde(default)]
    pub for_tasks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(rename = "type")]
    pub project_type: String,
    pub path: String,
    pub tasks: HashMap<String, TaskConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub tool: String,
    pub command: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Path::new("meta.toml");
        if !path.exists() {
            anyhow::bail!("meta.toml not found. Run 'meta init' first.");
        }

        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

pub fn init() -> Result<()> {
    let default_config = r#"version = "1"

[workspace]
name = "Meta Monorepo"
root = "."

# Tool declarations
[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript", "javascript"]
for_tasks = ["dev", "build", "lint", "typecheck"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]
for_tasks = ["dev"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]
for_tasks = ["build", "test"]

# Project definitions
[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
test = { tool = "cargo", command = "test" }

[projects.web]
type = "next"
path = "apps/web"

[projects.web.tasks]
dev = { tool = "turborepo", command = "dev --filter=@meta/web" }
build = { tool = "turborepo", command = "build --filter=@meta/web" }
test = { tool = "turborepo", command = "test --filter=@meta/web" }

[projects.app]
type = "next"
path = "apps/app"

[projects.app.tasks]
dev = { tool = "turborepo", command = "dev --filter=@meta/app" }
build = { tool = "turborepo", command = "build --filter=@meta/app" }
test = { tool = "turborepo", command = "test --filter=@meta/app" }
"#;

    fs::write("meta.toml", default_config)?;
    Ok(())
}
