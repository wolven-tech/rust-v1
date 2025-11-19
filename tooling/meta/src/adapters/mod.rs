use anyhow::Result;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub struct ToolAdapter {
    pub name: String,
    pub command: String,
}

impl ToolAdapter {
    pub fn new(name: String, command: String) -> Self {
        Self { name, command }
    }

    pub async fn execute(&self, args: &[&str]) -> Result<()> {
        let mut cmd = Command::new(&self.command);
        cmd.args(args);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd.status().await?;

        if !status.success() {
            anyhow::bail!(
                "{} command failed: {} {}",
                self.name,
                self.command,
                args.join(" ")
            );
        }

        Ok(())
    }

    pub async fn execute_in(&self, args: &[&str], working_dir: &Path) -> Result<()> {
        let mut cmd = Command::new(&self.command);
        cmd.args(args);
        cmd.current_dir(working_dir);
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let status = cmd.status().await?;

        if !status.success() {
            anyhow::bail!(
                "{} command failed: {} {} (in {})",
                self.name,
                self.command,
                args.join(" "),
                working_dir.display()
            );
        }

        Ok(())
    }

    pub fn spawn(&self, args: &[&str]) -> Result<tokio::process::Child> {
        let mut cmd = Command::new(&self.command);
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;
        Ok(child)
    }

    pub fn spawn_in(&self, args: &[&str], working_dir: &Path) -> Result<tokio::process::Child> {
        let mut cmd = Command::new(&self.command);
        cmd.args(args);
        cmd.current_dir(working_dir);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;
        Ok(child)
    }
}
