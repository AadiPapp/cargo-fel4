use std::fs;
use std::path::Path;
use std::process::Command;

use super::Error;
use command_ext::CommandExt;
use config::{get_fel4_manifest_with_root_dir, CleanCmd, ManifestWithRootDir};

pub fn handle_clean_cmd(clean_cmd: &CleanCmd) -> Result<(), Error> {
    let ManifestWithRootDir {
        fel4_manifest,
        root_dir,
    } = get_fel4_manifest_with_root_dir(&clean_cmd.cargo_manifest_path)?;
    let artifact_path = Path::new(&root_dir).join(fel4_manifest.artifact_path);

    clean_cargo_build_cache(clean_cmd)?;

    if artifact_path.exists() {
        info!("Removing {}", artifact_path.display());
        fs::remove_dir_all(&artifact_path)?;
    }

    Ok(())
}

fn clean_cargo_build_cache(clean_cmd: &CleanCmd) -> Result<(), Error> {
    let mut cmd = Command::new("cargo");
    cmd.add_loudness_args(&clean_cmd.loudness)
        .arg("clean")
        .run_cmd()?;

    Ok(())
}
