//! Integration tests for CLI

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::env;
use std::process::Command;
use std::fs;
use dirs;

#[test]
/// Test for set command if specified process is successful
/// Check if variable is set and envfetch exits with 0
/// We check it separately for Windows and Unix, because commands are different
fn set_command_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("set").arg("MY_VAR").arg("Hello");
    // Windows
    #[cfg(target_os = "windows")]
    cmd.arg("echo %MY_VAR%")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    // Linux and macOS
    #[cfg(not(target_os = "windows"))]
    cmd.arg("echo $MY_VAR")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
    Ok(())
}

#[test]
/// Test for set command if specified process is not successful
/// Check if envfetch exits with non-zero exit code
fn set_command_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("set").arg("MY_VARR").arg("Hello");
    // We can use only Windows command here because it should fail
    cmd.arg("%MY_VARIABLE%").assert().failure();
    Ok(())
}

#[test]
/// Test for get command if specified variable exists
fn get_variable_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    env::set_var("MY_VAR", "Hello");
    cmd.arg("get").arg("MY_VAR");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
    Ok(())
}

#[test]
/// Test for get command if specified variable doesn't exist
fn get_variable_doesnt_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("get").arg("MY_VARIABLE");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: can't find 'MY_VARIABLE'"));
    Ok(())
}

#[test]
/// Test for get command if specified variable doesn't exist and showing similar variables is enabled
fn get_variable_doesnt_exists_similar_enabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    env::set_var("MY_VARIABLEE", "Hello");
    cmd.arg("get").arg("MY_VARIABLE");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: can't find 'MY_VARIABLE'"))
        .stderr(predicate::str::contains("Did you mean:"))
        .stderr(predicate::str::contains("MY_VARIABLEE"));
    Ok(())
}

#[test]
/// Test for get command if specified variable doesn't exist and showing similar variables is disabled
fn get_variable_doesnt_exists_similar_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    env::set_var("MY_VARIABLEE", "Hello");
    cmd.arg("get").arg("MY_VARIABLE").arg("--no-similar-names");
    cmd.assert().failure();
    Ok(())
}

#[test]
/// Test for print command
fn print_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    env::set_var("PRINT_TEST", "Print");
    cmd.arg("print")
        .assert()
        .success()
        .stdout(predicate::str::contains("PRINT_TEST = \"Print\""));
    Ok(())
}

#[test]
/// Test for delete command if specified process is successful
fn delete_command_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    env::set_var("MY_VAR", "Hello");
    cmd.arg("delete").arg("MY_VAR");
    // Windows
    #[cfg(target_os = "windows")]
    cmd.arg("echo 'Hello'")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    // Linux and macOS
    #[cfg(not(target_os = "windows"))]
    cmd.arg("echo 'Hello'")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"));
    Ok(())
}

#[test]
/// Test for load command if file doesn't exist and exit on error flag is enabled
fn load_file_dont_found_with_exit_on_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("load");
    cmd.arg("--exit-on-error");
    cmd.arg("echo %MY_ENV_VAR%").assert().failure();
    Ok(())
}



#[test]
/// Test for load command if custom file exist
fn load_custom_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    let file = assert_fs::NamedTempFile::new(".env.test")?;
    file.write_str("MY_ENV_VAR='TEST'\nTEST='hello'")?;
    cmd.arg("load").arg("--file").arg(file.path());
    // Windows
    #[cfg(target_os = "windows")]
    cmd.arg("echo %MY_ENV_VAR%")
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST"));

    // Linux and macOS
    #[cfg(not(target_os = "windows"))]
    cmd.arg("echo $MY_ENV_VAR")
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST"));
    // Close file after test
    file.close().unwrap();
    Ok(())
}

#[test]
/// Test for load command if custom file exist and specified process failed
fn load_custom_file_exists_command_failed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    let file = assert_fs::NamedTempFile::new(".env.test")?;
    file.assert(predicate::path::missing());
    file.write_str("MY_ENV_VAR='TEST'\nTEST='hello'")?;
    cmd.arg("load").arg("--file").arg(file.path());
    // Windows
    #[cfg(target_os = "windows")]
    cmd.arg("echo %MY_ENV_VAR_TEST%")
        .assert()
        .success()
        .stdout(predicate::str::contains("%MY_ENV_VAR_TEST%"));

    // Linux and macOS
    #[cfg(not(target_os = "windows"))]
    cmd.arg("(exit 1)").assert().failure();
    // Close file after test
    file.close().unwrap();
    Ok(())
}

#[test]
/// Test for load command if custom file doesn't exist
fn load_custom_file_doesnt_exists() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("load").arg("--file").arg(".env.production");
    // Windows
    #[cfg(target_os = "windows")]
    cmd.arg("echo %MY_ENV_VAR%").assert().failure();

    // Linux and macOS
    #[cfg(not(target_os = "windows"))]
    cmd.arg("echo $MY_VARIABLE").assert().failure();
    Ok(())
}

