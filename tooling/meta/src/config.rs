use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    // Auto-detect projects in the monorepo
    let detected_projects = detect_projects()?;

    // Generate configuration based on detected projects
    let config = generate_config(&detected_projects)?;

    fs::write("meta.toml", config)?;
    Ok(())
}

fn detect_projects() -> Result<Vec<DetectedProject>> {
    let mut projects = Vec::new();

    // Check apps directory
    if let Ok(entries) = fs::read_dir("apps") {
        for entry in entries.flatten() {
            if entry.file_type()?.is_dir() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                // Check for Cargo.toml (Rust project)
                if path.join("Cargo.toml").exists() {
                    projects.push(DetectedProject {
                        name: name.clone(),
                        path: format!("apps/{}", name),
                        project_type: ProjectType::Rust,
                        package_name: None,
                    });
                }
                // Check for package.json (Node project)
                else if path.join("package.json").exists() {
                    let package_json = fs::read_to_string(path.join("package.json"))?;
                    let package: serde_json::Value = serde_json::from_str(&package_json)?;
                    let package_name = package["name"].as_str().map(|s| s.to_string());

                    // Detect Next.js project
                    let is_next = package["dependencies"]
                        .as_object()
                        .and_then(|deps| deps.get("next"))
                        .is_some();

                    projects.push(DetectedProject {
                        name: name.clone(),
                        path: format!("apps/{}", name),
                        project_type: if is_next {
                            ProjectType::Next
                        } else {
                            ProjectType::Node
                        },
                        package_name,
                    });
                }
            }
        }
    }

    Ok(projects)
}

#[derive(Debug)]
struct DetectedProject {
    name: String,
    path: String,
    project_type: ProjectType,
    package_name: Option<String>,
}

#[derive(Debug)]
enum ProjectType {
    Rust,
    Next,
    Node,
}

fn generate_config(projects: &[DetectedProject]) -> Result<String> {
    let mut config = String::from(
        r#"version = "1"

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

"#,
    );

    // Add detected projects
    config.push_str("# Project definitions\n");

    for project in projects {
        config.push_str(&format!("[projects.{}]\n", project.name));

        match project.project_type {
            ProjectType::Rust => {
                config.push_str(&format!("type = \"rust\"\npath = \"{}\"\n\n", project.path));
                config.push_str(&format!("[projects.{}.tasks]\n", project.name));
                config.push_str("dev = { tool = \"bacon\", command = \"run-long\" }\n");
                config.push_str("build = { tool = \"cargo\", command = \"build --release\" }\n");
                config.push_str("test = { tool = \"cargo\", command = \"test\" }\n\n");
            }
            ProjectType::Next | ProjectType::Node => {
                let project_type = if matches!(project.project_type, ProjectType::Next) {
                    "next"
                } else {
                    "node"
                };

                config.push_str(&format!(
                    "type = \"{}\"\npath = \"{}\"\n\n",
                    project_type, project.path
                ));
                config.push_str(&format!("[projects.{}.tasks]\n", project.name));

                if let Some(ref pkg_name) = project.package_name {
                    config.push_str(&format!(
                        "dev = {{ tool = \"turborepo\", command = \"dev --filter={}\" }}\n",
                        pkg_name
                    ));
                    config.push_str(&format!(
                        "build = {{ tool = \"turborepo\", command = \"build --filter={}\" }}\n",
                        pkg_name
                    ));
                    config.push_str(&format!(
                        "test = {{ tool = \"turborepo\", command = \"test --filter={}\" }}\n\n",
                        pkg_name
                    ));
                } else {
                    config.push_str("dev = { tool = \"turborepo\", command = \"dev\" }\n");
                    config.push_str("build = { tool = \"turborepo\", command = \"build\" }\n");
                    config.push_str("test = { tool = \"turborepo\", command = \"test\" }\n\n");
                }
            }
        }
    }

    Ok(config)
}
