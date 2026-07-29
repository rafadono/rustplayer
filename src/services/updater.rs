use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateChannel {
    Stable,
    Beta,
}

impl UpdateChannel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Stable => "Stable",
            Self::Beta => "Beta",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub notes: String,
    #[serde(default)]
    pub channel: String,
}

pub fn check_for_updates(_channel: UpdateChannel) -> Result<UpdateInfo, String> {
    let url = match _channel {
        UpdateChannel::Stable => "https://example.com/rplayer/stable.json",
        UpdateChannel::Beta => "https://example.com/rplayer/beta.json",
    };
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(6))
        .call()
        .map_err(|e| format!("update check failed: {}", e))?;
    let text = resp
        .into_string()
        .map_err(|e| format!("invalid update response: {}", e))?;
    serde_json::from_str::<UpdateInfo>(&text).map_err(|e| format!("invalid update json: {}", e))
}

pub fn is_newer(current: &str, latest: &str) -> bool {
    let parse = |s: &str| -> Vec<u32> {
        s.trim_start_matches('v')
            .split('.')
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect()
    };
    let a = parse(current);
    let b = parse(latest);
    let n = a.len().max(b.len());
    for i in 0..n {
        let av = *a.get(i).unwrap_or(&0);
        let bv = *b.get(i).unwrap_or(&0);
        if bv > av {
            return true;
        }
        if bv < av {
            return false;
        }
    }
    false
}

pub fn download_update_to_temp(url: &str) -> Result<PathBuf, String> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(20))
        .call()
        .map_err(|e| format!("download failed: {}", e))?;
    let mut bytes: Vec<u8> = Vec::new();
    let mut reader = resp.into_reader();
    std::io::copy(&mut reader, &mut bytes).map_err(|e| format!("read failed: {}", e))?;
    if bytes.len() < 64 * 1024 {
        return Err("download demasiado pequeño".into());
    }
    let path = std::env::temp_dir().join("rplayer_update_new.exe");
    let mut f = std::fs::File::create(&path).map_err(|e| format!("write failed: {}", e))?;
    f.write_all(&bytes)
        .map_err(|e| format!("write failed: {}", e))?;
    Ok(path)
}

pub fn install_update_with_rollback(download_url: &str, current_exe: &Path) -> Result<(), String> {
    let new_exe = download_update_to_temp(download_url)?;
    apply_update_with_rollback(&new_exe, current_exe)
}

#[cfg(target_os = "windows")]
pub fn apply_update_with_rollback(new_exe: &Path, current_exe: &Path) -> Result<(), String> {
    use std::fs;

    let backup = current_exe.with_extension("bak");
    fs::copy(current_exe, &backup).map_err(|e| format!("backup failed: {}", e))?;

    let script = updater_script_path(current_exe)?;
    let script_body = format!(
        "@echo off\r\n\
         timeout /t 1 /nobreak >nul\r\n\
         copy /Y \"{new}\" \"{cur}\" >nul\r\n\
         if errorlevel 1 (\r\n\
           copy /Y \"{bak}\" \"{cur}\" >nul\r\n\
           exit /b 1\r\n\
         )\r\n\
         \"{cur}\" --self-check >nul 2>nul\r\n\
         if errorlevel 1 (\r\n\
           copy /Y \"{bak}\" \"{cur}\" >nul\r\n\
           exit /b 1\r\n\
         )\r\n\
         start \"\" \"{cur}\"\r\n",
        new = new_exe.display(),
        cur = current_exe.display(),
        bak = backup.display()
    );
    fs::write(&script, script_body).map_err(|e| format!("script write failed: {}", e))?;
    Command::new("cmd")
        .args(["/C", script.to_string_lossy().as_ref()])
        .spawn()
        .map_err(|e| format!("updater launch failed: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn apply_update_with_rollback(new_exe: &Path, current_exe: &Path) -> Result<(), String> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let backup = current_exe.with_extension("bak");
    fs::copy(current_exe, &backup).map_err(|e| format!("backup failed: {}", e))?;

    let current_meta = fs::metadata(current_exe).map_err(|e| format!("metadata failed: {}", e))?;
    let mode = current_meta.permissions().mode();

    if let Err(e) = fs::copy(new_exe, current_exe) {
        let _ = fs::copy(&backup, current_exe);
        return Err(format!("replace failed: {}", e));
    }
    let _ = fs::set_permissions(current_exe, fs::Permissions::from_mode(mode));

    let health = Command::new(current_exe)
        .arg("--self-check")
        .status()
        .map_err(|e| format!("health check failed to run: {}", e))?;

    if !health.success() {
        let _ = fs::copy(&backup, current_exe);
        return Err("new version failed self-check; rolled back".into());
    }

    let _ = fs::remove_file(&backup);
    Ok(())
}

#[cfg(target_os = "windows")]
fn updater_script_path(current_exe: &Path) -> Result<PathBuf, String> {
    let dir = current_exe
        .parent()
        .ok_or_else(|| "exe path sin parent".to_string())?;
    Ok(dir.join("rplayer_update.cmd"))
}
