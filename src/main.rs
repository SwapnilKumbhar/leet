use clap::Parser;
use leetcode::Leetcode;
use log::{error, info, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::process::exit;
use tokio;

mod action;
mod cli;
mod error;
mod lconfig;
mod leetcode;

#[tokio::main]
async fn main() {
    // Parse commandline args
    let args = cli::Args::parse();

    // Configure logging
    let appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h([{l} {M}:{L}])} {m}{n}")))
        .build();
    let log_cfg = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(appender)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
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
        cli::Commands::New {
            template,
            link,
            language,
        } => {
            // Create a client for leetcode
            let leetcode = Leetcode::new();

            let question = match leetcode.get_question_by_link(&link).await {
                Ok(q) => q,
                Err(e) => {
                    error!("Failed to get question from leetcode: {:?}", e);
                    exit(-1);
                }
            };

            let lc_action = action::Action::new(question, &template, &cfg, language).unwrap();

            match lc_action.run() {
                Ok(_) => println!("Created!"),
                Err(e) => {
                    error!("Failed to run action: {:?}", e);

                    // Die or panic, it's all the same.
                    match e {
                        error::ActionError::DirectoryExistsError { dir_name } => {
                            error!("Please delete the directory {} and try again", dir_name);
                        }
                        _ => {
                            error!("Cleaning up!");
                            lc_action.clean_up().unwrap();
                        }
                    }
                }
            }
        }
        cli::Commands::ShowTemplates {} => {
            let templates = cfg.get_templates();
            println!(
                "The current config ({}) supports these languages - ",
                cfg.path
            );
            for template in templates {
                println!("\t- {}", template);
            }
        }
    };
}