#[test]
/// Test for gset command - setting variable permanently
fn gset_command_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("gset").arg("GSET_TEST_VAR").arg("GlobalValue");
    cmd.assert().success();

    // Check Windows registry
    #[cfg(windows)]
    {
        let output = std::process::Command::new("reg")
            .args(&["query", "HKCU\\Environment", "/v", "GSET_TEST_VAR"])
            .output()?;
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("GlobalValue"));
    }

    // Check shell rc file on Unix
    #[cfg(not(windows))]
    {
        let home = dirs::home_dir().unwrap();
        let rc_files = vec![".bashrc", ".zshrc", "config.fish"];
        let mut found = false;
        
        for rc in rc_files {
            let rc_path = if rc == "config.fish" {
                home.join(".config").join("fish").join(rc)
            } else {
                home.join(rc)
            };
            
            if rc_path.exists() {
                let content = fs::read_to_string(rc_path)?;
                if content.contains(&format!("export GSET_TEST_VAR=GlobalValue")) {
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "Variable not found in any rc file");
    }
    Ok(())
}

#[test]
/// Test for gdelete command - deleting variable permanently
fn gdelete_command_success() -> Result<(), Box<dyn std::error::Error>> {
    // First set the variable
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("gset").arg("GDELETE_TEST_VAR").arg("ToBeDeleted");
    cmd.assert().success();
    
    // Then delete it
    let mut delete_cmd = Command::cargo_bin("envfetch")?;
    delete_cmd.arg("gdelete").arg("GDELETE_TEST_VAR");
    delete_cmd.assert().success();
    
    // Verify the variable was deleted
    #[cfg(windows)]
    {
        let output = std::process::Command::new("reg")
            .args(&["query", "HKCU\\Environment", "/v", "GDELETE_TEST_VAR"])
            .output()?;
        assert!(!output.status.success()); // Should fail as variable doesn't exist
    }

    #[cfg(not(windows))]
    {
        let home = dirs::home_dir().unwrap();
        let rc_files = vec![".bashrc", ".zshrc", "config.fish"];
        let mut found = false;
        
        for rc in rc_files {
            let rc_path = if rc == "config.fish" {
                home.join(".config").join("fish").join(rc)
            } else {
                home.join(rc)
            };
            
            if rc_path.exists() {
                let content = fs::read_to_string(rc_path)?;
                if content.contains("GDELETE_TEST_VAR") {
                    found = true;
                    break;
                }
            }
        }
        assert!(!found, "Variable still exists in rc file");
    }
    Ok(())
}

#[test]
/// Test for gload command with valid file
fn gload_command_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    let file = assert_fs::NamedTempFile::new(".env.global.test")?;
    file.write_str("GLOBAL_TEST_VAR='GlobalTest'\nGLOBAL_TEST_VAR2='Hello'")?;
    
    cmd.arg("gload").arg("--file").arg(file.path());
    cmd.assert().success();
    
    // Check if variables were set
    #[cfg(windows)]
    {
        let output = std::process::Command::new("reg")
            .args(&["query", "HKCU\\Environment", "/v", "GLOBAL_TEST_VAR"])
            .output()?;
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("GlobalTest"));
        
        let output2 = std::process::Command::new("reg")
            .args(&["query", "HKCU\\Environment", "/v", "GLOBAL_TEST_VAR2"])
            .output()?;
        assert!(output2.status.success());
        assert!(String::from_utf8_lossy(&output2.stdout).contains("Hello"));
    }

    #[cfg(not(windows))]
    {
        let home = dirs::home_dir().unwrap();
        let rc_files = vec![".bashrc", ".zshrc", "config.fish"];
        let mut found_vars = 0;
        
        for rc in rc_files {
            let rc_path = if rc == "config.fish" {
                home.join(".config").join("fish").join(rc)
            } else {
                home.join(rc)
            };
            
            if rc_path.exists() {
                let content = fs::read_to_string(rc_path)?;
                if content.contains("export GLOBAL_TEST_VAR='GlobalTest'") {
                    found_vars += 1;
                }
                if content.contains("export GLOBAL_TEST_VAR2='Hello'") {
                    found_vars += 1;
                }
            }
        }
        assert!(found_vars == 2, "Not all variables were found in rc files");
    }
    
    file.close()?;
    Ok(())
}

#[test]
/// Test for gload command with invalid file
fn gload_command_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("gload").arg("--file").arg(".env.nonexistent");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
    Ok(())
}

#[test]
/// Test for gload command with invalid file content
fn gload_command_invalid_content() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    let file = assert_fs::NamedTempFile::new(".env.invalid.test")?;
    file.write_str("INVALID==Test")?;
    
    cmd.arg("gload").arg("--file").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid file format: contains double equals"));
    
    file.close()?;
    Ok(())
}

#[test]
/// Test for gset command with invalid variable name
fn gset_command_invalid_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("envfetch")?;
    cmd.arg("gset").arg("INVALID NAME").arg("Value");
    
    #[cfg(windows)]
    {
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("error:"));
    }
    
    #[cfg(not(windows))]
    {
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("error:"));
    }
    Ok(())
}
