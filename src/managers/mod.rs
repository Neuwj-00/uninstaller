

pub mod pacman;
pub mod flatpak;
pub mod apt;
pub mod dnf;

pub struct Package {
    pub name: String,
    pub version: String,
    pub id: String,
}

pub trait PackageManager {
    fn name(&self) -> &'static str;
    fn list_packages(&self) -> anyhow::Result<Vec<Package>>;
    
    fn build_remove_command(&self, packages: &[String]) -> std::process::Command;
}