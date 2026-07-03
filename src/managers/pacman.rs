

use super::{Package, PackageManager};
use std::process::Command;
use anyhow::{Context, Result};

pub struct Pacman;

impl PackageManager for Pacman {
    fn name(&self) -> &'static str {
        "Pacman"
    }

    fn list_packages(&self) -> Result<Vec<Package>> {
        let output = Command::new("pacman")
            .arg("-Q")
            .output()
            .context("Failed to execute pacman command")?;

        let stdout = String::from_utf8(output.stdout)?;
        let mut packages = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    id: parts[0].to_string(),
                });
            }
        }
        Ok(packages)
    }

    fn build_remove_command(&self, packages: &[String]) -> Command {
        let mut cmd = Command::new("sudo");
        cmd.arg("pacman");
        cmd.arg("-Rns");
        cmd.arg("--noconfirm");
        
        for pkg in packages {
            cmd.arg(pkg);
        }
        cmd
    }
}