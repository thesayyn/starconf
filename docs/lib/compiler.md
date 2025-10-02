# compiler

## compiler.compiles

```python
def compiler.compiles(
    code: str,
    /,
    *,
    args = ...,
    dependencies = ...,
    include_directories = ...,
    name: str = ...,
) -> bool
```

---

## compiler.get\_supported\_arguments

```python
def compiler.get_supported_arguments(*args) -> list
```

---

## compiler.has\_function

```python
def compiler.has_function(
    funcname,
    /,
    *,
    prefix = ...,
    dependencies = ...,
) -> bool
```

---

## compiler.has\_header

```python
def compiler.has_header(
    header_name,
    /,
    *,
    prefix = ...,
    dependencies = ...,
) -> bool
```

---

## compiler.has\_header\_symbol

```python
def compiler.has_header_symbol(header_name, symbol, /, *, prefix = ...) -> bool
```

---

## compiler.has\_member

```python
def compiler.has_member(type_name, member_name, /, *, prefix = ...) -> bool
```

---

## compiler.has\_type

```python
def compiler.has_type(sym, /, *, prefix = ..., args = ...) -> bool
```

---

## compiler.sizeof

```python
def compiler.sizeof(sym, /, *, prefix = ...) -> int
```