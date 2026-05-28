use std::fs;

use zed_extension_api::{self as zed, GithubRelease};

const CANIPLS_REPO: &str = "taylorplewe/canipls";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SemVer {
    major: usize,
    minor: usize,
    patch: usize,
}
#[derive(Debug)]
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

        let index_after_v: usize = if sem_ver_string.starts_with('v') {
            1
        } else {
            0
        };
        let mut it = sem_ver_string[index_after_v..].split('.');

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

    fn get_canipls_exe_path(language_server_id: &zed::LanguageServerId) -> Result<String, String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let (current_os, current_arch) = zed::current_platform();
        let current_bin_os = Canipls::get_os_bin_word(current_os);
        let current_bin_arch = Canipls::get_arch_bin_word(current_arch);

        let mut should_download_latest = true;

        // get latest version number
        let latest_sem_ver_str: &str;
        let latest_sem_ver: SemVer;
        let latest_release: GithubRelease;
        if let Ok(release) = zed::latest_github_release(
            CANIPLS_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            latest_release = release;
            latest_sem_ver_str = latest_release.version.as_str();
            if let Ok(sem_ver) = SemVer::from_string(latest_release.version.as_str()) {
                latest_sem_ver = sem_ver;
            } else {
                return Err(format!(
                    "GitHub release tag was not a valid semver string: \"{}\"",
                    latest_release.version,
                ));
            }
        } else {
            return Err(format!(
                "could not get latest GitHub release from repo \"{}\"",
                CANIPLS_REPO
            ));
        }

        // compare against installed version number
        if let Some(is_old) = Canipls::is_installed_version_old(latest_sem_ver) {
            should_download_latest = is_old;
        }

        // os-specific stuff
        let mut path_separator = "/";
        let mut exe_extension = "";
        let mut download_file_type = zed::DownloadedFileType::GzipTar;
        if current_os == zed::Os::Windows {
            path_separator = "\\";
            exe_extension = ".exe";
            download_file_type = zed::DownloadedFileType::Zip;
        }

        let exe_dir = format!("canipls-{}", latest_sem_ver_str);
        let exe_path = format!("{}{}canipls{}", exe_dir, path_separator, exe_extension);

        if should_download_latest {
            // find correct asset to download based on our arch & os
            let Some(asset) = latest_release.assets.iter().find(|asset| {
                if asset.name.split('-').collect::<Vec<&str>>().len() < 4 {
                    return false;
                }
                let mut it = asset.name.split('-');
                _ = it.next(); // "canipls"
                _ = it.next(); // (version)
                let Some(asset_arch) = it.next() else {
                    return false;
                };
                let Some(asset_os_with_ext) = it.next() else {
                    return false;
                };
                // remove archive extension
                let Some(asset_os) = asset_os_with_ext.split('.').next() else {
                    return false;
                };

                if asset_arch == current_bin_arch && asset_os == current_bin_os {
                    return true;
                }

                false
            }) else {
                return Err(format!(
                    "could not find a canipls release with OS \"{}\" and architecture \"{}\"",
                    current_bin_os, current_bin_arch
                ));
            };

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // download exe (Zed takes care of extracting it for us)
            if let Err(e) =
                zed::download_file(&asset.download_url, exe_dir.as_str(), download_file_type)
            {
                return Err(format!("could not download canipls release archive: {}", e));
            }

            if let Err(e) = zed::make_file_executable(&exe_path) {
                return Err(format!("could not make canipls binary executable: {}", e));
            }

            // clear out other installed versions
            let entries = fs::read_dir(".").map_err(|e| {
                format!(
                    "could not read canipls extension directory for cleanup: {}",
                    e
                )
            })?;
            for entry in entries.flatten() {
                if entry.file_name().to_str() != Some(exe_dir.as_str()) {
                    _ = fs::remove_dir_all(entry.path());
                }
            }
        }

        Ok(exe_path)
    }

    fn is_installed_version_old(latest_sem_ver: SemVer) -> Option<bool> {
        let entries = fs::read_dir(".").ok()?;
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name_str = file_name.to_str()?;
            if name_str.starts_with("canipls-") && entry.path().is_dir() {
                let installed_sem_ver_str = name_str.strip_prefix("canipls-")?;
                let installed_sem_ver = SemVer::from_string(installed_sem_ver_str).ok()?;
                if latest_sem_ver <= installed_sem_ver {
                    return Some(false);
                } else {
                    return None;
                }
            }
        }
        None
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
        language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        match Canipls::get_canipls_exe_path(language_server_id) {
            Ok(exe_path) => Ok(zed::Command {
                command: exe_path,
                args: vec![],
                env: vec![],
            }),
            Err(e) => Err(e),
        }
    }
}

zed::register_extension!(Canipls);
