use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

struct RustfmtExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for RustfmtExtension {
    fn new() -> Self {
        RustfmtExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = &self.cached_binary_path {
            return Ok(zed::Command {
                command: path.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        let settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), _worktree)?;

        if let Some(binary_settings) = settings.binary.as_ref() {
            if let Some(path) = &binary_settings.path {
                return Ok(zed::Command {
                    command: path.clone(),
                    args: binary_settings.arguments.clone().unwrap_or_default(),
                    env: Default::default(),
                });
            }
        }

        let (os, arch) = zed::current_platform();

        let asset_name = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "rustfmt-lsp-server-aarch64-macos",
            (zed::Os::Mac, zed::Architecture::X8664) => "rustfmt-lsp-server-x86_64-macos",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "rustfmt-lsp-server-aarch64-linux",
            (zed::Os::Linux, zed::Architecture::X8664) => "rustfmt-lsp-server-x86_64-linux",
            (zed::Os::Windows, zed::Architecture::X8664) => "rustfmt-lsp-server-x86_64-windows.exe",
            _ => return Err("unsupported platform".into()),
        };

        let release = zed::latest_github_release(
            "MysticalMike60t/rustfmt-lsp-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset found for {}", asset_name))?;

        let binary_path = format!("rustfmt-lsp-server-{}", release.version);

        if !fs::metadata(&binary_path).is_ok() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )?;

            zed::make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        #[allow(warnings)] completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        None
    }
}

zed::register_extension!(RustfmtExtension);
