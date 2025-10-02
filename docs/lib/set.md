# set

```python
def set(arg: typing.Iterable = ..., /) -> set[typing.Any]
```

---

## set.add

```python
def set.add(value, /) -> None
```

Add an item to the set. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) x.add(4) x == set([1, 2, 3, 4]) # "#); ```

---

## set.clear

```python
def set.clear() -> None
```

---

## set.difference

```python
def set.difference(other: typing.Iterable, /) -> set[typing.Any]
```

Returns a new set with elements unique the set when compared to the specified iterable. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [3, 4, 5] x.difference(y) == set([1, 2]) # "#); ```

---

## set.discard

```python
def set.discard(value, /) -> None
```

Remove the item from the set. It does nothing if there is no such item.

`discard` fails if the key is unhashable or if the dictionary is
frozen.
Time complexity of this operation is *O(N)* where *N* is the number of entries in the set.

```
x = set([1, 2, 3])
x.discard(2)
x == set([1, 3])
```
A subsequent call to `x.discard(2)` would do nothing.
```
x = set([1, 2, 3])
x.discard(2)
x.discard(2)
x == set([1, 3])
```

---

## set.intersection

```python
def set.intersection(other: typing.Iterable, /) -> set[typing.Any]
```

Return a new set with elements common to the set and all others. Unlike Python does not support variable number of arguments. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [3, 4, 5] x.intersection(y) == set([3]) # "#); ```

---

## set.issubset

```python
def set.issubset(other: typing.Iterable, /) -> bool
```

Test whether every element in the set is in other iterable. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [3, 1, 2] x.issubset(y) # "#); ```

---

## set.issuperset

```python
def set.issuperset(other: typing.Iterable, /) -> bool
```

Test whether every element other iterable is in the set. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [1, 3] x.issuperset(y) == True # "#); ```

---

## set.pop

```python
def set.pop()
```

Removes and returns the **last** element of a set.

`S.pop()` removes and returns the last element of the set S.

`pop` fails if the set is empty, or if the set is frozen or has active iterators.
Time complexity of this operation is *O(1)*.

```
x = set([1, 2, 3])
x.pop() == 3
x.pop() == 2
x == set([1])
```

---

## set.remove

```python
def set.remove(value, /) -> None
```

Remove the item from the set. It raises an error if there is no such item.

`remove` fails if the key is unhashable or if the dictionary is
frozen.
Time complexity of this operation is *O(N)* where *N* is the number of entries in the set.

```
x = set([1, 2, 3])
x.remove(2)
x == set([1, 3])
```
A subsequent call to `x.remove(2)` would yield an error because the
element won't be found.
```
x = set([1, 2, 3])
x.remove(2)
x.remove(2) # error: not found
```

---

## set.symmetric\_difference

```python
def set.symmetric_difference(other: typing.Iterable, /) -> set[typing.Any]
```

Returns a new set with elements in either the set or the specified iterable but not both. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [3, 4, 5] x.symmetric_difference(y) == set([1, 2, 4, 5]) # "#); ```

---

## set.union

```python
def set.union(other: typing.Iterable, /) -> set[typing.Any]
```

Return a new set with elements from the set and all others. Unlike Python does not support variable number of arguments. ``` # starlark::assert::is_true(r#" x = set([1, 2, 3]) y = [3, 4, 5] x.union(y) == set([1, 2, 3, 4, 5]) # "#); ```

---

## set.update

```python
def set.update(other: typing.Iterable, /) -> None
```

Update the set by adding items from an iterable. ``` # starlark::assert::is_true(r#" x = set([1, 3, 2]) x.update([4, 3]) list(x) == [1, 3, 2, 4] # "#); ```