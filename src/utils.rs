use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::models::ErrorKind;
use subprocess::Exec;
use log::error;

/// Runs given command using system shell
pub fn run(process: String) -> Result<(), ErrorKind> {
    let mut error = None;
    let result = Exec::shell(process).join().unwrap_or_else(|_| {
        error!("can't start process");
        error = Some(ErrorKind::ProcessFailed);
        subprocess::ExitStatus::Exited(1)
    });

    // Workaround
    if let Some(ErrorKind::StartingProcessError) = error {
        return Err(ErrorKind::StartingProcessError);
    }

    // Exit with non-zero exit code if process did not successful
    if !result.success() {
        return Err(ErrorKind::ProcessFailed);
    }
    Ok(())
}

/// Validate variable name
pub fn validate_var_name(name: &str) -> Result<(), String> {
    if name.contains(' ') {
        return Err("Variable name cannot contain spaces".into());
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
            result.is_ok(),
            "Empty string should be valid as per current implementation"
        );
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
    fn test_find_similar_string_multiple_matches() {
        let strings = vec![
            "TEST".to_string(),
            "TSET".to_string(),
            "TEXT".to_string(),
            "NONE".to_string(),
        ];
        let result = find_similar_string("TEST".to_string(), strings, 0.5);
        assert!(result.contains(&"TEST".to_string()));
        assert!(result.contains(&"TEXT".to_string()));
        assert!(result.contains(&"TSET".to_string()));
        assert!(!result.contains(&"NONE".to_string()));
    }
}
