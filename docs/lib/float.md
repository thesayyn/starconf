# float

```python
def float(a: bool | float | int | str = ..., /) -> float
```

[float]( https://github.com/bazelbuild/starlark/blob/master/spec.md#float ): interprets its argument as a floating-point number.

If x is a `float`, the result is x.
if x is an `int`, the result is the nearest floating point value to x.
If x is a string, the string is interpreted as a floating-point literal.
With no arguments, `float()` returns `0.0`.

```
float() == 0.0
float(1) == 1.0
float('1') == 1.0
float('1.0') == 1.0
float('.25') == 0.25
float('1e2') == 100.0
float(False) == 0.0
float(True) == 1.0
float("hello")   # error: not a valid number
float([])   # error
```