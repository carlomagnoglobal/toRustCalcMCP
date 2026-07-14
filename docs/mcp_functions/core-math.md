# Core Math & Arithmetic

305 functions.

| Name | Signature | Description |
|------|-----------|-------------|
| `abs` | `abs(x)` | absolute value |
| `sgn` | `sgn(x)` | sign: -1, 0, or 1 |
| `int` | `int(x)` | integer part |
| `frac` | `frac(x)` | fractional part |
| `floor` | `floor(x)` | largest integer <= x |
| `ceil` | `ceil(x)` | smallest integer >= x |
| `round` | `round(x)` | round to nearest integer |
| `min` | `min(x,...)` | minimum |
| `max` | `max(x,...)` | maximum |
| `avg` | `avg(x,...)` | average |
| `gcd` | `gcd(x,y)` | greatest common divisor |
| `lcm` | `lcm(x,y)` | least common multiple |
| `mod` | `mod(x,y)` | modulus |
| `sqrt` | `sqrt(x)` | square root (returns complex for negative x) |
| `root` | `root(x,n)` | nth root |
| `cbrt` | `cbrt(x)` | cube root |
| `isqrt` | `isqrt(x)` | integer square root |
| `iroot` | `iroot(x,n)` | integer nth root |
| `re` | `re(z)` | real part of complex number |
| `im` | `im(z)` | imaginary part of complex number |
| `arg` | `arg(z)` | argument (phase angle) of complex number |
| `fact` | `fact(n)` | factorial |
| `comb` | `comb(n,k)` | combinations |
| `perm` | `perm(n,k)` | permutations |
| `fib` | `fib(n)` | nth Fibonacci number |
| `isprime` | `isprime(n)` | is n prime? (1 or 0) |
| `nextprime` | `nextprime(n)` | next prime after n |
| `prevprime` | `prevprime(n)` | previous prime before n |
| `config` | `config(name[,val])` | read or set a named config item |
| `display` | `display([n])` | read or set displayed digits |
| `epsilon` | `epsilon([e])` | read or set the session epsilon |
| `places` | `places(x[,base])` | digits after the point, -1 if infinite |
| `base2` | `base2([b])` | secondary display base (not supported) |
| `hash` | `hash(x,...)` | stable hash of values |
| `scan` | `scan()` | read stdin line as whitespace-separated values |
| `scanf` | `scanf(fmt)` | read stdin line per scanf format |
| `rcin` | `rcin(x,m)` | convert into REDC form: x*R mod m |
| `rcout` | `rcout(x,m)` | convert out of REDC form: x/R mod m |
| `rcpow` | `rcpow(x,k,m)` | x^k within the REDC domain |
| `rcsq` | `rcsq(x,m)` | x^2 within the REDC domain |
| `freebernoulli` | `freebernoulli()` | release bernoulli cache (no-op) |
| `freeeuler` | `freeeuler()` | release euler cache (no-op) |
| `freeredc` | `freeredc()` | release REDC cache (no-op) |
| `freestatics` | `freestatics()` | release static caches (no-op) |
| `runtime` | `runtime()` | CPU time used, in seconds |
| `links` | `links(name)` | number of hard links to a file |
| `ltol` | `ltol(a[,eps])` | leg-to-leg: sqrt(1 - a^2) |
| `ferror` | `ferror(fd)` | 1 if descriptor is in an error state |
| `fgetstr` | `fgetstr(fd)` | next line without newline (null at EOF) |
| `fgetfield` | `fgetfield(fd)` | next whitespace-delimited token (null at EOF) |
| `fgetfile` | `fgetfile(fd)` | rest of the file from current position |
| `fgetline` | `fgetline(fd)` | alias for fgetstr |
| `fpathopen` | `fpathopen(name,mode[,path])` | open searching a :-separated path list (default CALCPATH) |
| `freopen` | `freopen(fd,mode[,name])` | reuse a descriptor for a file |
| `files` | `files([fd])` | open descriptors, or the filename of one |
| `isatty` | `isatty(fd)` | 1 if descriptor is a terminal |
| `ungetc` | `ungetc(fd)` | push last-read byte back |
| `cp` | `cp(src,dst)` | copy a file (returns bytes copied) |
| `rm` | `rm(name)` | delete a file (alias for remove) |
| `feof` | `feof(fd)` | 1 at end of file (alias for eof) |
| `ftell` | `ftell(fd)` | current file position (alias for tell) |
| `fputstr` | `fputstr(fd,s)` | write string (alias for fputs) |
| `head` | `head(list,n)` | first n elements |
| `tail` | `tail(list,n)` | last n elements |
| `segment` | `segment(list,start[,end])` | elements start..end inclusive (0-based) |
| `makelist` | `makelist(n)` | list of n nulls |
| `select` | `select(list,f)` | elements where f(element) is nonzero |
| `forall` | `forall(list,f)` | call f on every element |
| `modify` | `modify(list,f)` | list of f(element) |
| `search` | `search(list,value[,start])` | index of first match, or null |
| `rsearch` | `rsearch(list,value[,start])` | index of last match, or null |
| `copy` | `copy(src,dst)` | dst with leading elements from src |
| `cmp` | `cmp(a,b)` | compare values (-1/0/1) |
| `swap` | `swap(a,b)` | swapped pair [b, a] |
| `test` | `test(x)` | 1 if x is true (nonzero/nonempty) |
| `null` | `null()` | the null value |
| `frem` | `frem(x,y)` | x with all factors of y removed |
| `lcmfact` | `lcmfact(n)` | lcm of 1..n |
| `pfact` | `pfact(n)` | product of primes <= n (primorial) |
| `pix` | `pix(n)` | number of primes <= n |
| `mmin` | `mmin(x,m)` | residue of x mod m with minimal absolute value |
| `minv` | `minv(a,m)` | modular inverse of a mod m |
| `meq` | `meq(a,b,m)` | 1 if a == b (mod m) |
| `mne` | `mne(a,b,m)` | 1 if a != b (mod m) |
| `power` | `power(x,y[,eps])` | x^y computed to epsilon |
| `poly` | `poly(a_n,...,a_0,x)` | polynomial evaluation (or poly(list,x)) |
| `polar` | `polar(r,theta[,eps])` | complex from polar coordinates |
| `ssq` | `ssq(x,...)` | sum of squares (lists included) |
| `setbit` | `setbit(x,n[,v])` | set or clear bit n of x |
| `randombit` | `randombit([n])` | integer of n random bits |
| `popcnt` | `popcnt(x)` | count set bits (alias for popcount) |
| `d2dm` | `d2dm(x)` | degrees to [deg, min] (deg mod 360) |
| `d2dms` | `d2dms(x)` | degrees to [deg, min, sec] (deg mod 360) |
| `dm2d` | `dm2d(d,m)` | degrees+minutes to degrees |
| `dms2d` | `dms2d(d,m,s)` | degrees+minutes+seconds to degrees |
| `g2gm` | `g2gm(x)` | gradians to [grad, min] (grad mod 400) |
| `g2gms` | `g2gms(x)` | gradians to [grad, min, sec] (grad mod 400) |
| `gm2g` | `gm2g(g,m)` | gradians+minutes to gradians |
| `gms2g` | `gms2g(g,m,s)` | gradians+minutes+seconds to gradians |
| `h2hm` | `h2hm(x)` | hours to [hour, min] (hour mod 24) |
| `h2hms` | `h2hms(x)` | hours to [hour, min, sec] (hour mod 24) |
| `hm2h` | `hm2h(h,m)` | hours+minutes to hours |
| `hms2h` | `hms2h(h,m,s)` | hours+minutes+seconds to hours |
| `r2g` | `r2g(x)` | radians to gradians |
| `near` | `near(x,y[,eps])` | -1/0/1 as \|x-y\| is less/equal/greater than eps |
| `aversin` | `aversin(x)` | inverse versine: acos(1-x) |
| `avercos` | `avercos(x)` | inverse vercosine: acos(x-1) |
| `acoversin` | `acoversin(x)` | inverse coversine: asin(1-x) |
| `acovercos` | `acovercos(x)` | inverse covercosine: asin(x-1) |
| `ahaversin` | `ahaversin(x)` | inverse haversine: acos(1-2x) |
| `ahavercos` | `ahavercos(x)` | inverse havercosine: acos(2x-1) |
| `ahacoversin` | `ahacoversin(x)` | inverse hacoversine: asin(1-2x) |
| `ahacovercos` | `ahacovercos(x)` | inverse hacovercosine: asin(2x-1) |
| `aexsec` | `aexsec(x)` | inverse exsecant: asec(x+1) |
| `aexcsc` | `aexcsc(x)` | inverse excosecant: acsc(x+1) |
| `acrd` | `acrd(x)` | inverse chord: 2*asin(x/2) |
| `hacovercos` | `hacovercos(x)` | hacovercosine: (1 + sin(x)) / 2 |
| `strcat` | `strcat(s1,s2,...)` | concatenate strings |
| `strcmp` | `strcmp(s1,s2)` | compare strings (-1/0/1) |
| `strcasecmp` | `strcasecmp(s1,s2)` | case-insensitive compare |
| `strncmp` | `strncmp(s1,s2,n)` | compare first n characters |
| `strncasecmp` | `strncasecmp(s1,s2,n)` | case-insensitive compare of first n chars |
| `strcpy` | `strcpy(dst,src)` | copy of src |
| `strncpy` | `strncpy(dst,src,n)` | copy of first n chars of src |
| `strpos` | `strpos(haystack,needle)` | 1-based position of needle, 0 if absent |
| `strerror` | `strerror([code])` | message for an error code |
| `char` | `char(x)` | character for a code, or first char of string |
| `digit` | `digit(x,n[,base])` | digit of x at base^n place |
| `strscan` | `strscan(s,fmt)` | scan values from string per scanf format (returns list) |
| `strscanf` | `strscanf(s,fmt)` | alias for strscan |
| `strtolower` | `strtolower(s)` | lowercase (alias for tolower) |
| `strtoupper` | `strtoupper(s)` | uppercase (alias for toupper) |
| `strprintf` | `strprintf(fmt,...)` | formatted string (alias for sprintf) |
| `iseven` | `iseven(x)` | 1 if x is an even integer |
| `isodd` | `isodd(x)` | 1 if x is an odd integer |
| `isint` | `isint(x)` | 1 if x is an integer |
| `isnum` | `isnum(x)` | 1 if x is a number (real or complex) |
| `isreal` | `isreal(x)` | 1 if x is a real number |
| `isstr` | `isstr(x)` | 1 if x is a string |
| `islist` | `islist(x)` | 1 if x is a list |
| `isnull` | `isnull(x)` | 1 if x is null |
| `isassoc` | `isassoc(x)` | 1 if x is an associative array |
| `ishash` | `ishash(x)` | 1 if x is a hash/associative array |
| `ismat` | `ismat(x)` | 1 if x is a matrix (list of equal-length lists) |
| `isident` | `isident(m)` | 1 if m is an identity matrix |
| `iserror` | `iserror(x)` | 1 if x is an error value (always 0 here) |
| `ismult` | `ismult(x,y)` | 1 if x is an integer multiple of y |
| `isrel` | `isrel(x,y)` | 1 if x and y are relatively prime |
| `issq` | `issq(x)` | 1 if x is a perfect square (rational) |
| `issimple` | `issimple(x)` | 1 if x is a simple value (number/string/null) |
| `istype` | `istype(x,y)` | 1 if x and y have the same type |
| `isfile` | `isfile(x)` | 1 if x is an open file descriptor |
| `isdefined` | `isdefined(name)` | 1 if name is a defined variable |
| `isrand` | `isrand(x)` | 1 if x is a rand state (always 0 here) |
| `israndom` | `israndom(x)` | 1 if x is a random state (always 0 here) |
| `isconfig` | `isconfig(x)` | 1 if x is a config value (always 0 here) |
| `isobj` | `isobj(x)` | 1 if x is an object (always 0 here) |
| `isobjtype` | `isobjtype(x)` | 1 if x is an object type (always 0 here) |
| `isptr` | `isptr(x)` | 1 if x is a pointer (always 0 here) |
| `isblk` | `isblk(x)` | 1 if x is a block value (always 0 here) |
| `isoctet` | `isoctet(x)` | 1 if x is an octet (always 0 here) |
| `nextcand` | `nextcand(n[,count[,skip[,residue[,modulus]]]])` | next probable prime after n (optional residue mod modulus) |
| `prevcand` | `prevcand(n[,count[,skip[,residue[,modulus]]]])` | previous probable prime before n (optional residue mod modulus) |
| `gcdrem` | `gcdrem(x,y)` | remove from x all prime factors shared with y |
| `bround` | `bround(x[,places])` | round x to given number of binary places |
| `btrunc` | `btrunc(x[,places])` | truncate x to given number of binary places |
| `factor` | `factor(n)` | prime factorization (returns list) |
| `lfactor` | `lfactor(n)` | largest prime factor |
| `ptest` | `ptest(n,k)` | probabilistic primality test |
| `euler` | `euler(n)` | Euler number E_n |
| `bernoulli` | `bernoulli(n)` | Bernoulli number B_n |
| `jacobi` | `jacobi(a,n)` | Jacobi symbol (a\|n) |
| `num` | `num(x)` | numerator |
| `den` | `den(x)` | denominator |
| `pi` | `pi()` | π constant (60 digits) |
| `e` | `e()` | e constant (60 digits) |
| `base` | `base([ibase[,obase]])` | get/set input and output base (2-36) |
| `exp` | `exp(x)` | e^x |
| `ln` | `ln(x)` | natural logarithm |
| `log` | `log(x)` | base-10 logarithm |
| `log2` | `log2(x)` | base-2 logarithm |
| `logn` | `logn(x,n)` | logarithm base n |
| `ilog10` | `ilog10(x)` | integer log base 10 |
| `ilog2` | `ilog2(x)` | integer log base 2 |
| `ilog` | `ilog(x)` | integer log base e |
| `ilogn` | `ilogn(x,n)` | integer log base n |
| `sin` | `sin(x)` | sine (radians) |
| `cos` | `cos(x)` | cosine (radians) |
| `tan` | `tan(x)` | tangent (radians) |
| `cot` | `cot(x)` | cotangent (radians) |
| `sec` | `sec(x)` | secant (radians) |
| `csc` | `csc(x)` | cosecant (radians) |
| `asin` | `asin(x)` | inverse sine |
| `acos` | `acos(x)` | inverse cosine |
| `atan` | `atan(x)` | inverse tangent |
| `atan2` | `atan2(y,x)` | two-argument inverse tangent |
| `acot` | `acot(x)` | inverse cotangent |
| `asec` | `asec(x)` | inverse secant |
| `acsc` | `acsc(x)` | inverse cosecant |
| `sinh` | `sinh(x)` | hyperbolic sine |
| `cosh` | `cosh(x)` | hyperbolic cosine |
| `tanh` | `tanh(x)` | hyperbolic tangent |
| `coth` | `coth(x)` | hyperbolic cotangent |
| `sech` | `sech(x)` | hyperbolic secant |
| `csch` | `csch(x)` | hyperbolic cosecant |
| `asinh` | `asinh(x)` | inverse hyperbolic sine |
| `acosh` | `acosh(x)` | inverse hyperbolic cosine |
| `atanh` | `atanh(x)` | inverse hyperbolic tangent |
| `acoth` | `acoth(x)` | inverse hyperbolic cotangent |
| `asech` | `asech(x)` | inverse hyperbolic secant |
| `acsch` | `acsch(x)` | inverse hyperbolic cosecant |
| `cas` | `cas(x)` | cosine + sine |
| `cis` | `cis(x)` | cos(x) + i*sin(x) (returns complex) |
| `conj` | `conj(x)` | complex conjugate |
| `round` | `round(x[,places])` | round to decimal places |
| `hypot` | `hypot(x,y)` | sqrt(x^2 + y^2) |
| `erf` | `erf(x)` | error function |
| `erfc` | `erfc(x)` | complementary error function |
| `gd` | `gd(x)` | Gudermannian function |
| `agd` | `agd(x)` | inverse Gudermannian function |
| `j0` | `j0(x)` | Bessel function J0 |
| `j1` | `j1(x)` | Bessel function J1 |
| `y0` | `y0(x)` | Bessel function Y0 (second kind) |
| `y1` | `y1(x)` | Bessel function Y1 (second kind) |
| `gamma` | `gamma(x)` | gamma function (generalized factorial) |
| `lgamma` | `lgamma(x)` | log-gamma function |
| `polygamma` | `polygamma(n,x)` | polygamma function (nth derivative of log-gamma) |
| `zeta` | `zeta(s)` | Riemann zeta function |
| `rand` | `rand()` | random 32-bit integer |
| `random` | `random()` | random float [0,1) |
| `randbit` | `randbit()` | random bit (0 or 1) |
| `seed` | `seed(s)` | set random seed |
| `srand` | `srand(s)` | set random seed (alias) |
| `srandom` | `srandom(s)` | set random seed (alias) |
| `randint` | `randint(a,b)` | random integer in [a,b] |
| `randperm` | `randperm(n)` | random permutation of 0..n-1 (returns list) |
| `time` | `time()` | current Unix timestamp (seconds since epoch) |
| `systime` | `systime()` | system time (alias for time) |
| `ctime` | `ctime(t)` | convert Unix timestamp to string |
| `sleep` | `sleep(s)` | sleep for s seconds |
| `getenv` | `getenv(name)` | get environment variable |
| `putenv` | `putenv(name,value)` | set environment variable |
| `system` | `system(cmd)` | execute shell command (returns exit code) |
| `usertime` | `usertime()` | user/system time in seconds |
| `isalnum` | `isalnum(s)` | is alphanumeric (1 or 0) |
| `isupper` | `isupper(s)` | is uppercase letter (1 or 0) |
| `islower` | `islower(s)` | is lowercase letter (1 or 0) |
| `isprint` | `isprint(s)` | is printable (1 or 0) |
| `isgraph` | `isgraph(s)` | is visible character (1 or 0) |
| `iscntrl` | `iscntrl(s)` | is control character (1 or 0) |
| `ispunct` | `ispunct(s)` | is punctuation (1 or 0) |
| `isxdigit` | `isxdigit(s)` | is hex digit (1 or 0) |
| `isascii` | `isascii(s)` | is ASCII-only (1 or 0) |
| `toupper` | `toupper(s)` | convert to uppercase |
| `tolower` | `tolower(s)` | convert to lowercase |
| `strrev` | `strrev(s)` | reverse string |
| `pmod` | `pmod(x,y)` | positive modulus (result in [0,y)) |
| `quomod` | `quomod(x,y)` | quotient and modulus (returns [q,r]) |
| `quo` | `quo(x,y)` | quotient (floor(x/y)) |
| `rem` | `rem(x,y)` | remainder (x - y*floor(x/y)) |
| `hnrmod` | `hnrmod(x,y)` | Hensel modular |
| `appr` | `appr(x[,eps])` | rational approximation within epsilon |
| `cfappr` | `cfappr(x[,maxd])` | continued fraction approximation |
| `cfsim` | `cfsim(x[,maxd])` | continued fraction simplification |
| `scale` | `scale(x[,places])` | scale to decimal places |
| `matdim` | `matdim(m)` | matrix dimensions [rows, cols] |
| `mattrans` | `mattrans(m)` | matrix transpose |
| `mattrace` | `mattrace(m)` | matrix trace (sum of diagonal) |
| `det` | `det(m)` | matrix determinant (2x2, 3x3) |
| `inverse` | `inverse(m)` | matrix inverse (2x2) |
| `matsum` | `matsum(m)` | sum of all matrix elements |
| `matmin` | `matmin(m)` | minimum matrix element |
| `matmax` | `matmax(m)` | maximum matrix element |
| `matfill` | `matfill(r,c,v)` | create matrix filled with value |
| `catalan` | `catalan(n)` | Catalan number |
| `and` | `and(x,y)` | bitwise AND |
| `or` | `or(x,y)` | bitwise OR |
| `xor` | `xor(x,y)` | bitwise XOR |
| `comp` | `comp(x)` | bitwise complement |
| `lshift` | `lshift(x,n)` | left shift by n bits |
| `rshift` | `rshift(x,n)` | right shift by n bits |
| `bit` | `bit(x,n)` | is bit n set? (1 or 0) |
| `highbit` | `highbit(x)` | position of highest set bit |
| `lowbit` | `lowbit(x)` | position of lowest set bit |
| `fcnt` | `fcnt(x)` | count of set bits |
| `digits` | `digits(x[,base])` | number of digits (base 10 or specified) |
| `list` | `list(x,...)` | create a list from items |
| `size` | `size(list)` | number of items in list |
| `append` | `append(list,x,...)` | append items to list |
| `first` | `first(list)` | get first item |
| `last` | `last(list)` | get last item |
| `slice` | `slice(list,start[,end])` | get sublist from start to end |
| `strlen` | `strlen(s)` | length of string |
| `index` | `index(haystack,needle)` | find substring position (-1 if not found) |
| `isalpha` | `isalpha(s)` | is string all alphabetic? (1 or 0) |
| `isdigit` | `isdigit(s)` | is string all digits? (1 or 0) |
| `isspace` | `isspace(s)` | is string all whitespace? (1 or 0) |
| `typeof` | `typeof(x)` | get type of value (number, complex, string, list, function, null) |
| `isnan` | `isnan(x)` | is NaN? (always 0 for rationals) |
| `isinf` | `isinf(x)` | is infinite? (always 0 for rationals) |
| `d2r` | `d2r(x)` | degrees to radians |
| `r2d` | `r2d(x)` | radians to degrees |
| `d2g` | `d2g(x)` | degrees to gradians |
| `g2r` | `g2r(x)` | gradians to radians |
| `g2d` | `g2d(x)` | gradians to degrees |
