# OVER

[![](https://img.shields.io/crates/v/over.svg)](https://crates.io/crates/over) [![Documentation](https://docs.rs/over/badge.svg)](https://docs.rs/over) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![LoC](https://tokei.rs/b1/github/m-cat/over)](https://github.com/m-cat/over)

OVER: the best data format.

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [OVER](#over)
    - [About](#about)
    - [Usage](#usage)
    - [Examples](#examples)
        - [The Basics](#the-basics)
        - [Containers](#containers)
        - [Variables](#variables)
        - [Parents](#parents)
        - [File Includes](#file-includes)
        - [Arithmetic on Values and Variables](#arithmetic-on-values-and-variables)
        - [String Substitutions](#string-substitutions)
    - [Types](#types)
        - [Null](#null)
        - [Bool](#bool)
        - [Int](#int)
        - [Frac](#frac)
        - [Char](#char)
        - [Str](#str)
        - [Arr](#arr)
        - [Tup](#tup)
        - [Obj](#obj)
    - [What's wrong with JSON?](#whats-wrong-with-json)
    - [What about YAML/others?](#what-about-yamlothers)
    - [Change Log](#change-log)
    - [Copyright](#copyright)

<!-- markdown-toc end -->

## About

OVER is a general-purpose data format like XML or JSON, but much better. Here are some of its key features:

* OVER is designed to be intuitive for humans to read and write without sacrificing flexibility. 
* It is powerful, with concepts such as variables, file includes, and object parents.
* It has an elegant and versatile type system which can safely represent all common data.
* It is resilient to errors by design and has no weird behavior or syntax like YAML or TOML. 

## Usage

Add OVER to your `Cargo.toml`:

```toml
[dependencies]
over = "*"
```

Example Rust code reading the first example ("The Basics") from this README:

```rust
extern crate fraction;
#[macro_use]
extern crate over;

use fraction::Fraction;
use over::obj::Obj;

#[test]
fn example() {
    let obj = Obj::from_file("tests/test_files/example.over").unwrap();

    assert_eq!(obj.get("receipt").unwrap(), "Oz-Ware Purchase Invoice");
    assert_eq!(obj.get("date").unwrap(), "2012-08-06");
    assert_eq!(
        obj.get("customer").unwrap(),
        obj!{"first_name" => "Dorothy",
             "family_name" => "Gale"}
    );

    assert_eq!(
        obj.get("items").unwrap(),
        arr![
            obj!{"part_no" => "A4786",
                 "descrip" => "Water Bucket (Filled)",
                 "price" => 1.47,
                 "quantity" => 4},
            obj!{"part_no" => "E1628",
                 "descrip" => "High Heeled \"Ruby\" Slippers",
                 "size" => 8,
                 "price" => 133.7,
                 "quantity" => 1},
        ]
    );

    assert_eq!(
        obj.get("bill_to").unwrap(),
        obj!{"street" => "123 Tornado Alley\nSuite 16",
             "city" => "East Centerville",
             "state" => "KS",
        }
    );

    assert_eq!(obj.get("ship_to").unwrap(), obj.get("bill_to").unwrap());

    assert_eq!(
        obj.get("specialDelivery").unwrap(),
        "Follow the Yellow Brick Road to the Emerald City. \
         Pay no attention to the man behind the curtain."
    );
}
```

Currently OVER has only been implemented for Rust; more languages may be supported in the future.

## Examples

### The Basics

A basic usage of OVER as a data format might look like this:

```
receipt: "Oz-Ware Purchase Invoice"
date:    "2012-08-06"
customer: {
    first_name:  "Dorothy"
    family_name: "Gale"
}

items: [{
         part_no:  "A4786"
         descrip:  "Water Bucket (Filled)"
         price:    01.47
         quantity: 4
        }
        {
         part_no:  "E1628"
         descrip:  "High Heeled \"Ruby\" Slippers"
         size:     8
         price:    133.70
         quantity: 1
        }
       ]
       
bill_to: {
    street: 
    # A multi-line string. Can also be written as "123 Tornado Alley\nSuite16"
"123 Tornado Alley
Suite 16"
    city:  "East Centerville"
    state: "KS"
}

ship_to: bill_to

specialDelivery:
"Follow the Yellow Brick Road to the Emerald City. Pay no attention to the man behind the curtain."
```

This basic example already demonstrates a lot of nice features about OVER:

* You can immediately tell what kind of data each field contains.
* Multi-line strings require no special syntax; see `bill_to.street`.
* Variables; see `ship_to`.
* Comments (sounds simple, but JSON doesn't have them).

### Containers

OVER has three container types:
* An [array] where all elements must be of the same type of data (enforced by the parser).
* A (tuple) which can hold elements of different types.
* {Objects}.

The following is a valid array, as each sub-tuple is the same type:

`[ ("Alex" 10) ("Alice" 12) ]`

The following is not a valid array. Can you see why?

`[ ("Morgan" 13) ("Alan" 15 16) ]`

### Variables

Fields defined in an object can be referenced later, but only in the same scope of the object:

```
var: 2

number: var

obj: {
    number: var # Invalid!
}
```

You can also define global variables. These are private and do not appear as fields in the final data:

```
@var: 2

number: @var

obj: {
    number: @var # Valid!
}
```

### Parents

An object can inherit the fields of another object. In the following example we define a template object called `@default` and define it to be the parent of `foo` and `bar` using the `^` field:

```
@default: {
    a: 1
    b: 2
}

foo: {
    ^: @default
    b: 5 # Override the value of "b" inherited from "@default".
    # foo.a == 1
}

bar: {
    ^: @default
    a: 5 # Override the value of "a" inherited from "@default".
    # bar.b == 2
}
```

### File Includes

Coming in the next release!

### Arithmetic on Values and Variables

Coming soon!

### String Substitutions

Coming soon!

## Types

### Null

A simple null value, represented by `null`.

### Bool

`true`, or `false`. Take your pick.

### Int

An arbitrary-length signed integer type. Any token beginning with `-`, `+`, or a numeral will be either an `Int` or a `Frac` (see below).

**Examples:** `1`, `-2`, `+4`

### Frac

A sane representation of decimal values. Forget about float types and use fractions instead.

**Examples:** `-1/3`, `-5,1/4`, `+2,1/2`, `42,6/1`

Fracs can also be written as decimals, which get converted automatically to fraction representation.

**Examples:** `2.5`, `-.0`

### Char

A type representing a single, unicode character.

**Examples:** `'q'`, `' '`

### Str

A unicode string type. 

**Examples:** `"smörgåsbord"`, `"A string with \"quotes\""`

Multi-line strings are trivial:

```
"You don't need any
special syntax for multi-line strings;
newlines are captured automatically."
```

### Arr

An array container which can hold an arbitrary number of elements of a single type.

**Examples:** `[]`, `[1 2 3]`, `[(1 2) (3 4)]`

### Tup

A tuple container which can hold elements of different types.

**Examples:** `()`, `(1 "John")` `( ('x' 1/2) [1 2 3] )`

### Obj

The godfather of all types, the *object*. A hashmap of keys, which we call *fields*, to values, where a value can be of any type, including other objects.

Fields must be followed by a colon and cannot be "null", "true", or "false".

**Examples:** 

`{ a: 1 b: 2 list: [a b b a] }`

`{ id: 4 field: { field: "Objects can be nested and each has their own scope." } }`

## What's wrong with JSON?

I started this project because I wanted an alternative to JSON. Why?

* The last element in a list cannot have a trailing comma.
* What's with the opening/closing braces?
* Floating-point numeric type.
* Arrays where different types are allowed.
* Field names have to be in quotes, e.g. `"name": "Johnny"` instead of `name: "Johnny"`. It's verbose and not ergonomic.
* Finally, no support for comments is a dealbreaker. Some JSON implementations allow them, but it's not standard.

JSON and other options are also lacking many of the features that I'm interested in, such as the ability to define variables and the concept of object parents. 

## What about YAML/others?

Let's compare the first example in this README ("The Basics") with the YAML version of the same data, taken from [Wikipedia](https://en.wikipedia.org/wiki/YAML#Example):

```yaml
---
receipt:     Oz-Ware Purchase Invoice
date:        2012-08-06
customer:
    first_name:   Dorothy
    family_name:  Gale

items:
    - part_no:   A4786
      descrip:   Water Bucket (Filled)
      price:     1.47
      quantity:  4

    - part_no:   E1628
      descrip:   High Heeled "Ruby" Slippers
      size:      8
      price:     133.7
      quantity:  1

bill-to:  &id001
    street: |
            123 Tornado Alley
            Suite 16
    city:   East Centerville
    state:  KS

ship-to:  *id001

specialDelivery:  >
    Follow the Yellow Brick
    Road to the Emerald City.
    Pay no attention to the
    man behind the curtain.
...
```

As you can see, OVER is much more understandable, even if you're not at all familiar with it. The YAML version has strange syntax in some places (such as `&id001` and `*id001`; plus, what in the world are `>` and `|` supposed to be?) and a lack of useful syntax in others (every value looks like a string). Is the "data" field a number or a string? YAML is certainly more pleasing on a superficial level, which I suspect is the only reason it entered into general use, but it fails to stand up to light scrutiny.

Look at [this answer](https://stackoverflow.com/a/18708156) on StackExchange for an example of how unintuitive YAML is. Trust me, that's not even the worst of it; there is a shocking amount of weirdness in the official spec. This design disaster also makes it impossible to write an efficient parser for it.

Finally, as seen throughout this README, OVER manages to be more powerful than YAML while being much simpler! This may strike you as a paradox, but it is just a consequence of the thoughtless design of YAML and company (don't think I've forgotten about TOML). There are options such as [StrictYAML](https://github.com/crdoconnor/strictyaml) but they are, in my opinion, just bandaids on a broken solution.

## Change Log

See [CHANGELOG.md](CHANGELOG.md).

## Copyright

(c) 2017 Marcin Swieczkowski
