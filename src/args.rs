use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author)]
pub struct CursalArgs {
    pub url: String,
    #[arg(short, long)]
    pub method: Option<String>,
    #[arg(short, long)]
    pub data: Option<String>,
    #[arg(short, long)]
    pub output: Option<String>,
}
