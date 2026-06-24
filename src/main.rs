use std::env;
use torustcalcmcp::cli;
use torustcalcmcp::mcp;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let prog = args.get(0).map(|s| s.as_str()).unwrap_or("calc");
    let is_rcalc = prog.ends_with("rcalc");

    let args = &args[1..];

    // Check for --mcp flag
    if args.contains(&"--mcp".to_string()) || args.contains(&"mcp".to_string()) {
        mcp::serve_stdio().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // If argv[0] ends with "rcalc", force CLI mode
    if is_rcalc {
        let (cfg, exprs) = cli::parse_args(args)?;
        cli::run(cfg, exprs)?;
        return Ok(());
    }

    // Default: CLI mode
    let (cfg, exprs) = cli::parse_args(args)?;
    cli::run(cfg, exprs)?;
    Ok(())
}
