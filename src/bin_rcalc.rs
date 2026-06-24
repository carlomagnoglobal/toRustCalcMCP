use torustcalcmcp::cli;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let (cfg, exprs) = cli::parse_args(&args[1..])?;
    cli::run(cfg, exprs)
}
