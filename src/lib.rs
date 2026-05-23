use std::fs;

use zed_extension_api as zed;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SemVer {
    major: usize,
    minor: usize,
    patch: usize,
}
enum SemVerError {
    InvalidString,
}
impl SemVer {
    fn from_string(sem_ver_string: &str) -> Result<Self, SemVerError> {
        let mut sem_ver: SemVer = SemVer {
            major: 0,
            minor: 0,
            patch: 0,
        };

        let cursor: usize = if sem_ver_string.starts_with('v') {
            1
        } else {
            0
        };
        let mut it = sem_ver_string[cursor..].split('.');

        // major
        if let Some(major_str) = it.next() {
            match major_str.parse::<usize>() {
                Ok(major) => sem_ver.major = major,
                Err(_) => return Err(SemVerError::InvalidString),
            }
        } else {
            return Err(SemVerError::InvalidString);
        }

        // minor
        if let Some(minor_str) = it.next() {
            match minor_str.parse::<usize>() {
                Ok(minor) => sem_ver.minor = minor,
                Err(_) => return Err(SemVerError::InvalidString),
            }
        } else {
            return Err(SemVerError::InvalidString);
        }

        // patch
        if let Some(patch_str) = it.next() {
            match patch_str.parse::<usize>() {
                Ok(patch) => sem_ver.patch = patch,
                Err(_) => return Err(SemVerError::InvalidString),
            }
        } else {
            return Err(SemVerError::InvalidString);
        }

        return Ok(sem_ver);
    }
}

struct Canipls {}

const LATEST_FILENAME: &str = "latest";

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

    fn get_latest_canipls_version() -> Option<String> {
        let (os, arch) = zed::current_platform();
        let bin_os = Canipls::get_os_bin_word(os);
        let bin_arch = Canipls::get_arch_bin_word(arch);

        let mut should_download_latest = false;

        // compare against latest installed version
        if let Ok(installed_version_str) = fs::read_to_string(LATEST_FILENAME) {
        } else {
            should_download_latest = true;
        }

        if let Ok(sem_ver) = SemVer::from_string("v1.2.3") {
            // TODO: how to debug lol I can't find where this outputs to:
            println!(
                "sem ver: {}.{}.{}",
                sem_ver.major, sem_ver.minor, sem_ver.patch
            );
        }

        None
    }
}

impl zed::Extension for Canipls {
    fn new() -> Self
    where
        Self: Sized,
    {
        _ = Canipls::get_latest_canipls_version();
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
