# lib

## False

```python
False: bool
```

---

## None

```python
None: None
```

---

## True

```python
True: bool
```

---

## abs

```python
def abs(x: float | int, /) -> float | int
```

Take the absolute value of an int.

```
abs(0)   == 0
abs(-10) == 10
abs(10)  == 10
abs(10.0) == 10.0
abs(-12.34) == 12.34
```

---

## all

```python
def all(x: typing.Iterable, /) -> bool
```

[all]( https://github.com/bazelbuild/starlark/blob/master/spec.md#all ): returns true if all values in the iterable object have a truth value of true.

```
all([1, True]) == True
all([1, 1]) == True
all([0, 1, True]) == False
all([True, 1, True]) == True
all([0, 0]) == False
all([0, False]) == False
all([True, 0]) == False
all([1, False]) == False
```

---

## any

```python
def any(x: typing.Iterable, /) -> bool
```

[any]( https://github.com/bazelbuild/starlark/blob/master/spec.md#any ): returns true if any value in the iterable object have a truth value of true.

```
any([0, True]) == True
any([0, 1]) == True
any([0, 1, True]) == True
any([0, 0]) == False
any([0, False]) == False
```

---

## breakpoint

```python
def breakpoint() -> None
```

When a debugger is available, breaks into the debugger.

---

## call\_stack

```python
def call_stack(*, strip_frames: int = 0) -> str
```

Get a textual representation of the call stack.

This is intended only for debugging purposes to display to a human and
should not be considered stable or parseable.

strip_frames will pop N frames from the top of the call stack, which can
be useful to hide non-interesting lines - for example, strip_frames=1
will hide the call to and location of `call_stack()` itself.

---

## call\_stack\_frame

```python
def call_stack_frame(n: int, /) -> None | StackFrame
```

Get a structural representation of the n-th call stack frame.

With `n=0` returns `call_stack_frame` itself.
Returns `None` if `n` is greater than or equal to the stack size.

---

## chr

```python
def chr(i: int, /) -> str
```

[chr]( https://github.com/bazelbuild/starlark/blob/master/spec.md#bool ): returns a string encoding a codepoint.

`chr(i)` returns a string that encodes the single Unicode code
point whose value is specified by the integer `i`. `chr` fails
unless `0 ≤ i ≤ 0x10FFFF`.

```
chr(65) == 'A'
chr(1049) == 'Й'
chr(0x1F63F) == '😿'
```

---

## config\_in

```python
def config_in() -> str
```

---

## config\_out

```python
def config_out() -> str
```

---

## configure\_file

```python
def configure_file(*, input, output, configuration) -> None
```

---

## debug

```python
def debug(val, /) -> str
```

Print the value with full debug formatting. The result may not be stable over time. Intended for debugging purposes and guaranteed to produce verbose output not suitable for user display.

---

## dir

```python
def dir(x, /) -> list[str]
```

[dir]( https://github.com/bazelbuild/starlark/blob/master/spec.md#dir ): list attributes of a value.

`dir(x)` returns a list of the names of the attributes (fields and
methods) of its operand. The attributes of a value `x` are the names
`f` such that `x.f` is a valid expression.

```
"capitalize" in dir("abc")
```

---

## enum

```python
def enum(*args: str)
```

The `enum` type represents one value picked from a set of values.

For example:

```python
MyEnum = enum("option1", "option2", "option3")
```

This statement defines an enumeration `MyEnum` that consists of the three values `"option1"`, `"option2"` and `option3`.

Now `MyEnum` is defined, it's possible to do the following:

* Create values of this type with `MyEnum("option2")`. It is a runtime error if the argument is not one of the predeclared values of the enumeration.
* Get the type of the enum suitable for a type annotation with `MyEnum`.
* Given a value of the enum (for example, `v = MyEnum("option2")`), get the underlying value `v.value == "option2"` or the index in the enumeration `v.index == 1`.
* Get a list of the values that make up the array with `MyEnum.values() == ["option1", "option2", "option3"]`.
* Treat `MyEnum` a bit like an array, with `len(MyEnum) == 3`, `MyEnum[1] == MyEnum("option2")` and iteration over enums `[x.value for x in MyEnum] == ["option1", "option2", "option3"]`.

Enumeration types store each value once, which are then efficiently referenced by enumeration values.

---

## enumerate

```python
def enumerate(it: typing.Iterable, /, start: int = 0) -> list[(int, typing.Any)]
```

[enumerate]( https://github.com/bazelbuild/starlark/blob/master/spec.md#enumerate ): return a list of (index, element) from an iterable.

`enumerate(x)` returns a list of `(index, value)` pairs, each containing
successive values of the iterable sequence and the index of the
value within the sequence.

The optional second parameter, `start`, specifies an integer value to
add to each index.

```
enumerate(["zero", "one", "two"]) == [(0, "zero"), (1, "one"), (2, "two")]
enumerate(["one", "two"], 1) == [(1, "one"), (2, "two")]
```

---

## eval\_type

```python
def eval_type(ty: type, /) -> type
```

Create a runtime type object which can be used to check if a value matches the given type.

---

## fail

```python
def fail(*args) -> typing.Never
```

fail: fail the execution

```
fail("this is an error")  # fail: this is an error
fail("oops", 1, False)  # fail: oops 1 False
```

---

## field

```python
def field(typ, /, default = ...) -> field
```

Creates a field record. Used as an argument to the `record` function.

```
rec_type = record(host=field(str), port=field(int), mask=field(int, default=255))
rec = rec_type(host="localhost", port=80)
rec.port == 80
rec.mask == 255
```

---

## filter

```python
def filter(func: None | typing.Callable, seq: typing.Iterable, /) -> list
```

Apply a predicate to each element of the iterable, returning those that match. As a special case if the function is `None` then removes all the `None` values.

```
filter(bool, [0, 1, False, True]) == [1, True]
filter(lambda x: x > 2, [1, 2, 3, 4]) == [3, 4]
filter(None, [True, None, False]) == [True, False]
```

---

## getattr

```python
def getattr(a, attr: str, default = ..., /)
```

[getattr]( https://github.com/bazelbuild/starlark/blob/master/spec.md#getattr ): returns the value of an attribute

`getattr(x, name)` returns the value of the attribute (field or method)
of x named `name`. It is a dynamic error if x has no such attribute.

`getattr(x, "f")` is equivalent to `x.f`.

`getattr(x, "f", d)` is equivalent to `x.f if hasattr(x, "f") else d`
and will never raise an error.

```
getattr("banana", "split")("a") == ["b", "n", "n", ""] # equivalent to "banana".split("a")
```

---

## hasattr

```python
def hasattr(a, attr: str, /) -> bool
```

[hasattr]( https://github.com/bazelbuild/starlark/blob/master/spec.md#hasattr ): test if an object has an attribute

`hasattr(x, name)` reports whether x has an attribute (field or method)
named `name`.

---

## hash

```python
def hash(a: str, /) -> int
```

[hash]( https://github.com/bazelbuild/starlark/blob/master/spec.md#hash ): returns the hash number of a value.

`hash(x)` returns an integer hash value for x such that `x == y`
implies `hash(x) == hash(y)`.

`hash` fails if x, or any value upon which its hash depends, is
unhashable.

```
hash("hello") != hash("world")
```

---

## isinstance

```python
def isinstance(value, ty: type, /) -> bool
```

Check if a value matches the given type.

This operation can be very fast or very slow depending on how it is used.

`isinstance(x, list)` is very fast,
because it is compiled to a special bytecode instruction.

`isinstance(x, list[str])` is `O(N)` operation
because it checks every element in this list.

`L = list; [isinstance(x, L) for x in y]` is slow when `L` is not a constant:
`isinstance()` first converts `list` to a type in a loop, which is slow.

But last operation can be optimized like this:
`L = eval_type(list); [isinstance(x, L) for x in y]`:
`eval_type()` converts `list` value into prepared type matcher.

---

## len

```python
def len(a, /) -> int
```

[len]( https://github.com/bazelbuild/starlark/blob/master/spec.md#len ): get the length of a sequence

`len(x)` returns the number of elements in its argument.

It is a dynamic error if its argument is not a sequence.

```
len(()) == 0
len({}) == 0
len([]) == 0
len([1]) == 1
len([1,2]) == 2
len({'16': 10}) == 1
len(True)    # error: not supported
```

---

## map

```python
def map(func: typing.Callable, seq: typing.Iterable, /) -> list
```

Apply a function to each element of the iterable, returning the results.

```
map(abs, [7, -5, -6]) == [7, 5, 6]
map(lambda x: x * 2, [1, 2, 3, 4]) == [2, 4, 6, 8]
```

---

## max

```python
def max(*args, key = ...)
```

[max]( https://github.com/bazelbuild/starlark/blob/master/spec.md#max ): returns the maximum of a sequence.

`max(x)` returns the greatest element in the iterable sequence x.

It is an error if any element does not support ordered comparison,
or if the sequence is empty.

The optional named parameter `key` specifies a function to be applied
to each element prior to comparison.

```
max([3, 1, 4, 1, 5, 9])               == 9
max("two", "three", "four")           == "two"    # the lexicographically greatest
max("two", "three", "four", key=len)  == "three"  # the longest
```

---

## min

```python
def min(*args, key = ...)
```

[min]( https://github.com/bazelbuild/starlark/blob/master/spec.md#min ): returns the minimum of a sequence.

`min(x)` returns the least element in the iterable sequence x.

It is an error if any element does not support ordered comparison,
or if the sequence is empty.

```
min([3, 1, 4, 1, 5, 9])                 == 1
min("two", "three", "four")             == "four"  # the lexicographically least
min("two", "three", "four", key=len)    == "two"   # the shortest
```

---

## ord

```python
def ord(a: str, /) -> int
```

[ord]( https://github.com/bazelbuild/starlark/blob/master/spec.md#ord ): returns the codepoint of a character

`ord(s)` returns the integer value of the sole Unicode code point
encoded by the string `s`.

If `s` does not encode exactly one Unicode code point, `ord` fails.
Each invalid code within the string is treated as if it encodes the
Unicode replacement character, U+FFFD.

Example:

```
ord("A")                                == 65
ord("Й")                                == 1049
ord("😿")                               == 0x1F63F
```

---

## partial

```python
def partial(func, /, *args, **kwargs) -> function
```

Construct a partial application. In almost all cases it is simpler to use a `lamdba`.

---

## pprint

```python
def pprint(*args) -> None
```

---

## prepr

```python
def prepr(a, /) -> str
```

Like `repr`, but produces more verbose pretty-printed output

---

## print

```python
def print(*args) -> None
```

Print some values to the output.

---

## pstr

```python
def pstr(a, /) -> str
```

Like `str`, but produces more verbose pretty-printed output

---

## record

```python
def record(**kwargs) -> function
```

A `record` type represents a set of named values, each with their own type.

For example:

```python
MyRecord = record(host=str, port=int)
```

This above statement defines a record `MyRecord` with 2 fields, the first named `host` that must be of type `str`, and the second named `port` that must be of type `int`.

Now `MyRecord` is defined, it's possible to do the following:

* Create values of this type with `MyRecord(host="localhost", port=80)`. It is a runtime error if any arguments are missed, of the wrong type, or if any unexpected arguments are given.
* Get the type of the record suitable for a type annotation with `MyRecord.type`.
* Get the fields of the record. For example, `v = MyRecord(host="localhost", port=80)` will provide `v.host == "localhost"` and `v.port == 80`. Similarly, `dir(v) == ["host", "port"]`.

It is also possible to specify default values for parameters using the `field` function.

For example:

```python
MyRecord = record(host=str, port=field(int, 80))
```

Now the `port` field can be omitted, defaulting to `80` is not present (for example, `MyRecord(host="localhost").port == 80`).

Records are stored deduplicating their field names, making them more memory efficient than dictionaries.

---

## repr

```python
def repr(a, /) -> str
```

[repr]( https://github.com/bazelbuild/starlark/blob/master/spec.md#repr ): formats its argument as a string.

All strings in the result are double-quoted.

```
repr(1)                 == '1'
repr("x")               == "\"x\""
repr([1, "x"])          == "[1, \"x\"]"
repr("test \"'")        == "\"test \\\"'\""
repr("x\"y😿 \\'")      == "\"x\\\"y\\U0001f63f \\\\'\""
```

---

## reversed

```python
def reversed(a: typing.Iterable, /) -> list
```

[reversed]( https://github.com/bazelbuild/starlark/blob/master/spec.md#reversed ): reverse a sequence

`reversed(x)` returns a new list containing the elements of the iterable
sequence x in reverse order.

```
reversed(['a', 'b', 'c'])              == ['c', 'b', 'a']
reversed(range(5))                     == [4, 3, 2, 1, 0]
reversed("stressed".elems())           == ["d", "e", "s", "s", "e", "r", "t", "s"]
reversed({"one": 1, "two": 2}.keys())  == ["two", "one"]
```

---

## sorted

```python
def sorted(x: typing.Iterable, /, *, key = ..., reverse: bool = False) -> list
```

[sorted]( https://github.com/bazelbuild/starlark/blob/master/spec.md#sorted ): sort a sequence

`sorted(x)` returns a new list containing the elements of the iterable
sequence x, in sorted order.  The sort algorithm is stable.

The optional named parameter `reverse`, if true, causes `sorted` to
return results in reverse sorted order.

The optional named parameter `key` specifies a function of one
argument to apply to obtain the value's sort key.
The default behavior is the identity function.

```
sorted([3, 1, 4, 1, 5, 9])                               == [1, 1, 3, 4, 5, 9]
sorted([3, 1, 4, 1, 5, 9], reverse=True)                 == [9, 5, 4, 3, 1, 1]
sorted(["two", "three", "four"], key=len)                == ["two", "four", "three"] # shortest to longest
sorted(["two", "three", "four"], key=len, reverse=True)  == ["three", "four", "two"] # longest to shortest
```

---

## zip

```python
def zip(*args: typing.Iterable) -> list
```

[zip]( https://github.com/bazelbuild/starlark/blob/master/spec.md#zip ): zip several iterables together

`zip()` returns a new list of n-tuples formed from corresponding
elements of each of the n iterable sequences provided as arguments to
`zip`.  That is, the first tuple contains the first element of each of
the sequences, the second element contains the second element of each
of the sequences, and so on.  The result list is only as long as the
shortest of the input sequences.

```
zip()                           == []
zip(range(5))                   == [(0,), (1,), (2,), (3,), (4,)]
zip(range(5), "abc".elems())    == [(0, "a"), (1, "b"), (2, "c")]
```