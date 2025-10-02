# str

```python
def str(a, /) -> str
```

[str]( https://github.com/bazelbuild/starlark/blob/master/spec.md#str ): formats its argument as a string.

If x is a string, the result is x (without quotation).
All other strings, such as elements of a list of strings, are
double-quoted.

```
str(1)                          == '1'
str("x")                        == 'x'
str([1, "x"])                   == "[1, \"x\"]"
```

---

## str.capitalize

```python
def str.capitalize() -> str
```

[string.capitalize]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string%C2%B7capitalize ): returns a copy of string S, where the first character (if any) is converted to uppercase; all other characters are converted to lowercase.

```
"hello, world!".capitalize() == "Hello, world!"
"Hello, World!".capitalize() == "Hello, world!"
"".capitalize() == ""
```

---

## str.codepoints

```python
def str.codepoints() -> typing.Iterable[str]
```

[string.codepoints]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·codepoints ): returns an iterable of the unicode codepoint of a string.

`S.codepoints()` returns an iterable value containing the
sequence of integer Unicode code points encoded by the string S.
Each invalid code within the string is treated as if it encodes the
Unicode replacement character, U+FFFD.

By returning an iterable, not a list, the cost of decoding the string
is deferred until actually needed; apply `list(...)` to the result to
materialize the entire sequence.

```
list("Hello, 世界".codepoints()) == [72, 101, 108, 108, 111, 44, 32, 19990, 30028]
```

---

## str.count

```python
def str.count(
    needle: str,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[string.count]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·count ): count the number of occurrences of a string in another string.

`S.count(sub[, start[, end]])` returns the number of occurrences of
`sub` within the string S, or, if the optional substring indices
`start` and `end` are provided, within the designated substring of S.
They are interpreted according to Skylark's [indexing conventions](
https://github.com/bazelbuild/starlark/blob/master/spec.md#indexing).

This implementation does not count occurrence of `sub` in the string `S`
that overlap other occurrence of S (which can happen if some suffix of S
is a prefix of S). For instance, `"abababa".count("aba")` returns 2
for `[aba]a[aba]`, not counting the middle occurrence: `ab[aba]ba`
(this is following Python behavior).

```
"hello, world!".count("o") == 2
"abababa".count("aba") == 2
"hello, world!".count("o", 7, 12) == 1  # in "world"
```

---

## str.elems

```python
def str.elems() -> typing.Iterable[str]
```

[string.elems]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·elems ): returns an iterable of the bytes values of a string.

`S.elems()` returns an iterable value containing the
sequence of numeric bytes values in the string S.

To materialize the entire sequence of bytes, apply `list(...)` to the
result.

```
list("Hello, 世界".elems()) == ["H", "e", "l", "l", "o", ",", " ", "世", "界"]
```

---

## str.endswith

```python
def str.endswith(suffix: str | tuple[str, ...], /) -> bool
```

[string.endswith]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·endswith ): determine if a string ends with a given suffix.

`S.endswith(suffix)` reports whether the string S has the specified
suffix.

```
"filename.sky".endswith(".sky") == True
```

---

## str.find

```python
def str.find(
    needle: str,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[string.find]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·find ): find a substring in a string.

`S.find(sub[, start[, end]])` returns the index of the first
occurrence of the substring `sub` within S.

If either or both of `start` or `end` are specified,
they specify a subrange of S to which the search should be restricted.
They are interpreted according to Skylark's [indexing
conventions](#indexing).

If no occurrence is found, `found` returns -1.

```
"bonbon".find("on") == 1
"bonbon".find("on", 2) == 4
"bonbon".find("on", 2, 5) == -1
```

---

## str.format

```python
def str.format(*args, **kwargs) -> str
```

[string.format]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·format ): format a string.

`S.format(*args, **kwargs)` returns a version of the format string S
in which bracketed portions `{...}` are replaced
by arguments from `args` and `kwargs`.

Within the format string, a pair of braces `{{` or `}}` is treated as
a literal open or close brace.
Each unpaired open brace must be matched by a close brace `}`.
The optional text between corresponding open and close braces
specifies which argument to use and how to format it, and consists of
three components, all optional:
a field name, a conversion preceded by '`!`', and a format specifier
preceded by '`:`'.

```text
{field}
{field:spec}
{field!conv}
{field!conv:spec}
```

