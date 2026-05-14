use zed_extension_api as zed;

struct CaniuseLs {}

impl zed::Extension for CaniuseLs {
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
            command: "C:\\Users\\TaylorPlewe\\webroot\\canipls\\zig-out\\bin\\canipls.exe"
                .to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(CaniuseLs);
