# OVER

OVER: the best data format.

## About

OVER is a general-purpose data format like XML or JSON.

OVER is designed to be easy for humans to read and write and for computers to parse. It is resilient to errors by design and has no weird behavior or syntax like YAML or TOML. It features simple yet well-chosen types which are sufficient to represent any data you'll ever run into.

## Features

TODO

## Examples

TODO

## Types

### Null

A simple null value, represented by `null`.

### Bool

`true`, or `false`. Take your pick.

### Int

A 64-bit signed integer type. Any token beginning with `-`, `+`, or a numeral will be either an `Int` or a `Frac` (see below).

**Examples:** `1`, `-2`, `+4`

### Frac

A sane representation of decimal values. Throw JSON's float types in the trash bin and use fractions instead.

**Examples:** `-1/3`, `-5,1/4`, `+2,1/2`, `42,6/1`

Fracs can also be written as decimals, which get converted automatically to fraction representation.

**Examples:** `2.5`, `-.0`

### Char

A type representing a single, unicode character.

**Examples:** 'q', ' '

### Str

A unicode string type.

### Arr

An array container which can hold an arbitrary number of elements of a single type.

### Tup

A tuple container which can hold elements of different types.

### Obj

The godfather of all types, the *object*. A hashmap of keys to values, where values can be any type, including other objects.

## What's wrong with JSON?

JSON is a garbage format. Why?

* Bad syntax, chosen for compatibility with Javascript.
* The last element in a list cannot have a trailing comma.
* No support for comments? Are you fucking kidding me?
* Floating-point numeric type.
* Arrays where different types are allowed. This only makes sense in Javascript fairy-tale land.
* Field names have to be in quotes, e.g. `"name": "Johnny"` instead of `name: "Johnny"`. Why? Javascript.

Maybe JSON is the best option if you work with JS, but the rest of us should try to avoid bad languages and their bad formats.

## Disclaimer

I'm not responsible for anything.

(c) 2017 Marcin Swieczkowski
