use zed_extension_api::{self as zed, LanguageServerId, Result};

struct LammpsExtension;

impl zed::Extension for LammpsExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("lammps-lsp")
            .ok_or_else(|| "lammps-lsp not found in PATH. Install it from https://github.com/crack-time/lammps-lsp/releases".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(LammpsExtension);
