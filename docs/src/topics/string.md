STRINGS

Text values.

STRING LITERALS

Quoted with double quotes:

  "hello"
  "multi-word string"
  ""                      Empty string

ESCAPE SEQUENCES

  \"                      Double quote
  \\                      Backslash
  \n                      Newline
  \t                      Tab
  \r                      Carriage return

Example:

  "say \"hi\""            = say "hi"
  "line1\nline2"          = (two lines)

BASIC OPERATIONS

  strlen(str)             Length (number of characters)
  substr(str, start[, len]) Extract substring
  index(str, needle)      Find substring (position or -1)
  replace(str, old, new)  Replace all occurrences
  repeat(str, n)          Repeat string n times

Example:

  strlen("hello")         = 5
  substr("hello", 1, 2)   = "el"
  index("hello", "ll")    = 2
  replace("hello", "l", "L") = "heLLo"
  repeat("ab", 3)         = "ababab"

SPLITTING & JOINING

  split(str, sep)         Split by separator (returns list)
  join(list, sep)         Join list items with separator

Example:

  split("a,b,c", ",")     = ["a", "b", "c"]
  join(list("a", "b", "c"), ",") = "a,b,c"

CASE CONVERSION

  toupper(str)            Convert to uppercase
  tolower(str)            Convert to lowercase
  swapcase(str)           Swap case of all characters
  title(str)              Title case (capitalize words)

Example:

  toupper("Hello")        = "HELLO"
  tolower("Hello")        = "hello"
  title("hello world")    = "Hello World"

TRIMMING

  trim(str)               Remove whitespace from both ends
  ltrim(str)              Remove whitespace from left
  rtrim(str)              Remove whitespace from right

Example:

  trim("  hello  ")       = "hello"

PADDING

  lpad(str, width[, fill]) Pad left to width (default fill is space)
  rpad(str, width[, fill]) Pad right to width

Example:

  lpad("5", 3, "0")       = "005"
  rpad("hi", 5, "-")      = "hi---"

CHARACTER OPERATIONS

  ord(char)               Character to ASCII code
  chr(code)               ASCII code to character

Example:

  ord("A")                = 65
  chr(65)                 = "A"

TESTING

  startswith(str, prefix) 1 if starts with prefix
  endswith(str, suffix)   1 if ends with suffix

Example:

  startswith("hello", "he") = 1
  endswith("hello", "lo")   = 1

CHARACTER CLASS TESTS

  isalpha(str)            All alphabetic? (1 or 0)
  isdigit(str)            All digits? (1 or 0)
  isalnum(str)            All alphanumeric? (1 or 0)
  isspace(str)            All whitespace? (1 or 0)
  isupper(str)            All uppercase? (1 or 0)
  islower(str)            All lowercase? (1 or 0)
  isprint(str)            All printable? (1 or 0)

Example:

  isalpha("hello")        = 1
  isdigit("123")          = 1
  isdigit("12a")          = 0

PARSING & CONVERSION

  str(value)              Convert value to string

Example:

  str(123)                = "123"
  str(1/3)                = "1/3"
  str(list(1, 2))         = "[1, 2]"