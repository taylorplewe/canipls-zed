use zed_extension_api as zed;

struct Canipls {}

impl Canipls {
    fn get_os_bin_word(os: zed::Os) -> String {
        match os {
            zed::Os::Windows => "windows".to_string(),
            zed::Os::Linux => "linux".to_string(),
            zed::Os::Mac => "macos".to_string(),
        }
    }

    fn get_arch_bin_word(arch: zed::Architecture) -> String {
        match arch {
            zed::Architecture::X8664 => "x86_64".to_string(),
            zed::Architecture::Aarch64 => "aarch64".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_latest_canipls_version() {
        let (os, arch) = zed::current_platform();
        let bin_os = Canipls::get_os_bin_word(os);
        let bin_arch = Canipls::get_arch_bin_word(arch);
    }
}

impl zed::Extension for Canipls {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed::Command {
            command: "C:\\Users\\tplew\\webroot\\canipls\\zig-out\\bin\\canipls.exe".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(Canipls);