The *field name* may be either a decimal number or a keyword.
A number is interpreted as the index of a positional argument;
a keyword specifies the value of a keyword argument.
If all the numeric field names form the sequence 0, 1, 2, and so on,
they may be omitted and those values will be implied; however,
the explicit and implicit forms may not be mixed.

The *conversion* specifies how to convert an argument value `x` to a
string. It may be either `!r`, which converts the value using
`repr(x)`, or `!s`, which converts the value using `str(x)` and is
the default.

The *format specifier*, after a colon, specifies field width,
alignment, padding, and numeric precision.
Currently it must be empty, but it is reserved for future use.

```rust
"a {} c".format(3) == "a 3 c"
"a{x}b{y}c{}".format(1, x=2, y=3) == "a2b3c1"
"a{}b{}c".format(1, 2) == "a1b2c"
"({1}, {0})".format("zero", "one") == "(one, zero)"
"Is {0!r} {0!s}?".format("heterological") == "Is \"heterological\" heterological?"
```

---

## str.index

```python
def str.index(
    needle: str,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[string.index]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·index ): search a substring inside a string, failing on not found.

`S.index(sub[, start[, end]])` returns the index of the first
occurrence of the substring `sub` within S, like `S.find`, except
that if the substring is not found, the operation fails.

```
"bonbon".index("on") == 1
"bonbon".index("on", 2) == 4
"bonbon".index("on", 2, 5)    # error: not found
```

---

## str.isalnum

```python
def str.isalnum() -> bool
```

[string.isalnum]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·isalnum ): test if a string is composed only of letters and digits.

`S.isalnum()` reports whether the string S is non-empty and consists
only Unicode letters and digits.

```
"base64".isalnum() == True
"Catch-22".isalnum() == False
```

---

## str.isalpha

```python
def str.isalpha() -> bool
```

[string.isalpha]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·isalpha ): test if a string is composed only of letters.

`S.isalpha()` reports whether the string S is non-empty and consists
only of Unicode letters.

```
"ABC".isalpha() == True
"Catch-22".isalpha() == False
"".isalpha() == False
```

---

## str.isdigit

```python
def str.isdigit() -> bool
```

[string.isdigit]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·isdigit ): test if a string is composed only of digits.

`S.isdigit()` reports whether the string S is non-empty and consists
only of Unicode digits.

```
"123".isdigit() == True
"Catch-22".isdigit() == False
"".isdigit() == False
```

---

## str.islower

```python
def str.islower() -> bool
```

[string.islower]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·islower ): test if all letters of a string are lowercase.

`S.islower()` reports whether the string S contains at least one cased
Unicode letter, and all such letters are lowercase.

```
"hello, world".islower() == True
"Catch-22".islower() == False
"123".islower() == False
```

---

## str.isspace

```python
def str.isspace() -> bool
```

[string.isspace]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·isspace ): test if all characters of a string are whitespaces.

`S.isspace()` reports whether the string S is non-empty and consists
only of Unicode spaces.

```
"    ".isspace() == True
"\r\t\n".isspace() == True
"".isspace() == False
```

---

## str.istitle

```python
def str.istitle() -> bool
```

[string.istitle]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·istitle ): test if the string is title cased.

`S.istitle()` reports whether the string S contains at least one cased
Unicode letter, and all such letters that begin a word are in title
case.

```
"Hello, World!".istitle() == True
"Catch-22".istitle() == True
"HAL-9000".istitle() == False
"123".istitle() == False
```

---

## str.isupper

```python
def str.isupper() -> bool
```

[string.isupper]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·isupper ): test if all letters of a string are uppercase.

`S.isupper()` reports whether the string S contains at least one cased
Unicode letter, and all such letters are uppercase.

```
"HAL-9000".isupper() == True
"Catch-22".isupper() == False
"123".isupper() == False
```

---

## str.join

```python
def str.join(to_join: typing.Iterable[str], /) -> str
```

[string.join]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·join ): join elements with a separator.

`S.join(iterable)` returns the string formed by concatenating each
element of its argument, with a copy of the string S between
successive elements. The argument must be an iterable whose elements
are strings.

```
", ".join([]) == ""
", ".join(("x", )) == "x"
", ".join(["one", "two", "three"]) == "one, two, three"
"a".join("ctmrn".elems()) == "catamaran"
```

---

## str.lower

```python
def str.lower() -> str
```

[string.lower]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·lower ): convert a string to all lowercase.

`S.lower()` returns a copy of the string S with letters converted to
lowercase.

```
"Hello, World!".lower() == "hello, world!"
```

---

## str.lstrip

```python
def str.lstrip(chars: str = ..., /) -> str
```

[string.lstrip]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·lstrip ): trim leading whitespaces.

`S.lstrip()` returns a copy of the string S with leading whitespace removed.
In most cases instead of passing an argument you should use `removeprefix`.

```
"  hello  ".lstrip() == "hello  "
"x!hello  ".lstrip("!x ") == "hello  "
```

---

## str.partition

```python
def str.partition(needle: str, /) -> (str, str, str)
```

[string.partition]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·partition ): partition a string in 3 components

`S.partition(x = " ")` splits string S into three parts and returns them
as a tuple: the portion before the first occurrence of string `x`,
`x` itself, and the portion following it.
If S does not contain `x`, `partition` returns `(S, "", "")`.

`partition` fails if `x` is not a string, or is the empty string.

```
"one/two/three".partition("/") == ("one", "/", "two/three")
"one".partition("/") == ("one", "", "")
```

---

## str.removeprefix

```python
def str.removeprefix(prefix: str, /) -> str
```

[string.removeprefix]( https://docs.python.org/3.9/library/stdtypes.html#str.removeprefix ): remove a prefix from a string. _Not part of standard Starlark._

If the string starts with the prefix string, return `string[len(prefix):]`.
Otherwise, return a copy of the original string:

```
"Hello, World!".removeprefix("Hello") == ", World!"
"Hello, World!".removeprefix("Goodbye") == "Hello, World!"
"Hello".removeprefix("Hello") == ""
```

---

## str.removesuffix

```python
def str.removesuffix(suffix: str, /) -> str
```

[string.removesuffix]( https://docs.python.org/3.9/library/stdtypes.html#str.removesuffix ): remove a prefix from a string. _Not part of standard Starlark._

If the string starts with the prefix string, return `string[len(prefix):]`.
Otherwise, return a copy of the original string:

```
"Hello, World!".removesuffix("World!") == "Hello, "
"Hello, World!".removesuffix("World") == "Hello, World!"
"Hello".removesuffix("Hello") == ""
```

---

## str.replace

```python
def str.replace(old: str, new: str, count: int = ..., /) -> str
```

[string.replace]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·replace ): replace all occurrences of a substring.

`S.replace(old, new[, count])` returns a copy of string S with all
occurrences of substring `old` replaced by `new`. If the optional
argument `count`, which must be an `int`, is non-negative, it
specifies a maximum number of occurrences to replace.

```
"banana".replace("a", "o") == "bonono"
"banana".replace("a", "o", 2) == "bonona"
"banana".replace("z", "x") == "banana"
"banana".replace("", "x") == "xbxaxnxaxnxax"
"banana".replace("", "x", 2) == "xbxanana"
"".replace("", "x") == "x"
"banana".replace("a", "o", -2)  # error: argument was negative
```

---

## str.rfind

```python
def str.rfind(
    needle: str,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[string.rfind]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·rfind ): find the last index of a substring.

