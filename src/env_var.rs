use duration_str::parse;
use std::{any::TypeId, env, net::IpAddr, path::PathBuf, str::FromStr, time::Duration};
use tracing::Level;

pub fn get_env_value_with_panic<T>(var_name: &str) -> T
where
    T: FromStr + 'static,
{
    let mut var_value_str = env::var(var_name)
        .unwrap_or_else(|_| panic!("Unable to get environment variable. Var Name: {}", var_name));

    if TypeId::of::<T>() == TypeId::of::<bool>() {
        // Bool values should be case insensitive.
        var_value_str = var_value_str.to_lowercase();
    }

    T::from_str(var_value_str.as_str()).unwrap_or_else(|_| {
        panic!(
            "Failed to parse string to {}. Var Name: {}, Raw value: {}",
            std::any::type_name::<T>(),
            var_name,
            var_value_str
        )
    })
}

pub fn env_to_path_with_panic(env_name: &str) -> PathBuf {
    let env_value = get_env_value_with_panic::<String>(env_name);

    PathBuf::from_str(&env_value).unwrap_or_else(|_| {
        panic!(
            "Invalid path representation detected. Var Name: {}., Value: {}.",
            env_name, env_value
        )
    })
}

#[allow(dead_code)]
pub fn env_to_ipaddr_with_panic(env_name: &str) -> IpAddr {
    let env_value = get_env_value_with_panic::<String>(env_name);

    IpAddr::from_str(&env_value).unwrap_or_else(|_| {
        panic!(
            "Invalid IP Address representation detected. Var Name: {}., Value: {}.",
            env_name, env_value
        )
    })
}

#[allow(dead_code)]
pub fn env_to_duration_with_panic(env_name: &str) -> Duration {
    let env_value = get_env_value_with_panic::<String>(env_name);

    parse(env_value.as_str()).unwrap_or_else(|_| {
        panic!(
            "Invalid Duration representation detected. Var Name: {}., Value: {}.",
            env_name, env_value
        )
    })
}

#[allow(dead_code)]
pub fn env_to_log_level_with_panic(env_name: &str) -> Level {
    let env_value = get_env_value_with_panic::<String>(env_name);

    match env_value.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "error" => Level::ERROR,
        "info" => Level::INFO,
        "trace" => Level::TRACE,
        "warn" => Level::WARN,
        _ => panic!(
            "Invalid log level detected. env_name: {}., env_value: {}.",
            env_name, env_value
        ),
    }
}

