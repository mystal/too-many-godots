use std::process::{Command, Stdio};

use anyhow::{bail, Result};

use crate::{
    dirs::FygDirs,
    version::GodotVersion,
};

pub fn cmd(version: &str, mono: bool) -> Result<()> {
    let version = GodotVersion::new(version, mono);
    let fyg_dirs = FygDirs::get();

    // Try to launch the specified version.
    let bin_path = fyg_dirs.get_binary_path(&version);
    if !bin_path.is_file() {
        bail!("Version {} is not installed.", &version);
    }

    println!("Running: {}", bin_path.to_string_lossy());
    Command::new(&bin_path)
        .arg("--project-manager")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
