use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum ConfigError {
    #[fail(display = "Invalid YAML file: {}", err)]
    InvalidYaml { err: yaml_rust::scanner::ScanError },

    #[fail(display = "Unknown root element `{}`", name)]
    UnkownRootElement { name: String },

    #[fail(display = "{}", message)]
    GeneralError { message: String },

    #[fail(display = "Unknown checker attribute `{}` in checkers.{}", attr_name, checker_id)]
    UnknownCheckerAttribute { checker_id: String, attr_name: String },

    #[fail(display = "Failed to parse interval `{}` in checkers.{}.interval", interval, checker_id)]
    InvalidCheckerInterval { checker_id: String, interval: String },

    #[fail(display = "Failed to parse URL `{}` in checkers.{}.url", url, checker_id)]
    InvalidCheckerUrl { checker_id: String, url: String },

    #[fail(display = "Invalid notifier type `{}` in `notifiers.{}.type`", type_value, notifier_id)]
    InvalidNotifierType { notifier_id: String, type_value: String },

    #[fail(display = "Unknown {} notifier attribute `{}` in checkers.{}", notifier_type, attr_name, notifier_id)]
    UnknownNotifierAttribute { notifier_id: String, notifier_type: String, attr_name: String },

    #[fail(display = "Field `{}` is missing", path)]
    FieldMissing { path: String },
}

#[derive(Debug, Fail, PartialEq)]
pub enum ConfigValidationError {
    #[fail(display = "Invalid YAML file: {}", err)]
    InvalidYaml { err: yaml_rust::scanner::ScanError },

    #[fail(display = "`checkers.{}.notifiers` refers to an undeclared notifier `{}`", checker_id, notifier_id)]
    UnknownNotifier { checker_id: String, notifier_id: String }
}
