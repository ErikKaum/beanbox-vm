use clap::{Parser, Subcommand};

mod bindings;
mod utils;
mod vm;
mod welcome;

#[derive(Parser)]
#[command(name = "Beanbox")]
#[command(about = "A microVM for you and your LLM", long_about = None)]
#[command(version)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        #[arg(default_value_t = 2)]
        cpus: u32,

        #[arg(short, long)]
        #[arg(default_value_t = 512)]
        mem: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    let port = utils::find_available_port(3002, 4000).unwrap();

    match &cli.command {
        Some(Commands::Run { cpus, mem }) => vm::run(*cpus, *mem, port),
        None => {}
    }
}
