SCRIPTS & RESOURCE FILES

Load and execute files.

LOADING A SCRIPT

  rcalc -f script.cal

The file is read and executed line by line. Variables and functions persist.

SCRIPT SYNTAX

Plain text, one statement per line:

  # Comment (starts with #)
  x = 10
  y = 20
  print x + y

Semicolons separate statements on one line:

  x = 5; y = 10; print x + y

MULTI-LINE STATEMENTS

Use blocks for control flow:

  if (x > 10) {
      print "large"
  } else {
      print "small"
  }

  define fib(n) {
      if (n <= 1) return n
      return fib(n-1) + fib(n-2)
  }

COMMENTS

Lines starting with # are comments:

  # This is a comment
  x = 5  # Also a comment after code

LOADING WITHIN A SESSION

Use read(filename) or source(filename) to load a file (if implemented):

  read("lib.cal")

STARTUP SCRIPT

Environment variable CALCRC points to an optional startup script, auto-loaded
on interactive startup:

  export CALCRC=~/.calcrc
  rcalc                         Loads ~/.calcrc first

COMMON PATTERNS

Defining a function library:

  # lib.cal
  define double(x) = x * 2
  define triple(x) = x * 3

Load and use:

  rcalc -f lib.cal
  > double(5)
  10