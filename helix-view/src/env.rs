pub fn binary_exists(binary_name: &str) -> bool {
    which::which(binary_name).is_ok()
}

#[cfg(not(windows))]
pub fn env_var_is_set(env_var_name: &str) -> bool {
    std::env::var_os(env_var_name).is_some()
}

#[cfg(not(windows))]
pub fn env_var_is_eq(env_var_name: &str, value: &str) -> bool {
    std::env::var_os(env_var_name)
        .iter()
        .all(|v| v.eq_ignore_ascii_case(value))
}
