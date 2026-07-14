# Residue Class & Modular Operations

8 functions.

| Name | Signature | Description |
|------|-----------|-------------|
| `rc` | `rc(n,m)` | residue class: reduce n modulo m |
| `rcadd` | `rcadd(a,b,m)` | residue addition: (a+b) mod m |
| `rcsub` | `rcsub(a,b,m)` | residue subtraction: (a-b) mod m |
| `rcmul` | `rcmul(a,b,m)` | residue multiplication: (a*b) mod m |
| `rcdiv` | `rcdiv(a,b,m)` | residue division: (a/b) mod m |
| `rcinv` | `rcinv(a,m)` | modular inverse of a mod m |
| `rceq` | `rceq(a,b,m)` | residue equality: check if a≡b (mod m) |
| `rcneg` | `rcneg(a,m)` | residue negation: (-a) mod m |