`S.rfind(sub[, start[, end]])` returns the index of the substring `sub`
within S, like `S.find`, except that `rfind` returns the index of
the substring's _last_ occurrence.

```
"bonbon".rfind("on") == 4
"bonbon".rfind("on", None, 5) == 1
"bonbon".rfind("on", 2, 5) == -1
```

---

## str.rindex

```python
def str.rindex(
    needle: str,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[string.rindex]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·rindex ): find the last index of a substring, failing on not found.

`S.rindex(sub[, start[, end]])` returns the index of the substring `sub`
within S, like `S.index`, except that `rindex` returns the index of
the substring's _last_ occurrence.

```
"bonbon".rindex("on") == 4
"bonbon".rindex("on", None, 5) == 1  # in "bonbo"
"bonbon".rindex("on", 2, 5) #   error: not found
```

---

## str.rpartition

```python
def str.rpartition(needle: str, /) -> (str, str, str)
```

[string.rpartition]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·rpartition ): partition a string in 3 elements.

`S.rpartition([x = ' '])` is like `partition`, but splits `S` at the
last occurrence of `x`.

```
"one/two/three".rpartition("/") == ("one/two", "/", "three")
"one".rpartition("/") == ("", "", "one")
```

---

## str.rsplit

```python
def str.rsplit(
    sep: None | str = None,
    maxsplit: None | int = None,
    /,
) -> list[str]
```

[string.rsplit]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·rsplit ): splits a string into substrings.

`S.rsplit([sep[, maxsplit]])` splits a string into substrings like
`S.split`, except that when a maximum number of splits is specified,
`rsplit` chooses the rightmost splits.

```
"banana".rsplit("n") == ["ba", "a", "a"]
"banana".rsplit("n", 1) == ["bana", "a"]
"one two  three".rsplit(None, 1) == ["one two", "three"]
```

---

## str.rstrip

```python
def str.rstrip(chars: str = ..., /) -> str
```

[string.rstrip]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·rstrip ): trim trailing whitespace.

`S.rstrip()` returns a copy of the string S with trailing whitespace removed.
In most cases instead of passing an argument you should use `removesuffix`.

```
"  hello  ".rstrip() == "  hello"
"  hello!x".rstrip(" x!") == "  hello"
```

---

## str.split

```python
def str.split(
    sep: None | str = None,
    maxsplit: None | int = None,
    /,
) -> list[str]
```

[string.split]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·split ): split a string in substrings.

`S.split([sep [, maxsplit]])` returns the list of substrings of S,
splitting at occurrences of the delimiter string `sep`.

Consecutive occurrences of `sep` are considered to delimit empty
strings, so `'food'.split('o')` returns `['f', '', 'd']`.
Splitting an empty string with a specified separator returns `['']`.
If `sep` is the empty string, `split` fails.

If `sep` is not specified or is `None`, `split` uses a different
algorithm: it removes all leading spaces from S
(or trailing spaces in the case of `rsplit`),
then splits the string around each consecutive non-empty sequence of
Unicode white space characters.

If S consists only of white space, `split` returns the empty list.

If `maxsplit` is given and non-negative, it specifies a maximum number
of splits.

```
"one two  three".split() == ["one", "two", "three"]
"one two  three".split(" ") == ["one", "two", "", "three"]
"one two  three".split(None, 1) == ["one", "two  three"]
"banana".split("n") == ["ba", "a", "a"]
"banana".split("n", 1) == ["ba", "ana"]
```

---

## str.splitlines

```python
def str.splitlines(keepends: bool = False, /) -> list[str]
```

[string.splitlines]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·splitlines ): return the list of lines of a string.

`S.splitlines([keepends])` returns a list whose elements are the
successive lines of S, that is, the strings formed by splitting S at
line terminators ('\n', '\r' or '\r\n').

The optional argument, `keepends`, is interpreted as a Boolean.
If true, line terminators are preserved in the result, though
the final element does not necessarily end with a line terminator.

```
"one\n\ntwo".splitlines() == ["one", "", "two"]
"one\n\ntwo".splitlines(True) == ["one\n", "\n", "two"]
"a\nb".splitlines() == ["a", "b"]
```

---

## str.startswith

```python
def str.startswith(prefix: str | tuple[str, ...], /) -> bool
```

[string.startswith]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·startswith ): test whether a string starts with a given prefix.

`S.startswith(suffix)` reports whether the string S has the specified
prefix.

```
"filename.sky".startswith("filename") == True
"filename.sky".startswith("sky") == False
'abc'.startswith(('a', 'A')) == True
'ABC'.startswith(('a', 'A')) == True
'def'.startswith(('a', 'A')) == False
```

---

## str.strip

```python
def str.strip(chars: str = ..., /) -> str
```

[string.strip]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·strip ): trim leading and trailing whitespaces.

`S.strip()` returns a copy of the string S with leading and trailing
whitespace removed.

```
"  hello  ".strip() == "hello"
"xxhello!!".strip("x!") == "hello"
```

---

## str.title

```python
def str.title() -> str
```

[string.title]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·title ): convert a string to title case.

`S.title()` returns a copy of the string S with letters converted to
titlecase.

Letters are converted to uppercase at the start of words, lowercase
elsewhere.

```
"hElLo, WoRlD!".title() == "Hello, World!"
```

---

## str.upper

```python
def str.upper() -> str
```

[string.upper]( https://github.com/bazelbuild/starlark/blob/master/spec.md#string·upper ): convert a string to all uppercase.

`S.upper()` returns a copy of the string S with letters converted to
uppercase.

```
"Hello, World!".upper() == "HELLO, WORLD!"
```