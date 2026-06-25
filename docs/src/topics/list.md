LISTS

Ordered collections of values.

CREATING LISTS

Use list() function:

  list(1, 2, 3)           = [1, 2, 3]
  list("a", "b", "c")     = ["a", "b", "c"]
  list(1, "two", 3.5)     = [1, "two", 7/2]

Create a range:

  range(1, 5)             = [1, 2, 3, 4, 5]
  range(0, 10, 2)         = [0, 2, 4, 6, 8, 10]

Empty list:

  list()                  = []

INDEXING

Access elements with [n]:

  x = list(10, 20, 30)
  x[0]                    = 10  (first, 0-indexed)
  x[1]                    = 20
  x[2]                    = 30
  x[-1]                   = 30  (last)
  x[-2]                   = 20  (second from last)

Out of bounds returns null.

BASIC OPERATIONS

  size(list)              Length
  first(list)             First element
  last(list)              Last element
  append(list, item, ...) Add items (mutates list)
  slice(list, start[, end]) Sublist from start to end

Example:

  x = list(1, 2, 3)
  append(x, 4, 5)         # x is now [1, 2, 3, 4, 5]
  slice(x, 1, 3)          = [2, 3]

SORTING & REVERSING

  sort(list)              Sort ascending
  rsort(list)             Sort descending
  reverse(list)           Reverse order
  unique(list)            Remove duplicates

Example:

  sort(list(3, 1, 2))     = [1, 2, 3]
  reverse(list(1, 2, 3))  = [3, 2, 1]
  unique(list(1, 1, 2))   = [1, 2]

SEARCHING

  find(list, value)       Index of first match (-1 if not found)
  contains(list, value)   1 if found, 0 otherwise
  count(list, value)      Number of occurrences

Example:

  find(list(10, 20, 30), 20)    = 1
  contains(list(1, 2, 3), 2)    = 1

AGGREGATION

  sum(list)               Sum of elements
  product(list)           Product of elements
  min(list)               Minimum value
  max(list)               Maximum value
  mean(list)              Arithmetic mean
  median(list)            Middle value
  variance(list)          Variance
  stdev(list)             Standard deviation
  rms(list)               Root mean square
  gmean(list)             Geometric mean
  hmean(list)             Harmonic mean
  mode(list)              Most common value

Example:

  sum(list(1, 2, 3, 4))   = 10
  mean(list(1, 2, 3, 4))  = 2.5
  median(list(1, 3, 5))   = 3

COMBINING LISTS

  zip(list1, list2)       Pair corresponding elements
  flatten(list)           Flatten nested lists
  union(list1, list2)     Set union
  intersection(list1, list2) Set intersection
  difference(list1, list2)   Set difference (list1 - list2)
  subset(list1, list2)    1 if list1 ⊆ list2

Example:

  zip(list(1, 2), list("a", "b")) = [[1, "a"], [2, "b"]]
  flatten(list(1, list(2, 3)))    = [1, 2, 3]

TRANSFORMATION

  cumsum(list)            Cumulative sum
  diff(list)              Consecutive differences

Example:

  cumsum(list(1, 2, 3))   = [1, 3, 6]
  diff(list(1, 3, 6))     = [2, 3]

ITERATION

In a script, use for() to iterate:

  for (x = list(1, 2, 3)) {
      print x
  }