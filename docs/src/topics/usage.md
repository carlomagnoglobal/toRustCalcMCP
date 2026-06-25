COMMAND LINE USAGE

SYNOPSIS

  rcalc [options] [expression...]
  rcalc --mcp

OPTIONS

  -p, --pipe              Pipe mode: read expressions from stdin, one per line
  -q, --quiet             Suppress output (useful for scripts)
  -f, --file FILE         Load and execute a .cal script file
  -m, --mode MODE         Output mode: real, frac, or int (default: real)
                          - real: decimal with ~ for inexact
                          - frac: exact rationals (e.g., 1/2)
                          - int: integer truncation
  -v, --version           Show version and exit
  -h, --help              Show this help and exit
  --mcp                   Start MCP server (JSON-RPC 2.0 over stdio)

INVOCATION MODES

Interactive REPL:
  rcalc
  > 2 + 3
  5
  > quit

Single Expression:
  rcalc '2 + 3'
  5

Pipe Mode:
  echo '2+3' | rcalc -p
  5

Script File:
  rcalc -f script.cal

EXAMPLES

  rcalc '2^256'                    # Big number
  rcalc -m frac '1/3 + 1/6'        # Fractional output: 1/2
  echo -e '2+3\n4*5' | rcalc -p   # Pipe multiple expressions
  rcalc -f math.cal -q             # Run script silently

ENVIRONMENT VARIABLES

  CALCRC                  Path to startup script (auto-loaded in interactive mode)
  HOME                    Used for history file (~/.rcalc_history)

INTERACTIVE MODE

When run without arguments, rcalc enters an interactive REPL with:

  > prompt for input
  Arrow Up/Down           Recall previous commands from history
  Ctrl+R                  Reverse history search
  Ctrl+A/E                Jump to line start/end
  Tab                     Auto-complete (partial support)
  help                    Show all topics and functions
  help <topic>            Show topic documentation
  help <name>             Search for function (e.g., help sin)
  quit, exit              Exit the REPL
  Ctrl+D                  EOF exit

SCRIPT FILES

Scripts are plain text with one statement per line. Comments start with #:

  # This is a comment
  x = 10
  y = 20
  x + y
  define f(n) = n * n
  f(5)

Use -f to load: rcalc -f script.cal

MCP SERVER

Start rcalc as a JSON-RPC 2.0 MCP server for integration with AI tools:

  rcalc --mcp

The server provides 4 tools:
  - calc_eval: evaluate expressions
  - calc_config: get/set session config
  - calc_functions: list builtin functions
  - calc_session: reset or inspect session state

See: help mcp