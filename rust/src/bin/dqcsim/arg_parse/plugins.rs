use dqcsim::{
    common::{log::tee_file::TeeFileConfiguration, log::*},
    host::configuration::*,
};

/// Structure containing the NONfunctional options for a plugin, i.e. the
/// options that only affect how the plugin represents its output.
///
/// This differs from `PluginProcessNonfunctionalConfiguration` in that
/// unspecified values have not yet been replaced with their defaults. This
/// allows the structure to be built incrementally (see `apply()`).
#[derive(Debug, PartialEq)]
pub struct PluginNonfunctionalOpts {
    /// Specifies the verbosity of the messages sent to DQCsim. If this is
    /// `None`, the value of DQCsim's `--plugin_level` option should be used.
    pub verbosity: Option<LoglevelFilter>,

    /// Specifies the tee files for this plugin.
    pub tee_files: Vec<TeeFileConfiguration>,

    /// Specifies how the stdout stream of the plugin should be connected.
    /// `None` implies default.
    pub stdout_mode: Option<StreamCaptureMode>,

    /// Specifies how the stderr stream of the plugin should be connected.
    /// `None` implies default.
    pub stderr_mode: Option<StreamCaptureMode>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub accept_timeout: Option<Timeout>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub shutdown_timeout: Option<Timeout>,
}

impl PluginNonfunctionalOpts {
    /// Converts this structure to a `PluginProcessNonfunctionalConfiguration`
    /// structure by replacing unset values with their defaults.
    pub fn into_config(
        self,
        default_verbosity: LoglevelFilter,
    ) -> PluginProcessNonfunctionalConfiguration {
        PluginProcessNonfunctionalConfiguration {
            verbosity: self.verbosity.unwrap_or(default_verbosity),
            tee_files: self.tee_files,
            stdout_mode: self
                .stdout_mode
                .unwrap_or(StreamCaptureMode::Capture(Loglevel::Info)),
            stderr_mode: self
                .stderr_mode
                .unwrap_or(StreamCaptureMode::Capture(Loglevel::Info)),
            accept_timeout: self
                .accept_timeout
                .unwrap_or_else(|| Timeout::from_seconds(5)),
            shutdown_timeout: self
                .shutdown_timeout
                .unwrap_or_else(|| Timeout::from_seconds(5)),
        }
    }
}

impl Default for PluginNonfunctionalOpts {
    fn default() -> Self {
        PluginNonfunctionalOpts {
            verbosity: None,
            tee_files: vec![],
            stdout_mode: None,
            stderr_mode: None,
            accept_timeout: None,
            shutdown_timeout: None,
        }
    }
}

/// Represents the definition of a plugin.
///
/// In combination with some modifiers and defaults set by DQCsim itself, this
/// contains everything needed to construct a plugin.
#[derive(Debug)]
pub struct PluginDefinition {
    /// Name of the plugin, used to refer to the plugin by the log system and
    /// by `PluginModification`s.
    pub name: String,

    /// Plugin specification, i.e. the plugin executable and optionally the
    /// script it should execute.
    pub specification: PluginProcessSpecification,

    /// The functional configuration of the plugin, i.e. the options
    /// configuring how the plugin behaves (besides the specification).
    pub functional: PluginProcessFunctionalConfiguration,

    /// The nonfunctional configuration of the plugin, i.e. any options that
    /// do not affect how the plugin behaves functionally, but only affect its
    /// output representation.
    pub nonfunctional: PluginNonfunctionalOpts,
}

