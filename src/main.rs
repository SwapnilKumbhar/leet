use clap::Parser;
use log::{error, info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::process::exit;

mod action;
mod cli;
mod error;
mod lconfig;

fn main() {
    // Parse commandline args
    let args = cli::Args::parse();

    // Get log level.
    let log_level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };

    // Configure logging
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h([{l}])} {m}{n}")))
        .build();
    let log_cfg = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(appender)))
        .build(Root::builder().appender("stdout").build(log_level))
    {
        Ok(cfg) => cfg,
        Err(e) => {
            // maybe we can do this better?
            println!("Failed to configure logger: {}.", e);
            println!("Exiting...");
            exit(-1)
        }
    };

    match log4rs::init_config(log_cfg) {
        Ok(_) => (),
        Err(_) => {
            // maybe we can do this better?
            println!("Failed to configure logger. Exiting...");
            exit(-1)
        }
    };

    info!(
        "Loading config from {}.",
        args.config.clone().unwrap_or("default paths".to_string())
    );

    // Check and parse the config
    let cfg = match lconfig::parse_config(args.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("{}", e);
            exit(-1)
        }
    };

    // Run the given command
    match args.command {
        cli::Commands::New { language, name } => {
            let template = match cfg.get_template(&language) {
                Some(t) => t,
                None => {
                    error!(
                        "Could not find language `{}` in the current config. Exiting...",
                        language
                    );
                    exit(-1)
                }
            };
            match action::doit(&language, template, cfg.get_tdir_path(), name) {
                Ok(_) => println!("Done!"),
                Err(e) => {
                    error!("Fatal: {}", e)
                }
            };
        }
        cli::Commands::ShowLanguages {} => {
            let languages = cfg.get_languages();
            println!(
                "The current config ({}) supports these languages - ",
                cfg.path
            );
            for language in languages {
                println!("\t- {}", language)
            }
        }
    };
}
