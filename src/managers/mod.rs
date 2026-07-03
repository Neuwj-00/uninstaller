

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

pub fn detect_system_manager() -> Box<dyn PackageManager> {
    if std::path::Path::new("/usr/bin/pacman").exists() || std::path::Path::new("/bin/pacman").exists() {
        Box::new(pacman::Pacman)
    } else if std::path::Path::new("/usr/bin/dnf").exists() || std::path::Path::new("/bin/dnf").exists() {
        Box::new(dnf::Dnf)
    } else if std::path::Path::new("/usr/bin/apt-get").exists() || std::path::Path::new("/bin/apt-get").exists() {
        Box::new(apt::Apt)
    } else {
        Box::new(apt::Apt)
    }
}