# range

```python
def range(a1: int, a2: int = ..., step: int = 1, /) -> range
```

[range]( https://github.com/bazelbuild/starlark/blob/master/spec.md#range ): return a range of integers

`range` returns a tuple of integers defined by the specified interval
and stride.

```python
range(stop)                             # equivalent to range(0, stop)
range(start, stop)                      # equivalent to range(start, stop, 1)
range(start, stop, step)
```

`range` requires between one and three integer arguments.
With one argument, `range(stop)` returns the ascending sequence of
non-negative integers less than `stop`.
With two arguments, `range(start, stop)` returns only integers not less
than `start`.

With three arguments, `range(start, stop, step)` returns integers
formed by successively adding `step` to `start` until the value meets or
passes `stop`. A call to `range` fails if the value of `step` is
zero.

```
list(range(10))                         == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
list(range(3, 10))                      == [3, 4, 5, 6, 7, 8, 9]
list(range(3, 10, 2))                   == [3, 5, 7, 9]
list(range(10, 3, -2))                  == [10, 8, 6, 4]
```