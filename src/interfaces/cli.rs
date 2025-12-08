use clap::{Parser, Subcommand};

use crate::{
    application::{Service, run_analysis},
    interfaces::{shared::Present, supervisor::SupervisorController},
};

/// Define the CLI arguments
#[derive(Parser)]
#[command(name = "patseeker", about = "CLI for Patseeker system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    RunAnalysis {
        #[arg(short, long)]
        symbol: String,
        #[arg(short, long)]
        timeframe: String,
        #[arg(short, long, default_value_t = 100)]
        lookback: usize,
    },
}

pub struct CliRunner<'a, S, P> {
    supervisor_controller: SupervisorController<'a, S, P>,
}

impl<'a, S, P> CliRunner<'a, S, P>
where
    S: Service<run_analysis::Request, run_analysis::Result>,
    P: Present<run_analysis::Result>,
{
    pub fn new(supervisor_controller: SupervisorController<'a, S, P>) -> Self {
        Self {
            supervisor_controller,
        }
    }

    pub async fn run_once(&self) {
        let cli = Cli::parse();
        self.exec_command(cli.command).await;
    }

    pub async fn run_loop(&self) {
        loop {
            println!("Enter command (or type 'exit' to quit):");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "exit" {
                break;
            }

            let args = std::env::args()
                .take(1)
                .chain(input.split_whitespace().map(|s| s.to_string()));
            let cli = Cli::parse_from(args);

            self.exec_command(cli.command).await;
        }
    }

    async fn exec_command(&self, command: Commands) {
        match command {
            Commands::RunAnalysis {
                symbol,
                timeframe,
                lookback,
            } => {
                let res = self
                    .supervisor_controller
                    .run_analysis(symbol, timeframe, lookback)
                    .await;
                println!("{}", res);
            }
        }
    }
}
