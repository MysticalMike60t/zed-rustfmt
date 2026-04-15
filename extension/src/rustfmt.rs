use zed_extension_api::{self as zed, Result};

struct RustfmtLSP;

impl zed::Extension for RustfmtLSP {
    fn new() -> Self {
        RustfmtLSP
    }

    fn language_server_command(
        &mut self,
        #[allow(warnings)] id: &zed::LanguageServerId,
        #[allow(warnings)] worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "rustfmt-lsp-server".into(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RustfmtLSP);
