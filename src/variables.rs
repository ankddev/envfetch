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
    validate_var_name(key).map_err(ErrorKind::NameValidationError)?;

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

/// Add value to existing variable or create new one
pub fn add_variable(
    key: &str,
    value: &str,
    global: bool,
    process: Option<String>,
) -> Result<(), ErrorKind> {
    validate_var_name(key).map_err(ErrorKind::NameValidationError)?;

    let current_value = env::var(key).unwrap_or_default();
    let new_value = format!("{}{}", current_value, value);

    set_variable(key, &new_value, global, process)
}

/// Delete variable with given name
pub fn delete_variable(name: String, global: bool) -> Result<(), ErrorKind> {
    validate_var_name(&name).map_err(ErrorKind::NameValidationError)?;

    if global {
        if let Err(err) = globalenv::unset_var(&name) {
            return Err(ErrorKind::CannotDeleteVariableGlobally(err.to_string()));
        }
    } else {
        unsafe { env::remove_var(&name) };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_variables_list() {
        unsafe { env::set_var("TEST_GET_VARIABLES", "test_value") };
        let list = get_variables();
        assert!(list.contains(&("TEST_GET_VARIABLES".to_string(), "test_value".to_string())));
        unsafe { env::remove_var("TEST_GET_VARIABLES") };
    }

    #[test]
    fn test_set_variable_simple() {
        let result = set_variable("TEST_VAR", "test_value", false, None);
        assert!(result.is_ok());
        assert_eq!(env::var("TEST_VAR").unwrap(), "test_value");
        unsafe { env::remove_var("TEST_VAR") };
    }

    #[test]
    fn test_set_variable_with_process() {
        #[cfg(windows)]
        let cmd = "cmd /C echo test";
        #[cfg(not(windows))]
        let cmd = "echo test";

        let result = set_variable("TEST_PROC_VAR", "test_value", false, Some(cmd.to_string()));
        assert!(result.is_ok());
        assert_eq!(env::var("TEST_PROC_VAR").unwrap(), "test_value");
        unsafe { env::remove_var("TEST_PROC_VAR") };
    }

    #[test]
    fn test_print_env() {
        unsafe { env::set_var("TEST_PRINT_VAR", "test_value") };
        let mut buffer = vec![];
        print_env("{name} = \"{value}\"", &mut buffer);
        assert!(
            String::from_utf8(buffer)
                .unwrap()
                .contains("TEST_PRINT_VAR = \"test_value\"")
        );
        unsafe { env::remove_var("TEST_PRINT_VAR") };
    }

    #[test]
    fn test_add_variable_new() {
        let result = add_variable("TEST_ADD_NEW", "value", false, None);
        assert!(result.is_ok());
        assert_eq!(env::var("TEST_ADD_NEW").unwrap(), "value");
        unsafe { env::remove_var("TEST_ADD_NEW") };
    }

    #[test]
    fn test_add_variable_existing() {
        unsafe { env::set_var("TEST_ADD_EXISTING", "initial") };
        let result = add_variable("TEST_ADD_EXISTING", "_added", false, None);
        assert!(result.is_ok());
        assert_eq!(env::var("TEST_ADD_EXISTING").unwrap(), "initial_added");
        unsafe { env::remove_var("TEST_ADD_EXISTING") };
    }

    #[test]
    fn test_delete_variable() {
        unsafe { env::set_var("TEST_DELETE_VAR", "test_value") };
        let result = delete_variable("TEST_DELETE_VAR".to_string(), false);
        assert!(result.is_ok());
        assert!(env::var("TEST_DELETE_VAR").is_err());
    }
}