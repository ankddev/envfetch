use std::env;
use similar_string::compare_similarity;
use dirs::home_dir;
use std::fs;

#[cfg(windows)]
use winreg::{enums::*, RegKey};

const SIMILARITY_THRESHOLD: f64 = 0.6;

pub fn find_similar_vars(key: &str) -> Option<Vec<String>> {
    let similar = env::vars()
        .map(|el| el.0)
        .filter(|el| {
            compare_similarity(
                key.to_lowercase(),
                el.to_lowercase(),
            ) > SIMILARITY_THRESHOLD
        })
        .collect::<Vec<_>>();
    
    if similar.is_empty() {
        None
    } else {
        Some(similar)
    }
}

#[cfg(windows)]
pub fn set_permanent_env(key: &str, value: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    validate_env_var(key, value)?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey(r"Environment")?;
    
    match value {
        Some(val) => env.set_value(key, &val.to_string())?,
        None => env.delete_value(key)?,
    }
    
    notify_windows_env_change();
    Ok(())
}

#[cfg(windows)]
fn notify_windows_env_change() {
    unsafe {
        winapi::um::winuser::SendMessageTimeoutW(
            winapi::um::winuser::HWND_BROADCAST,
            winapi::um::winuser::WM_SETTINGCHANGE,
            0,
            "Environment\0".as_ptr() as winapi::shared::minwindef::LPARAM,
            winapi::um::winuser::SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );
    }
}

#[cfg(not(windows))]
pub fn set_permanent_env(key: &str, value: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    validate_env_var(key, value)?;
    
    let rc_path = get_rc_path()?;
    let mut content = fs::read_to_string(&rc_path).unwrap_or_default();
    
    remove_existing_var(&mut content, key);
    
    if let Some(val) = value {
        #[cfg(target_os = "macos")]
        content.push_str(&format!("export {}='{}'\n", key, val));
        #[cfg(not(target_os = "macos"))]
        content.push_str(&format!("export {}=\"{}\"\n", key, val));
    }
    
    fs::write(rc_path, content)?;
    Ok(())
}

fn validate_env_var(key: &str, value: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if key.contains(' ') {
        return Err("Variable name cannot contain spaces".into());
    }
    
    if let Some(val) = value {
        if val.contains("==") {
            return Err("Invalid variable format: value contains double equals".into());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn get_rc_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let home = home_dir().ok_or("Cannot find home directory")?;
    Ok(home.join(".bashrc"))
}

#[cfg(not(windows))]
fn remove_existing_var(content: &mut String, key: &str) {
    let patterns = [
        format!("export {}=", key),
        format!("export {}=\"", key),
        format!("export {}='", key),
    ];
    
    for pattern in patterns.iter() {
        if let Some(pos) = content.find(pattern) {
            if let Some(end) = content[pos..].find('\n') {
                content.replace_range(pos..pos+end+1, "");
            }
        }
    }
} 