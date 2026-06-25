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

                if trimmed.starts_with("help") {
                    let filter = if trimmed == "help" {
                        None
                    } else if trimmed.starts_with("help ") {
                        Some(trimmed[5..].trim())
                    } else {
                        None
                    };
                    print_function_help(filter);
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

fn print_function_help(filter: Option<&str>) {
    // No argument: show topic list and full catalog
    let Some(f) = filter else {
        println!("\nHelp topics: {}", crate::help::TOPICS.join("  "));
        println!("\nUsage:");
        println!("  help <topic>  — show topic documentation (e.g. help intro)");
        println!("  help <name>   — search functions by name (e.g. help sin)");
        println!("\nAvailable functions (351 total):\n");
        println!("{:<20} {:<40} {}", "Function", "Signature", "Description");
        println!("{}", "─".repeat(100));
        for (name, sig, desc) in crate::builtins::catalog() {
            println!("{:<20} {:<40} {}", name, sig, desc);
        }
        println!();
        return;
    };

    // Check if it's a known topic first
    if let Some(text) = crate::help::topic(f) {
        println!("{}", text);
        return;
    }

    // Not a topic, search functions
    let filter_lower = f.to_ascii_lowercase();
    let mut matches = Vec::new();
    for (name, sig, desc) in crate::builtins::catalog() {
        if name.contains(filter_lower.as_str()) || sig.contains(filter_lower.as_str()) || desc.to_ascii_lowercase().contains(filter_lower.as_str()) {
            matches.push((name, sig, desc));
        }
    }

    if matches.is_empty() {
        println!("\n❌ No topic or functions found matching '{}'", f);
        println!("\n💡 Try one of these topics:");
        println!("  help intro    — introduction and basic usage");
        println!("  help usage    — command-line options");
        println!("  help builtin  — about builtin functions");
        println!("  help define   — user-defined functions");
        println!("  help operator — operator table");
        println!("  help list     — list operations");
        println!("  help string   — string operations");
        println!("\nOr search functions: help sin, help list, help string, etc.\n");
        return;
    }

    let title = format!("📚 Functions matching '{}' ({} found)", f, matches.len());
    println!("\n{}\n", title);
    println!("{:<20} {:<40} {}", "Function", "Signature", "Description");
    println!("{}", "─".repeat(100));

    for (name, sig, desc) in &matches {
        println!("{:<20} {:<40} {}", name, sig, desc);
    }
    println!();
}
