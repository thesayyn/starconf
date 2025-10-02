# type

```python
def type(a, /) -> str
```

[type]( https://github.com/bazelbuild/starlark/blob/master/spec.md#type ): returns a string describing the type of its operand.

```
type(None)              == "NoneType"
type(0)                 == "int"
type(1)                 == "int"
type(())                == "tuple"
type("hello")           == "string"
```