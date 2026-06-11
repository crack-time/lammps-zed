use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result,
};

struct LammpsExtension {
    cached_binary_path: Option<String>,
}

impl LammpsExtension {
    fn asset_name(os: Os, arch: Architecture) -> Option<&'static str> {
        match (os, arch) {
            (Os::Linux, Architecture::X8664) => Some("lammps-lsp-x86_64-unknown-linux-gnu"),
            (Os::Windows, Architecture::X8664) => Some("lammps-lsp-x86_64-pc-windows-msvc.exe"),
            (Os::Mac, Architecture::X8664) => Some("lammps-lsp-x86_64-apple-darwin"),
            (Os::Mac, Architecture::Aarch64) => Some("lammps-lsp-aarch64-apple-darwin"),
            _ => None,
        }
    }

    fn install_binary(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "crack-time/lammps-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| {
            let msg = format!("Failed to fetch latest release: {e}");
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Failed(msg.clone()),
            );
            msg
        })?;

        let (os, arch) = zed::current_platform();
        let asset_name = Self::asset_name(os, arch).ok_or_else(|| {
            let msg = format!("Unsupported platform: {os:?} {arch:?}");
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Failed(msg.clone()),
            );
            msg
        })?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                let msg = format!(
                    "Asset {asset_name} not found in release {}",
                    release.version
                );
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(msg.clone()),
                );
                msg
            })?;

        let version_dir = format!("lammps-lsp-{}", release.version);
        let binary_path = format!("{version_dir}/{asset_name}");

        if !std::path::Path::new(&binary_path).exists() {
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            if std::path::Path::new(&version_dir).is_file() {
                std::fs::remove_file(&version_dir).ok();
            }
            std::fs::create_dir_all(&version_dir).map_err(|e| {
                let msg = format!("Failed to create directory: {e}");
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(msg.clone()),
                );
                msg
            })?;

            zed::download_file(
                &asset.download_url,
                &binary_path,
                DownloadedFileType::Uncompressed,
            )
            .map_err(|e| {
                let msg = format!("Failed to download binary: {e}");
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(msg.clone()),
                );
                msg
            })?;

            zed::make_file_executable(&binary_path).map_err(|e| {
                let msg = format!("Failed to make binary executable: {e}");
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(msg.clone()),
                );
                msg
            })?;

            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.starts_with("lammps-lsp-") && name != version_dir {
                            std::fs::remove_dir_all(entry.path()).ok();
                        }
                    }
                }
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        self.cached_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }
}

impl zed::Extension for LammpsExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        if let Some(path) = self.cached_binary_path.clone() {
            if std::fs::metadata(&path).is_ok() {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::None,
                );
                return Ok(zed::Command {
                    command: path,
                    args: vec![],
                    env: worktree.shell_env(),
                });
            }
        }

        let binary_path = self.install_binary(language_server_id)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(LammpsExtension);