// Note: There might be some dependency between tests, causing a UT to fail (#[should_panic] does)
// not panic. This does not happen in all runs. It is a known issue which we will try to fix soon.

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    #[should_panic]
    fn parse_non_existent_env_var() {
        get_env_value_with_panic::<i32>("NonExistentEnvName");
    }

    // i32 ---------------------------------------------------------------------
    #[rstest]
    #[case("some_string")]
    #[case("123abc")]
    #[case("1.234")]
    #[case("2147483648")]
    #[case("")]
    #[should_panic]
    fn parse_i32_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            get_env_value_with_panic::<i32>("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("1234", 1234)]
    #[case("0", 0)]
    #[case("-4321", -4321)]
    fn parse_i32_type_success(#[case] value: &str, #[case] expected: i32) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = get_env_value_with_panic::<i32>("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // u32 ---------------------------------------------------------------------
    #[rstest]
    #[case("some_string")]
    #[case("123abc")]
    #[case("1.234")]
    #[case("")]
    #[case("-4321")]
    #[case("4294967296")]
    #[should_panic]
    fn parse_u32_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            get_env_value_with_panic::<u32>("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("1234", 1234)]
    #[case("0", 0)]
    fn parse_u32_type_success(#[case] value: &str, #[case] expected: u32) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = get_env_value_with_panic::<u32>("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // f32 ---------------------------------------------------------------------
    #[rstest]
    #[case("some_string")]
    #[case("123abc")]
    #[case("")]
    #[should_panic]
    fn parse_f32_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            get_env_value_with_panic::<f32>("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("1.234", 1.234)]
    #[case("1234", 1234.0)]
    #[case("0.0", 0.0)]
    #[case("0", 0.0)]
    #[case("-4.321", -4.321)]
    #[case("-4321", -4321.0)]
    fn parse_f32_type_success(#[case] value: &str, #[case] expected: f32) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = get_env_value_with_panic::<f32>("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // bool ---------------------------------------------------------------------
    #[rstest]
    #[case("some_string")]
    #[case("123abc")]
    #[case("1234")]
    #[case("1.234")]
    #[case("")]
    #[should_panic]
    fn parse_bool_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            get_env_value_with_panic::<bool>("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("true", true)]
    #[case("TrUe", true)]
    #[case("false", false)]
    #[case("fAlSe", false)]
    fn parse_bool_type_success(#[case] value: &str, #[case] expected: bool) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = get_env_value_with_panic::<bool>("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // String ------------------------------------------------------------------
    #[rstest]
    #[case("some_string")]
    #[case("123abc")]
    #[case("")]
    fn parse_string_type_success(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = get_env_value_with_panic::<String>("MY_ENV_VAR");
            assert_eq!(result, value);
        });
    }

    // Path --------------------------------------------------------------------
    #[test]
    #[should_panic]
    fn parse_path_type_non_existent_var() {
        env_to_path_with_panic("MY_ENV_VAR");
    }

    #[rstest]
    #[case("some/valid/path")]
    #[case("another/path/")]
    #[case("another_one/")]
    #[case("/another_one/")]
    fn parse_path_type_success(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = env_to_path_with_panic("MY_ENV_VAR");
            assert_eq!(result.display().to_string(), value);
        });
    }

    // IP Address --------------------------------------------------------------
    #[test]
    #[should_panic]
    fn parse_ipaddr_type_non_existent_var() {
        env_to_ipaddr_with_panic("MY_ENV_VAR");
    }

    #[rstest]
    #[case("127.0.0.1:8000")]
    #[case("127.1.1")]
    #[case("2001:db8:85a3:87b2:3590:8a2e:370:7334:db8:85a3:87b2:3590")]
    #[case("2001:3db8:85a3:87b2:3590:8a2e:370:xxxx")]
    #[case("invalid_ip")]
    #[should_panic]
    fn parse_ipaddr_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            env_to_ipaddr_with_panic("MY_ENV_VAR");
        });
    }

    //Note: IPAddr uses shorthand for Ipv6:
    //      "0:0:0:0:0:0:0:1" is shortened to ::1
    //      "0000" is shortened to an empty space
    //      "0db8" is shortened to db8
    #[rstest]
    #[case("127.0.0.1")]
    #[case("::1")]
    #[case("2001:db8:85a3::8a2e:370:7334")]
    #[case("2001:3db8:85a3:87b2:3590:8a2e:370:7334")]
    fn parse_ipaddr_type_success(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = env_to_ipaddr_with_panic("MY_ENV_VAR");
            assert_eq!(result.to_string(), value);
        });
    }

    // Duration ----------------------------------------------------------------
    #[test]
    #[should_panic]
    fn parse_duration_type_non_existent_var() {
        env_to_duration_with_panic("MY_ENV_VAR");
    }

    #[rstest]
    #[case("1")]
    #[case("1.5")]
    #[case("001")]
    #[case("1 s")]
    #[should_panic]
    fn parse_duration_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            env_to_duration_with_panic("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("1s", Duration::from_secs(1))]
    #[case("1m", Duration::from_secs(60))]
    #[case("1second", Duration::from_secs(1))]
    #[case("1m + 1s", Duration::from_secs(61))]
    fn parse_duration_type_success(#[case] value: &str, #[case] expected: Duration) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = env_to_duration_with_panic("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // Log Level ---------------------------------------------------------------
    #[test]
    #[should_panic]
    fn parse_log_level_type_non_existent_var() {
        env_to_log_level_with_panic("MY_ENV_VAR");
    }

    #[rstest]
    #[case("warning")]
    #[case("some_level")]
    #[case("LEVEL")]
    #[should_panic]
    fn parse_log_level_type_failure(#[case] value: &str) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            env_to_log_level_with_panic("MY_ENV_VAR");
        });
    }

    #[rstest]
    #[case("debug", Level::DEBUG)]
    #[case("error", Level::ERROR)]
    #[case("info", Level::INFO)]
    #[case("trace", Level::TRACE)]
    #[case("warn", Level::WARN)]
    #[case("DEBUG", Level::DEBUG)]
    #[case("DEbUg", Level::DEBUG)]
    fn parse_log_level_type_success(#[case] value: &str, #[case] expected: Level) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result = env_to_log_level_with_panic("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    // URL ---------------------------------------------------------------
    #[rstest]
    #[case("https://www.test.com", url::Url::parse("https://www.test.com").unwrap())]
    fn parse_url_type_success(#[case] value: &str, #[case] expected: url::Url) {
        temp_env::with_var("MY_ENV_VAR", Some(value), || {
            let result: url::Url = get_env_value_with_panic("MY_ENV_VAR");
            assert_eq!(result, expected);
        });
    }

    #[test]
    #[should_panic]
    fn parse_invalid_url_should_panic() {
        temp_env::with_var("MY_ENV_VAR", Some("invalid_url"), || {
            get_env_value_with_panic::<url::Url>("MY_ENV_VAR");
        });
    }
}
