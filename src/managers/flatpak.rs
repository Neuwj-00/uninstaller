

use super::{Package, PackageManager};
use std::process::Command;
use anyhow::{Context, Result};

pub struct Flatpak;

impl PackageManager for Flatpak {
    fn name(&self) -> &'static str {
        "Flatpak"
    }

    fn list_packages(&self) -> Result<Vec<Package>> {
        let output = Command::new("flatpak")
            .args(["list", "--app", "--columns=name,version,application"])
            .output()
            .context("Failed to execute flatpak command")?;

        let stdout = String::from_utf8(output.stdout)?;
        let mut packages = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                packages.push(Package {
                    name: parts[0].trim().to_string(),
                    version: parts[1].trim().to_string(),
                    id: parts[2].trim().to_string(),
                });
            }
        }
        Ok(packages)
    }

    fn build_remove_command(&self, packages: &[String]) -> Command {
        let mut cmd = Command::new("flatpak");
        cmd.arg("uninstall");
        cmd.arg("-y");
        for pkg in packages {
            cmd.arg(pkg);
        }
        cmd
    }
}