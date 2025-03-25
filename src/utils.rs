use crate::models::ErrorKind;
use log::error;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use subprocess::Exec;

/// Runs given command using system shell
pub fn run(process: String, capture: bool) -> Result<(), ErrorKind> {
    let cmd = Exec::shell(process);
    let result = if capture {
        cmd.capture()
    } else {
        cmd.join().map(|status| subprocess::CaptureData {
            stdout: Vec::new(),
            stderr: Vec::new(),
            exit_status: status,
        })
    }
    .map_err(|_| {
        error!("can't start process");
        ErrorKind::StartingProcessError
    })?;

    if !result.exit_status.success() {
        return Err(ErrorKind::ProcessFailed);
    }
    Ok(())
}

/// Validate variable name
pub fn validate_var_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Variable name cannot be empty".to_string());
    }
    if name.contains(' ') {
        return Err("Variable name cannot contain spaces".to_string());
    }
    Ok(())
}

/// Returns vector of string that are similar by threshold to given string in given vector
pub fn find_similar_string(string: String, strings: Vec<String>, threshold: f64) -> Vec<String> {
    strings
        .par_iter()
        .filter(|name| {
            similar_string::compare_similarity(string.to_lowercase(), name.to_lowercase())
                > threshold
        })
        .map(|name| name.to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_var_name_valid() {
        let valid_names = vec![
            "VALID_NAME",
            "MY_VAR_123",
            "PATH",
            "_HIDDEN",
            "VALID_NAME_WITH_NUMBERS_123",
            "A", // Single character
        ];

        for name in valid_names {
            assert!(validate_var_name(name).is_ok());
        }
    }

    #[test]
    fn test_validate_var_name_with_spaces() {
        let invalid_names = vec![
            "INVALID NAME",
            "MY VAR",
            " LEADING_SPACE",
            "TRAILING_SPACE ",
            "MULTIPLE   SPACES",
        ];

        for name in invalid_names {
            let result = validate_var_name(name);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Variable name cannot contain spaces");
        }
    }

    #[test]
    fn test_validate_var_name_empty() {
        let result = validate_var_name("");
        assert!(
            result.is_err(),
            "Empty string should be invalid as per current implementation"
        );
        assert_eq!(result.unwrap_err(), "Variable name cannot be empty");
    }

    #[test]
    fn test_find_similar_string_exact_match() {
        let strings = vec!["PATH".to_string(), "HOME".to_string(), "USER".to_string()];
        let result = find_similar_string("PATH".to_string(), strings, 0.8);
        assert_eq!(result, vec!["PATH"]);
    }

    #[test]
    fn test_find_similar_string_case_insensitive() {
        let strings = vec!["PATH".to_string(), "HOME".to_string(), "USER".to_string()];
        let result = find_similar_string("path".to_string(), strings, 0.8);
        assert_eq!(result, vec!["PATH"]);
    }

    #[test]
    fn test_find_similar_string_no_match() {
        let strings = vec!["PATH".to_string(), "HOME".to_string(), "USER".to_string()];
        let result = find_similar_string("XXXXXX".to_string(), strings, 0.8);
        assert!(result.is_empty());
    }

    #[test]
    fn test_run_successful_command() {
        #[cfg(windows)]
        let cmd = "cmd /C echo test";
        #[cfg(not(windows))]
        let cmd = "echo test";

        let result = run(cmd.to_string(), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_nonexistent_command() {
        let result = run("nonexistent_command_123".to_string(), true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ErrorKind::ProcessFailed));
    }
}