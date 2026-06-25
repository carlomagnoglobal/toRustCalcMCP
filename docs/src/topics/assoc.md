ASSOCIATIVE ARRAYS (HASHES)

Key-value mappings.

CREATING HASHES

Use assoc() with alternating keys and values:

  h = assoc("name", "Alice", "age", 30)

Retrieve values:

  h["name"]               = "Alice"
  h["age"]                = 30

OPERATIONS

  indices(h)              List all keys
  count(h)                Number of key-value pairs
  insert(h, key, value)   Add or update key (mutates h)
  delete(h, key)          Remove key (mutates h)
  join(h, sep)            Join values with separator

Example:

  h = assoc("x", 1, "y", 2)
  indices(h)              = ["x", "y"]
  insert(h, "z", 3)       # h now has 3 key-value pairs
  delete(h, "y")          # h now has 2 key-value pairs
  count(h)                = 2
  join(h, ", ")           = "1, 3"