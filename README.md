# OVER

## About

"OVER" stands for "Overman's Awesome Format". It's a data exchange format like XML or JSON.

OVER is great, dude. It's designed to be easy for humans to read and write, and trivially easy for computers to parse. It also is resilient to errors by design. It features simple yet well-chosen types which are sufficient to represent any data you'll ever run into.

### Features

* Simple, error-resistent, flexible syntax.
* Commas are entirely optional and are ignored.

### Examples

### Types

#### Bool

`true`, or `false`. Take your pick.

#### Int

A 64-bit signed integer type. Any token beginning with `-`, `+`, or a numeral will be either an `Int` or a `Frac` (see below).

**Examples:** `1`, `-2`, `+4`

#### Frac

A sane representation of decimal values. Throw JSON's float types in the trash bin and use fractions instead.

**Examples:** `1/3`, `-5,1/4`, `+2,1/2`, `42,6/1`

#### Char

A type representing a single, unicode character.

#### Str

A unicode string type.

#### Arr

An array container which can hold an arbitrary number of elements of a single type.

#### Tup

A tuple container which can hold elements of different types.

#### Obj

The godfather of the types, the *object*. A hashmap of keys to values, where values can be any type, including other objects. Objects can also be `null`.

## What's wrong with JSON?

JSON is a format designed by dumbos, for dumbos. What is it that makes JSON worthy for the trash bin?

* Bad syntax, chosen for compatibility with Javascript.
* The last element in a list cannot have a trailing comma.
* No comments? Are you fucking kidding me?
* Floating-point numeric type.
* No type for single characters.
* Arrays where different types are allowed. This only makes sense in Javascript fairy-tale land.

Maybe JSON is the best option if you work with Javascript, but the rest of us should try to avoid bad languages and their bad formats.

## Disclaimer

I'm not responsible for anything.

(c) 2017 Marcin Swieczkowski
