//! Command-line interface for rcalc.

use crate::config::Mode;
use crate::eval::Interp;
use rustyline::DefaultEditor;
use std::io::{self, BufRead};

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
    let mut rl = DefaultEditor::new().map_err(|e| format!("failed to create editor: {}", e))?;
    let history_file = dirs_home().map(|home| format!("{}/.rcalc_history", home));

    if let Some(ref hf) = history_file {
        let _ = rl.load_history(hf);
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(trimmed);

                if trimmed == "exit" || trimmed == "quit" {
                    break;
                }

                if trimmed == "help" {
                    print_function_help();
                    continue;
                }

                match interp.eval_render(trimmed) {
                    Ok(out) => {
                        if !quiet && !out.is_empty() {
                            println!("{}", out);
                        }
                    }
                    Err(e) => {
                        if !quiet {
                            eprintln!("error: {}", e);
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                return Err(format!("readline error: {}", e));
            }
        }
    }

    if let Some(hf) = history_file {
        let _ = rl.save_history(&hf);
    }

    Ok(())
}

fn dirs_home() -> Option<String> {
    if let Ok(home) = std::env::var("HOME") {
        return Some(home);
    }
    #[cfg(windows)]
    if let Ok(home) = std::env::var("USERPROFILE") {
        return Some(home);
    }
    None
}

fn print_function_help() {
    println!("\n📚 Available Functions (351 total)\n");
    println!("{:<20} {:<40} {}", "Function", "Signature", "Description");
    println!("{}", "─".repeat(100));

    let mut count = 0;
    for (name, sig, desc) in crate::builtins::catalog() {
        println!("{:<20} {:<40} {}", name, sig, desc);
        count += 1;
        if count % 20 == 0 {
            println!("  ... (showing {} of 351 functions, use grep or search above)", count);
        }
    }
    println!("\n💡 Usage examples:");
    println!("  > 2^100                          # Big number (exact)");
    println!("  > sin(pi()/6)                    # Trigonometric functions");
    println!("  > list(1,2,3); sort(list(3,1,2)) # List operations");
    println!("  > substr(\"hello\", 1, 3)         # String functions");
    println!("  > hex(255); oct(64); bin(15)    # Base conversion");
    println!("  > mean(list(1,2,3,4,5))         # Statistics");
    println!("\n🔍 Search for a function: grep the output or use Ctrl+R for history search\n");
}
