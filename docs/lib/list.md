# list

```python
def list(a: typing.Iterable = ..., /) -> list
```

[list]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list ): construct a list.

`list(x)` returns a new list containing the elements of the
iterable sequence x.

With no argument, `list()` returns a new empty list.

```
list()        == []
list((1,2,3)) == [1, 2, 3]
list("strings are not iterable") # error: not supported
```

---

## list.append

```python
def list.append(el, /) -> None
```

[list.append]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·append ): append an element to a list.

`L.append(x)` appends `x` to the list L, and returns `None`.

`append` fails if the list is frozen or has active iterators.

```
x = []
x.append(1)
x.append(2)
x.append(3)
x == [1, 2, 3]
```

---

## list.clear

```python
def list.clear() -> None
```

[list.clear]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·clear ): clear a list

`L.clear()` removes all the elements of the list L and returns `None`.
It fails if the list is frozen or if there are active iterators.

```
x = [1, 2, 3]
x.clear()
x == []
```

---

## list.extend

```python
def list.extend(other: typing.Iterable, /) -> None
```

[list.extend]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·extend ): extend a list with another iterable's content.

`L.extend(x)` appends the elements of `x`, which must be iterable, to
the list L, and returns `None`.

`extend` fails if `x` is not iterable, or if the list L is frozen or has
active iterators.

```
x = []
x.extend([1, 2, 3])
x.extend(["foo"])
x == [1, 2, 3, "foo"]
```

---

## list.index

```python
def list.index(
    needle,
    start: None | int = None,
    end: None | int = None,
    /,
) -> int
```

[list.index]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·index ): get the index of an element in the list.

`L.index(x[, start[, end]])` finds `x` within the list L and returns its
index.

The optional `start` and `end` parameters restrict the portion of
list L that is inspected.  If provided and not `None`, they must be list
indices of type `int`. If an index is negative, `len(L)` is effectively
added to it, then if the index is outside the range `[0:len(L)]`, the
nearest value within that range is used; see [Indexing](#indexing).

`index` fails if `x` is not found in L, or if `start` or `end`
is not a valid index (`int` or `None`).

```
x = ["b", "a", "n", "a", "n", "a"]
x.index("a") == 1      # bAnana
x.index("a", 2) == 3   # banAna
x.index("a", -2) == 5  # bananA
```

---

## list.insert

```python
def list.insert(index: int, el, /) -> None
```

[list.insert]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·insert ): insert an element in a list.

`L.insert(i, x)` inserts the value `x` in the list L at index `i`,
moving higher-numbered elements along by one.  It returns `None`.

As usual, the index `i` must be an `int`. If its value is negative,
the length of the list is added, then its value is clamped to the
nearest value in the range `[0:len(L)]` to yield the effective index.

`insert` fails if the list is frozen or has active iterators.

```
x = ["b", "c", "e"]
x.insert(0, "a")
x.insert(-1, "d")
x == ["a", "b", "c", "d", "e"]
```

---

## list.pop

```python
def list.pop(index: int = ..., /)
```

[list.pop]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·pop ): removes and returns the last element of a list.

`L.pop([index])` removes and returns the last element of the list L, or,
if the optional index is provided, at that index.

`pop` fails if the index is negative or not less than the length of
the list, of if the list is frozen or has active iterators.

```
x = [1, 2, 3]
x.pop() == 3
x.pop() == 2
x == [1]
```

---

## list.remove

```python
def list.remove(needle, /) -> None
```

[list.remove]( https://github.com/bazelbuild/starlark/blob/master/spec.md#list·remove ): remove a value from a list

`L.remove(x)` removes the first occurrence of the value `x` from the
list L, and returns `None`.

`remove` fails if the list does not contain `x`, is frozen, or has
active iterators.

```
x = [1, 2, 3, 2]
x.remove(2)
x == [1, 3, 2]
x.remove(2)
x == [1, 3]
```

A subsequent call to `x.remove(2)` would yield an error because the
element won't be found.

```
x = [1, 2, 3, 2]
x.remove(2)
x.remove(2)
x.remove(2) # error: not found
```