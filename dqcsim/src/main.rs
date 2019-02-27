use dqcsim_core::plugin;
use dqcsim_log::LogThread;
use log::{debug, info, LevelFilter};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Set logging verbosity to <loglevel>
    /// [OFF, ERROR, WARN, INFO, DEBUG, TRACE]
    #[structopt(short = "l", long = "loglevel")]
    loglevel: Option<LevelFilter>,

    /// Plugin configurations.
    #[structopt(raw(required = "true", min_values = "1"))]
    plugins: Vec<plugin::config::PluginConfig>,
}

fn main() -> Result<(), ()> {
    // Parse arguments
    let opt = Opt::from_args();

    // Setup logging
    dqcsim_log::init(opt.loglevel).expect("Failed to initialize logger.");
    let logger = LogThread::new(opt.loglevel);

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Debug message with parsed Opt struct
    debug!("Parsed arguments: {:?}", &opt);

    // Create plugins from PluginConfigs
    let plugins: Vec<plugin::Plugin> = opt
        .plugins
        .into_iter()
        .map(|config| plugin::Plugin::new(config, &logger))
        .collect();
    for plugin in &plugins {
        plugin.init().expect("init failed");
    }

    Ok(())
}
