

use super::{Package, PackageManager};
use std::process::Command;
use anyhow::{Context, Result};

pub struct Apt;

impl PackageManager for Apt {
    fn name(&self) -> &'static str {
        "APT"
    }

    fn list_packages(&self) -> Result<Vec<Package>> {
        let output = Command::new("dpkg-query")
            .args(["-W", "-f=${Package}\t${Version}\n"])
            .output()
            .context("Failed to execute dpkg-query command")?;

        let stdout = String::from_utf8(output.stdout)?;
        let mut packages = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].trim().to_string(),
                    version: parts[1].trim().to_string(),
                    id: parts[0].trim().to_string(),
                });
            }
        }

        Ok(packages)
    }

    fn build_remove_command(&self, packages: &[String]) -> Command {
        let mut cmd = Command::new("sudo");
        cmd.arg("apt-get");
        cmd.arg("remove");
        cmd.arg("-y");
        
        for pkg in packages {
            cmd.arg(pkg);
        }
        cmd
    }
}