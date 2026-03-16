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
    #[serde(default = "default_true")]
    pub dev_default: bool,
    pub tasks: HashMap<String, TaskConfig>,
}

fn default_true() -> bool {
    true
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

    /// Returns projects that have a "dev" task configured
    pub fn projects_with_dev_task(&self) -> HashMap<String, &ProjectConfig> {
        self.projects
            .iter()
            .filter(|(_, project)| project.tasks.contains_key("dev"))
            .map(|(name, project)| (name.clone(), project))
            .collect()
    }

    /// Returns projects that should run by default with `meta dev`
    /// (have a dev task AND dev_default is not false)
    pub fn default_dev_projects(&self) -> HashMap<String, &ProjectConfig> {
        self.projects
            .iter()
            .filter(|(_, project)| {
                project.tasks.contains_key("dev") && project.dev_default
            })
            .map(|(name, project)| (name.clone(), project))
            .collect()
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

/// Parse a TOML string into a Config
#[cfg(test)]
pub fn parse(contents: &str) -> Result<Config> {
    let config: Config = toml::from_str(contents)?;
    Ok(config)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config_toml() -> &'static str {
        r#"
version = "1"

[workspace]
name = "Test"
root = "."

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

[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build" }

[projects.shared]
type = "rust"
path = "crates/shared"

[projects.shared.tasks]
build = { tool = "cargo", command = "build" }
test = { tool = "cargo", command = "test" }

[projects.trainee-app]
type = "rust"
path = "apps/trainee-app"

[projects.trainee-app.tasks]
dev = { tool = "bacon", command = "run-long" }

[projects.trainee-android]
type = "rust"
path = "apps/trainee-app"
dev_default = false

[projects.trainee-android.tasks]
dev = { tool = "cargo", command = "tauri android dev" }
"#
    }

    // Issue #7: doctor should not falsely report dev task for projects without one
    #[test]
    fn test_project_without_dev_task_not_in_dev_projects() {
        let config = parse(test_config_toml()).unwrap();
        let dev_projects = config.projects_with_dev_task();
        assert!(!dev_projects.contains_key("shared"));
    }

    #[test]
    fn test_project_with_dev_task_in_dev_projects() {
        let config = parse(test_config_toml()).unwrap();
        let dev_projects = config.projects_with_dev_task();
        assert!(dev_projects.contains_key("api"));
        assert!(dev_projects.contains_key("trainee-app"));
    }

    #[test]
    fn test_projects_sharing_path_have_independent_dev_task_detection() {
        let config = parse(test_config_toml()).unwrap();
        let dev_projects = config.projects_with_dev_task();
        // Both have dev tasks, both should appear despite shared path
        assert!(dev_projects.contains_key("trainee-app"));
        assert!(dev_projects.contains_key("trainee-android"));
    }

    #[test]
    fn test_project_sharing_path_without_dev_task_excluded() {
        let toml = r#"
version = "1"

[workspace]
name = "Test"
root = "."

[tools.bacon]
enabled = true
command = "bacon"

[projects.trainee-app]
type = "rust"
path = "apps/trainee-app"

[projects.trainee-app.tasks]
dev = { tool = "bacon", command = "run-long" }

[projects.trainee-android]
type = "rust"
path = "apps/trainee-app"

[projects.trainee-android.tasks]
build = { tool = "bacon", command = "build" }
"#;
        let config = parse(toml).unwrap();
        let dev_projects = config.projects_with_dev_task();
        assert!(dev_projects.contains_key("trainee-app"));
        assert!(!dev_projects.contains_key("trainee-android"));
    }

    // Issue #2: status should only show projects with dev task
    #[test]
    fn test_projects_with_dev_task_excludes_libraries() {
        let config = parse(test_config_toml()).unwrap();
        let dev_projects = config.projects_with_dev_task();
        assert!(!dev_projects.contains_key("shared"));
        assert!(dev_projects.contains_key("api"));
    }

    // Issue #1: dev_default = false excludes from default dev run
    #[test]
    fn test_dev_default_false_excludes_from_default_projects() {
        let config = parse(test_config_toml()).unwrap();
        let default_projects = config.default_dev_projects();
        assert!(!default_projects.contains_key("trainee-android"));
        assert!(default_projects.contains_key("api"));
        assert!(default_projects.contains_key("trainee-app"));
    }

    // Issue #7: exact reproduction of the reported config
    #[test]
    fn test_issue_7_exact_reproduction() {
        // This is the exact config from the GitHub issue
        let toml = r#"
version = "1"

[workspace]
name = "Test"
root = "."

[tools.trunk]
enabled = true
command = "trunk"

[tools.tauri]
enabled = true
command = "cargo-tauri"

[projects.trainee-app]
type = "rust"
path = "apps/trainee-app"

[projects.trainee-app.tasks]
dev = { tool = "trunk", command = "serve --port 3853" }

[projects.trainee-android]
type = "rust"
path = "apps/trainee-app"

[projects.trainee-android.tasks]
build = { tool = "tauri", command = "android build --target aarch64" }
"#;
        let config = parse(toml).unwrap();

        // trainee-app HAS dev task
        assert!(
            config.projects["trainee-app"].tasks.contains_key("dev"),
            "trainee-app should have dev task"
        );

        // trainee-android does NOT have dev task — only has build
        assert!(
            !config.projects["trainee-android"].tasks.contains_key("dev"),
            "trainee-android should NOT have dev task"
        );

        // Verify via the helper methods too
        let dev_projects = config.projects_with_dev_task();
        assert!(dev_projects.contains_key("trainee-app"));
        assert!(
            !dev_projects.contains_key("trainee-android"),
            "trainee-android should not appear in dev projects"
        );
    }

    #[test]
    fn test_dev_default_defaults_to_true() {
        let config = parse(test_config_toml()).unwrap();
        assert!(config.projects["api"].dev_default);
        assert!(!config.projects["trainee-android"].dev_default);
    }
}
