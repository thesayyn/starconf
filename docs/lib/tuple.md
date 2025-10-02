# tuple

```python
def tuple(a: typing.Iterable = ..., /) -> tuple
```

[tuple]( https://github.com/bazelbuild/starlark/blob/master/spec.md#tuple ): returns a tuple containing the elements of the iterable x.

With no arguments, `tuple()` returns the empty tuple.

```
tuple() == ()
tuple([1,2,3]) == (1, 2, 3)
```