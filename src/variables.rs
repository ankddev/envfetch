use std::{env, io::Write};

use crate::{models::ErrorKind, utils::*};

/// List of variables
type VariablesList = Vec<(String, String)>;

/// Print all environment variables
pub fn print_env<W: Write>(format: &str, mut buffer: W) {
    for (key, value) in get_variables() {
        let entry = format.replace("{name}", &key).replace("{value}", &value);
        writeln!(buffer, "{}", entry).expect("Failed to write to buffer");
    }
}

/// Get list of environment variables with values
pub fn get_variables() -> VariablesList {
    env::vars().collect()
}

/// Set variable with given key and value
pub fn set_variable(
    key: &str,
    value: &str,
    global: bool,
    process: Option<String>,
) -> Result<(), ErrorKind> {
    // Input validation
    if key.is_empty() {
        return Err(ErrorKind::InvalidInput("Variable key cannot be empty".to_string()));
    }

    // Check for invalid characters (optional, adjust as needed)
    if key.contains('=') || key.contains('\0') {
        return Err(ErrorKind::InvalidInput("Invalid characters in variable name".to_string()));
    }

    if global {
        if let Err(err) = globalenv::set_var(key, value) {
            return Err(ErrorKind::CannotSetVariableGlobally(err.to_string()));
        }
    } else {
        unsafe { env::set_var(key, value) };
    }

    if let Some(process) = process {
        return run(process, false);
    }
    Ok(())
}

/// Update variable - allows partial updates
pub fn update_variable(
    old_key: &str, 
    new_key: Option<&str>, 
    new_value: Option<&str>, 
    global: bool
) -> Result<(), ErrorKind> {
    // If no changes specified, return early
    if new_key.is_none() && new_value.is_none() {
        return Ok(());
    }

    // Get current value if key exists
    let current_value = env::var(old_key).ok();

    // Determine the key to use
    let key_to_use = new_key.unwrap_or(old_key);
    
    // Determine the value to use
    let value_to_use = new_value.or(current_value);

    // If we have a value, set the variable
    if let Some(value) = value_to_use {
        // If key changed, delete the old one first
        if new_key.is_some() && new_key.unwrap() != old_key {
            delete_variable(old_key.to_string(), global)?;
        }

        // Set the new or updated variable
        set_variable(key_to_use, &value, global, None)
    } else {
        Err(ErrorKind::InvalidInput("Cannot update variable without a value".to_string()))
    }
}

/// Delete variable with given name
pub fn delete_variable(name: String, global: bool) -> Result<(), ErrorKind> {
    // Check if variable exists before attempting to delete
    if global {
        if let Err(err) = globalenv::unset_var(&name) {
            return Err(ErrorKind::CannotDeleteVariableGlobally(err.to_string()));
        }
    } else {
        // Only attempt to remove if the variable exists
        if env::var(&name).is_ok() {
            unsafe { env::remove_var(&name) };
        } else {
            return Err(ErrorKind::InvalidInput(format!("Variable '{}' does not exist", name)));
        }
    }
    Ok(())
}

/// Find a variable by its key
pub fn find_variable(key: &str) -> Option<(String, String)> {
    env::vars().find(|(k, _)| k == key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_update_variable_key() {
        // Set initial variable
        unsafe { env::set_var("OLD_TEST_KEY", "test_value") };

        // Update key
        let result = update_variable("OLD_TEST_KEY", Some("NEW_TEST_KEY"), None, false);
        
        assert!(result.is_ok());
        assert_eq!(env::var("NEW_TEST_KEY").unwrap(), "test_value");
        assert!(env::var("OLD_TEST_KEY").is_err());

        // Cleanup
        unsafe { env::remove_var("NEW_TEST_KEY") };
    }

    #[test]
    fn test_update_variable_value() {
        // Set initial variable
        unsafe { env::set_var("TEST_UPDATE_VAR", "old_value") };

        // Update value
        let result = update_variable("TEST_UPDATE_VAR", None, Some("new_value"), false);
        
        assert!(result.is_ok());
        assert_eq!(env::var("TEST_UPDATE_VAR").unwrap(), "new_value");

        // Cleanup
        unsafe { env::remove_var("TEST_UPDATE_VAR") };
    }

    #[test]
    fn test_update_variable_both() {
        // Set initial variable
        unsafe { env::set_var("OLD_KEY", "test_value") };

        // Update both key and value
        let result = update_variable("OLD_KEY", Some("NEW_KEY"), Some("new_value"), false);
        
        assert!(result.is_ok());
        assert_eq!(env::var("NEW_KEY").unwrap(), "new_value");
        assert!(env::var("OLD_KEY").is_err());

        // Cleanup
        unsafe { env::remove_var("NEW_KEY") };
    }

}