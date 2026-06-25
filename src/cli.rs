//! Command-line interface for rcalc.

use crate::config::Mode;
use crate::eval::Interp;
use std::io::{self, BufRead, Write};

pub struct CliConfig {
    pub quiet: bool,
    pub pipe: bool,
    pub mode: Mode,
    pub file: Option<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            quiet: false,
            pipe: false,
            mode: Mode::Real,
            file: None,
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<(CliConfig, Vec<String>), String> {
    let mut cfg = CliConfig::default();
    let mut exprs = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-q" => cfg.quiet = true,
            "-p" => cfg.pipe = true,
            "-f" => {
                i += 1;
                if i >= args.len() {
                    return Err("'-f' requires a filename".to_string());
                }
                cfg.file = Some(args[i].clone());
            }
            "-m" => {
                i += 1;
                if i >= args.len() {
                    return Err("'-m' requires an argument".to_string());
                }
                cfg.mode = Mode::parse(&args[i]).ok_or("invalid mode")?;
            }
            "-v" => {
                println!("rcalc (toRustCalcMCP) {}", crate::RCALC_VERSION);
                std::process::exit(0);
            }
            "-h" => {
                print_help();
                std::process::exit(0);
            }
            // Calc compatibility flags (ignored)
            "-c" | "-C" | "-d" | "-e" | "-i" | "-O" | "-s" | "-u" => {}
            s if s.starts_with('-') => {
                return Err(format!("unknown option: {}", s));
            }
            _ => {
                exprs.push(arg.clone());
            }
        }
        i += 1;
    }

    Ok((cfg, exprs))
}

fn print_help() {
    println!("usage: rcalc [options] [expressions...]");
    println!("options:");
    println!("  -p              pipe mode (read from stdin)");
    println!("  -q              quiet (no output)");
    println!("  -f file         read and execute from file");
    println!("  -m real|frac|int  output mode (default: real)");
    println!("  -v              version");
    println!("  -h              help");
}

pub fn run(cfg: CliConfig, exprs: Vec<String>) -> Result<(), String> {
    let mut interp = Interp::new();
    interp.cfg.mode = cfg.mode;

    // Handle file loading (-f flag)
    if let Some(filename) = &cfg.file {
        let contents = std::fs::read_to_string(filename)
            .map_err(|e| format!("cannot read file '{}': {}", filename, e))?;
        match interp.eval_all(&contents) {
            Ok(vals) => {
                if !cfg.quiet {
                    for v in vals {
                        let rendered = v.render(&interp.cfg);
                        if !rendered.is_empty() {
                            println!("{}", rendered);
                        }
                    }
                }
            }
            Err(e) => {
                if !cfg.quiet {
                    eprintln!("error: {}", e);
                }
                return Err(e);
            }
        }
        return Ok(());
    }

    if cfg.pipe {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            match interp.eval_render(trimmed) {
                Ok(out) => {
                    if !cfg.quiet && !out.is_empty() {
                        println!("{}", out);
                    }
                }
                Err(e) => {
                    if !cfg.quiet {
                        eprintln!("error: {}", e);
                    }
                }
            }
        }
        Ok(())
    } else if !exprs.is_empty() {
        for expr in exprs {
            match interp.eval_render(&expr) {
                Ok(out) => {
                    if !cfg.quiet && !out.is_empty() {
                        println!("{}", out);
                    }
                }
                Err(e) => {
                    if !cfg.quiet {
                        eprintln!("error: {}", e);
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    } else {
        repl(&mut interp, cfg.quiet)
    }
}

fn repl(interp: &mut Interp, quiet: bool) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

    loop {
        writer.write_all(b"> ").map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => return Err(e.to_string()),
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        match interp.eval_render(trimmed) {
            Ok(out) => {
                if !quiet && !out.is_empty() {
                    writeln!(writer, "{}", out).map_err(|e| e.to_string())?;
                }
            }
            Err(e) => {
                if !quiet {
                    writeln!(writer, "error: {}", e).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    Ok(())
}
