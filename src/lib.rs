use std::sync::Mutex;
use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId, Os,
    Result,
};

struct LammpsExtension {
    cached_binary_path: Mutex<Option<String>>,
}

impl LammpsExtension {
    fn asset_for_platform(os: Os, arch: Architecture) -> Option<&'static str> {
        match (os, arch) {
            (Os::Linux, Architecture::X8664) => Some("lammps-lsp-x86_64-unknown-linux-gnu"),
            (Os::Windows, Architecture::X8664) => Some("lammps-lsp-x86_64-pc-windows-gnu.exe"),
            (Os::Mac, Architecture::X8664) => Some("lammps-lsp-x86_64-apple-darwin"),
            (Os::Mac, Architecture::Aarch64) => Some("lammps-lsp-x86_64-apple-darwin"),
            _ => None,
        }
    }
}

impl zed::Extension for LammpsExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: Mutex::new(None),
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = self.cached_binary_path.lock().unwrap().clone() {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![],
            });
        }

        let (os, arch) = zed::current_platform();
        let asset_name = Self::asset_for_platform(os, arch)
            .ok_or_else(|| format!("unsupported platform: {os:?} {arch:?}"))?;

        if let Some(path) = worktree.which("lammps-lsp") {
            *self.cached_binary_path.lock().unwrap() = Some(path.clone());
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![],
            });
        }

        let release = zed::latest_github_release(
            "crack-time/lammps-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "asset {asset_name} not found in release {}",
                    release.version
                )
            })?;

        let binary_path = format!("lammps-lsp-{}", asset_name);
        zed::download_file(
            &asset.download_url,
            &binary_path,
            DownloadedFileType::Uncompressed,
        )?;

        #[cfg(not(target_os = "windows"))]
        zed::make_file_executable(&binary_path)?;

        let abs_path = binary_path;
        *self.cached_binary_path.lock().unwrap() = Some(abs_path.clone());

        Ok(zed::Command {
            command: abs_path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(LammpsExtension);
