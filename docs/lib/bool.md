# bool

```python
def bool(x = ..., /) -> bool
```

[bool]( https://github.com/bazelbuild/starlark/blob/master/spec.md#bool ): returns the truth value of any starlark value.

```
bool() == False
bool([]) == False
bool([1]) == True
bool(True) == True
bool(False) == False
bool(None) == False
bool(bool) == True
bool(1) == True
bool(0) == False
bool({}) == False
bool({1:2}) == True
bool(()) == False
bool((1,)) == True
bool("") == False
bool("1") == True
```