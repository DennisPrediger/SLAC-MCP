The Syntax is similar to Delphi Pascal/ Free Pascal.

# Operators

```
ARITHMETIC
  +       Addition, string concat, array concat
  -       Subtraction, unary negation
  *       Multiplication
  /       Float division (result is always f64)
  div     Integer division (truncated toward zero)
  mod     Modulo (floating-point remainder)

COMPARISON
  =       Equal
  <>      Not equal (NOT !=)
  >       Greater than
  >=      Greater than or equal
  <       Less than
  <=      Less than or equal

LOGICAL
  and     Logical AND (short-circuit)
  or      Logical OR (short-circuit)
  xor     Logical XOR
  not     Logical NOT (unary)

GROUPING
  ( )     Parentheses override precedence
  [ ]     Array literal

FUNCTION CALLS
  name(arg, arg, ...)    Function invocation

PRECEDENCE (highest to lowest):
  Unary (not, -) -> Factor (*, /, div, mod) -> Term (+, -)
  -> Comparison (=, <>, <, >, <=, >=) -> Xor -> And -> Or
```

# Functions: Common (23 functions)

```
TYPE CONVERSION
  bool(value)               -> Boolean    Convert to bool (0/''/[] = false)
  float(value)              -> Number     Convert to float
  int(value)                -> Number     Convert to integer (truncated)
  str(value)                -> String     Convert to string

COLLECTIONS & STRINGS
  length(value)             -> Number     Length of string or array
  empty(value)              -> Boolean    True if 0, '', false, or []
  at(values, index)         -> Any        Element at index (strings: 1-based, arrays: 0-based)
  contains(haystack, needle)-> Boolean    Substring or element membership
  count(haystack, needle)   -> Number     Count non-overlapping occurrences
  find(haystack, needle)    -> Number     Index of first occurrence (-1 if not found)
  copy(source, start, count)-> String/Array  Extract sub-range
  insert(target, source, idx)-> String/Array Insert at position
  replace(value, from, to)  -> String/Array  Replace all matches (2 args = remove)
  reverse(value)            -> String/Array  Reverse elements/characters

AGGREGATES
  max(...)                  -> Any        Maximum of values (variadic or Array)
  min(...)                  -> Any        Minimum of values (variadic or Array)
  sort(values)              -> Array      Sorted copy of array
  unique(values)            -> Array      Deduplicated (order preserved)

BOOLEAN LOGIC
  all(...)                  -> Boolean    True if all values are true (variadic or Array)
  any(...)                  -> Boolean    True if any value is true (variadic or Array)

COMPARISON & RANGE
  between(val, lower, upper)-> Boolean    Inclusive range check: val >= lower && val <= upper
  compare(left, right)      -> Number     -1 (less), 0 (equal), 1 (greater)

CONDITIONAL
  if_then(cond, then, else?)-> Any        Ternary (optimizer converts to short-circuit)
                                        2-arg form: else defaults to type's empty value
```

# Functions: Math

```
BASIC
  abs(value)                -> Number     Absolute value
  pow(value, exp=2)         -> Number     Power (default: square)
  round(value)              -> Number     Round to nearest integer
  trunc(value)              -> Number     Truncate toward zero
  frac(value)               -> Number     Fractional part

TRIGONOMETRIC
  sin(value)                -> Number     Sine (radians)
  cos(value)                -> Number     Cosine (radians)
  arc_tan(value)            -> Number     Arctangent

LOGARITHMIC & EXPONENTIAL
  ln(value)                 -> Number     Natural logarithm
  exp(value)                -> Number     e^value

INTEGER CHECKS
  even(value)               -> Boolean    True if floor(value) is even
  odd(value)                -> Boolean    True if floor(value) is odd
  int_to_hex(value)         -> String     Hex string (uppercase, e.g. "DEADBEEF")

RANDOM (impure)
  random(range=1)           -> Number     Random in [0, range)
  choice(...)               -> Any        Random element (variadic or Array)
```

# Functions: Strings

```
CASE
  lowercase(text)           -> String     To lowercase (unicode-aware)
  uppercase(text)           -> String     To uppercase (unicode-aware)
  same_text(left, right)    -> Boolean    Case-insensitive comparison

CHARACTER OPS
  ord(char)                 -> Number     ASCII ordinal (0-127)
  chr(ord)                  -> String     Character from ordinal (0-127)

TRIMMING
  trim(text)                -> String     Trim whitespace both sides
  trim_left(text)           -> String     Trim left whitespace
  trim_right(text)          -> String     Trim right whitespace

SPLITTING
  split(line, separator)    -> Array      Split by separator (plain)
  split_csv(line, sep=';')  -> Array      Quote-aware CSV split
```

# Functions: Time

