VARIABLES

A variable stores a value and persists across statements in a session.

DECLARING & ASSIGNING

Simply assign a value:

  x = 10
  y = x + 5

Variables are created on first assignment.

SCOPE

All variables are global within a session. Inside a function, parameters and
local assignments shadow globals but do not modify them:

  x = 100
  define test(x) {
      x = 200
      return x
  }
  test(50)            = 200
  x                   = 100  (unchanged)

LISTING VARIABLES

Use vars() to list all global variables:

  x = 1
  y = 2
  z = 3
  vars()              = [x, y, z]

CHECKING EXISTENCE

Use defined(name) to check if a variable exists:

  defined("x")        = 1  (exists)
  defined("unknown")  = 0  (does not exist)

DELETING VARIABLES

Use undefine(name) or del(name) to remove a variable:

  undefine("x")
  del("y")

After deletion, vars() no longer includes them.

GETTING SIZE

Use sizeof(x) to get the approximate size in bytes:

  x = list(1, 2, 3)
  sizeof(x)           (size in bytes)

TYPE CHECKING

Use typeof(x) to get the type of a value:

  typeof(5)           = "number"
  typeof("hello")     = "string"
  typeof(list(1,2))   = "list"

SPECIAL VARIABLES

Some functions act like variables (read/query):

  vars()              List all global variables
  env()               List environment variables as pairs
  epsilon             Current epsilon (precision target)
  pi()                Constant π (60 digits)
  e()                 Constant e (60 digits)