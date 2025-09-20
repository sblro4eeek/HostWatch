use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    long_about = None
)]
pub struct Cli {
    #[arg(
        short = 'P' , 
        long = "port",
        default_value_t = 7878
    )]
    pub port: u16,
}
