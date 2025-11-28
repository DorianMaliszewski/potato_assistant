use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Potato Assistant CLI", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub debug: bool,
}
