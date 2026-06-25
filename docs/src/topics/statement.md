STATEMENTS

rcalc supports several statement types.

ASSIGNMENT

Assign a value to a variable:

  x = 5
  y = x + 10

Variables persist across statements.

PRINT

Print a value to output:

  print x
  print "Hello"
  println "Done"

(println adds a newline)

CONDITIONAL: IF/ELSE

  if (x > 10) {
      print "x is large"
  } else {
      print "x is small"
  }

The else branch is optional.

LOOP: WHILE

Repeat while a condition is true:

  i = 0
  while (i < 5) {
      print i
      i = i + 1
  }

LOOP: FOR

Loop from start to end (inclusive):

  for (i = 1; 10) {
      print i
  }

This prints 1 through 10.

RETURN

Exit a function with a value:

  define f(x) {
      if (x < 0) return 0
      return x * 2
  }

QUIT/EXIT

Exit the REPL:

  quit
  exit

(Only in interactive mode; in scripts, quits the script.)

BLOCK STATEMENTS

Group multiple statements with braces:

  {
      x = 5
      y = 10
      print x + y
  }

SEMICOLON AS SEPARATOR

Statements are separated by semicolons or newlines:

  x = 5; y = 10; print x + y

Multiple statements on one line are allowed.