```
DATETIME REPRESENTATION
  Datetimes are stored as Number: integral = days since epoch,
  fractional = time of day as fraction of 24h (e.g. 0.5 = noon).
  All operations use the system's local timezone.

CONSTRUCTION
  encode_date(year, month, day)                        -> Number
  encode_time(hour, minute, second, ms=0)              -> Number

PARSING (string -> Number)
  string_to_date(str, fmt='%Y-%m-%d')                  -> Number
  string_to_time(str, fmt='%H:%M:%S')                  -> Number
  string_to_datetime(str, fmt='%Y-%m-%d %H:%M:%S')    -> Number
  date_from_rfc2822(str)                               -> Number
  date_from_rfc3339(str)                               -> Number

FORMATTING (Number -> string)
  date_to_string(fmt, datetime)                        -> String
  time_to_string(fmt, datetime)                        -> String
  date_to_rfc2822(datetime)                            -> String
  date_to_rfc3339(datetime)                            -> String

EXTRACTION
  date(datetime)          -> Number   Date portion (trunc)
  time(datetime)          -> Number   Time portion (frac)
  year(datetime)          -> Number
  month(datetime)         -> Number   (1-12)
  day(datetime)           -> Number   (1-31)
  hour(datetime)          -> Number   (0-23)
  minute(datetime)        -> Number   (0-59)
  second(datetime)        -> Number   (0-59)
  millisecond(datetime)   -> Number   (0-999)
  day_of_week(datetime)   -> Number   (0=Monday, 6=Sunday)

ARITHMETIC & QUERIES
  inc_month(dt, n=1)      -> Number   Add/subtract months
  is_leap_year(datetime)  -> Boolean
```

# Functions: Regex

```
re_is_match(haystack, pattern)              -> Boolean   Any match exists
re_find(haystack, pattern)                  -> Array     All non-overlapping matches
re_capture(haystack, pattern)               -> Array     First match + capture groups (0-indexed = full match)
re_replace(hay, pat, repl='', limit=0)      -> String    Replace matches (supports $1, $2 backrefs; 0=unlimited)
re_split(haystack, pattern)                 -> Array     Split on pattern matches
```

# Examples

```
ARITHMETIC
  40 + 1 * 2                                  -> 42
  50 div 20 mod 2                             -> 2
  (40 + 1) * 2                                -> 82
  pow(2, 10)                                  -> 1024
  abs(-42)                                    -> 42

STRINGS
  'Hello' + ' ' + 'World'                     -> Hello World
  'It''s here'                                -> It's here
  length('hello')                             -> 5
  lowercase('HELLO WORLD')                    -> hello world
  at('abcde', 3)                              -> c
  find('hello world', 'world')                -> 7
  replace('hello', 'l', 'r')                  -> herro
  split_csv('"a;b";c', ';')                   -> ['a;b', 'c']
  trim('  hello  ')                           -> hello

BOOLEANS
  True and not False                          -> true
  10 > 5                                      -> true
  between(5, 1, 10)                           -> true
  empty('')                                   -> true
  empty('x')                                  -> false

ARRAYS
  [1, 2, 3] + [4, 5]                          -> [1, 2, 3, 4, 5]
  sort([3, 1, 2])                             -> [1, 2, 3]
  unique([1, 1, 2, 3, 3])                     -> [1, 2, 3]
  at([10, 20, 30], 1)                         -> 20
  reverse([1, 2, 3])                          -> [3, 2, 1]

CONDITIONAL
  if_then(10 > 5, 'yes', 'no')                -> yes
  if_then(False, 1, 2)                        -> 2

MATH
  sin(0)                                      -> 0
  sqrt(144)                                   -> 12
  round(3.7)                                  -> 4
  even(4)                                     -> true
  int_to_hex(255)                             -> FF

TIME
  year(string_to_date('2024-12-25'))          -> 2024
  month(string_to_date('2024-12-25'))         -> 12
  day(string_to_date('2024-12-25'))           -> 25
  date_to_string('%Y-%m-%d', encode_date(2024, 7, 4))  -> 2024-07-04
  inc_month(string_to_date('2024-01-31'), 1)  -> 19787 (2024-02-29, leap year)

REGEX
  re_is_match('abc123', '\d+')                -> true
  re_find('abc123def456', '\d+')              -> ['123', '456']
  re_capture('2024-07-04', '(\d{4})-(\d{2})-(\d{2})')  -> ['2024-07-04', '2024', '07', '04']
  re_replace('hello world', 'world', 'there') -> hello there
  re_split('a1b2c', '\d+')                    -> ['a', 'b', 'c']

CHAINING
  length(lowercase('Hello World'))            -> 11
  at(split('a,b,c', ','), 1)                  -> b
  str(max(10, 20, 30))                        -> 30
  int(float('3.14') * 10)                     -> 31
  contains(str(pow(2, 10)), '1024')           -> true
  sort(unique([3,1,2,1,3]))                   -> [1, 2, 3]
```
