MATRICES

Matrices are represented as lists of lists (rows).

CREATING MATRICES

  matfill(rows, cols, val) Create matrix filled with val
  [[1, 2], [3, 4]]         Literal 2x2 matrix

Example:

  m = matfill(2, 3, 0)    = [[0, 0, 0], [0, 0, 0]]

QUERYING MATRICES

  matdim(m)               Dimensions [rows, cols]
  matmin(m)               Minimum element
  matmax(m)               Maximum element
  matsum(m)               Sum of all elements

Example:

  matdim([[1, 2], [3, 4]]) = [2, 2]

OPERATIONS

  det(m)                  Determinant (2x2, 3x3)
  inverse(m)              Matrix inverse
  mattrans(m)             Transpose (swap rows/cols)
  mattrace(m)             Trace (sum of diagonal)
  matmul(m1, m2)          Matrix multiplication

Example:

  det([[1, 2], [3, 4]])           = -2
  mattrans([[1, 2], [3, 4]])      = [[1, 3], [2, 4]]
  mattrace([[1, 0], [0, 2]])      = 3

VECTORS

Vectors are 1-row or 1-column lists:

  v = [1, 2, 3]
  dot(v, [4, 5, 6])               = 1*4 + 2*5 + 3*6 = 32
  norm(v)                          = sqrt(1^2 + 2^2 + 3^2) = sqrt(14)

Example:

  dot([1, 2, 3], [4, 5, 6])        = 32
  norm([3, 4])                     = 5