impl PluginDefinition {
    /// Converts this structure to a PluginProcessConfiguration structure by
    /// replacing unset values with their defaults.
    pub fn into_config(self, default_verbosity: LoglevelFilter) -> PluginProcessConfiguration {
        PluginProcessConfiguration {
            name: self.name,
            specification: self.specification,
            functional: self.functional,
            nonfunctional: self.nonfunctional.into_config(default_verbosity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dqcsim::{
        common::types::PluginType, host::configuration::PluginProcessNonfunctionalConfiguration,
    };

    #[test]
    fn non_func_opt() {
        let pnfo = PluginNonfunctionalOpts::default();
        assert_eq!(
            pnfo,
            PluginNonfunctionalOpts {
                verbosity: None,
                tee_files: vec![],
                stdout_mode: None,
                stderr_mode: None,
                accept_timeout: None,
                shutdown_timeout: None,
            },
        );
    }

    #[test]
    fn non_func_opt_debug() {
        let pnfo = PluginNonfunctionalOpts::default();
        assert_eq!(
            format!("{:?}", pnfo),
            "PluginNonfunctionalOpts { verbosity: None, tee_files: [], stdout_mode: None, stderr_mode: None, accept_timeout: None, shutdown_timeout: None }",
        );
    }

    #[test]
    fn into_process_conf() {
        let p = PluginNonfunctionalOpts::default();
        let c: PluginProcessNonfunctionalConfiguration = p.into_config(LoglevelFilter::Debug);
        assert_eq!(
            c,
            PluginProcessNonfunctionalConfiguration {
                verbosity: LoglevelFilter::Debug,
                tee_files: vec![],
                stdout_mode: StreamCaptureMode::Capture(Loglevel::Info),
                stderr_mode: StreamCaptureMode::Capture(Loglevel::Info),
                accept_timeout: Timeout::from_seconds(5),
                shutdown_timeout: Timeout::from_seconds(5),
            }
        );

        let p = PluginNonfunctionalOpts {
            verbosity: Some(LoglevelFilter::Fatal),
            tee_files: vec![TeeFileConfiguration::new(
                LoglevelFilter::Error,
                "/dev/null",
            )],
            stdout_mode: Some(StreamCaptureMode::Null),
            stderr_mode: Some(StreamCaptureMode::Pass),
            accept_timeout: Some(Timeout::Infinite),
            shutdown_timeout: Some(Timeout::from_seconds(1)),
        };
        let c: PluginProcessNonfunctionalConfiguration = p.into_config(LoglevelFilter::Debug);
        assert_eq!(
            c,
            PluginProcessNonfunctionalConfiguration {
                verbosity: LoglevelFilter::Fatal,
                tee_files: vec![TeeFileConfiguration::new(
                    LoglevelFilter::Error,
                    "/dev/null",
                )],
                stdout_mode: StreamCaptureMode::Null,
                stderr_mode: StreamCaptureMode::Pass,
                accept_timeout: Timeout::Infinite,
                shutdown_timeout: Timeout::from_seconds(1),
            }
        );
    }

    #[test]
    fn into_def_conf() {
        let p = PluginDefinition {
            name: "name".to_string(),
            specification: PluginProcessSpecification::from_sugar(
                "/bin/echo",
                PluginType::Operator,
            )
            .unwrap(),
            functional: PluginProcessFunctionalConfiguration::default(),
            nonfunctional: PluginNonfunctionalOpts::default(),
        };
        let c: PluginProcessConfiguration = p.into_config(LoglevelFilter::Trace);
        assert_eq!(
            c,
            PluginProcessConfiguration {
                name: "name".to_string(),
                specification: PluginProcessSpecification::from_sugar(
                    "/bin/echo",
                    PluginType::Operator
                )
                .unwrap(),
                functional: PluginProcessFunctionalConfiguration::default(),
                nonfunctional: PluginProcessNonfunctionalConfiguration::default(),
            }
        );
    }

    #[test]
    fn debug() {
        let p = PluginDefinition {
            name: "name".to_string(),
            specification: PluginProcessSpecification::from_sugar(
                "/bin/echo",
                PluginType::Operator,
            )
            .unwrap(),
            functional: PluginProcessFunctionalConfiguration::default(),
            nonfunctional: PluginNonfunctionalOpts::default(),
        };
        assert_eq!(format!("{:?}", p), "PluginDefinition { name: \"name\", specification: PluginProcessSpecification { executable: \"/bin/echo\", script: None, typ: Operator }, functional: PluginProcessFunctionalConfiguration { init: [], env: [], work: \".\" }, nonfunctional: PluginNonfunctionalOpts { verbosity: None, tee_files: [], stdout_mode: None, stderr_mode: None, accept_timeout: None, shutdown_timeout: None } }");
    }
}
