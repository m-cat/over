#![allow(clippy::cyclomatic_complexity)]

//! Tests.

use crate::error::OverError;
use crate::types::Type;
use crate::value::Value;

// Display nicely-formatted values on failure.
macro_rules! test_eq {
    ($left:expr, $right:expr) => {{
        if $left != $right {
            panic!(format!(
                "Left did not equal right.\nLeft: {}\nRight: {}\n",
                $left, $right
            ));
        }
    }};
}

// Test setting and getting values.
#[test]
fn set_and_get() {
    let obj = obj! {
        "null" => Value::Null,
        "bool" => true,
        "int" => -5,
        "frac" => frac!(1, 1),
        "char" => 'x',
        "str1" => "hello",
        "str2" => "yo",
        "arr" => arr![-5, 0, 1],
    };

    // Null

    test_eq!(obj.get("null").unwrap(), Value::Null);
    assert!(obj.get("null").unwrap().is_null());

    // Bool

    test_eq!(obj.get_bool("bool").unwrap(), true);
    assert_eq!(obj.get_bool("bool"), Ok(true));

    // Int

    assert_eq!(obj.get_int("int"), Ok((-5).into()));

    // Frac

    assert_eq!(obj.get_frac("frac"), Ok(frac!(1, 1)));

    // Char

    assert_eq!(obj.get_char("char"), Ok('x'));

    // String

    let yo = String::from("yo");
    test_eq!(obj.get("str1").unwrap(), "hello");
    test_eq!(obj.get("str2").unwrap(), yo);

    // Arr

    test_eq!(obj.get("arr").unwrap(), arr![-5, 0, 1]);

    // Errors

    let res = obj.get("bool").unwrap().get_str();
    assert_eq!(res, Err(OverError::TypeMismatch(Type::Str, Type::Bool)));

    assert_eq!(obj.get(""), None);
    assert_eq!(obj.get("cool"), None);
}

// Test setting and getting values through parents.
#[test]
fn parents() {
    let def2 = obj! {
        "bool2" => true,
        "bool3" => true
    };
    let def1 = obj! {
        "^" => def2.clone(),
        "bool1" => true,
        "bool2" => false,
        "test2" => "bye",
    };
    let obj = obj! {
        "^" => def1.clone(),
        "bool1" => true,
        "test1" => "hi",
    };

    // Test object equality when parents are involved.

    assert_ne!(obj, def1);
    assert_ne!(def1, def2);
    assert_ne!(obj, def2);

    let obj2 = obj! { "^" => def1.clone() };
    assert_ne!(obj, obj2);

    let obj2 = obj! { "^" => def1.clone(), "test1" => "hi", "bool1" => true };
    test_eq!(obj, obj2);

    // Bool

    let (v, o) = obj.get_with_source("bool1").unwrap();
    test_eq!(v, true);
    assert!(o.ptr_eq(&obj));

    let (v, o) = obj.get_with_source("bool2").unwrap();
    test_eq!(v, false);
    assert!(o.ptr_eq(&def1));

    let (v, o) = obj.get_with_source("bool3").unwrap();
    test_eq!(v, true);
    assert!(o.ptr_eq(&def2));

    // String

    test_eq!(obj.get("test1").unwrap(), "hi");
    test_eq!(obj.get("test2").unwrap(), "bye");
}

#[test]
fn types() {
    let obj = obj! {
        "bool" => true,
        "str" => "",
        "arr_char" => arr!['w', 'o', 'w'],
        "arr_arr" => try_arr![arr![], arr![true, false]].unwrap(),
        "tup" => tup!('!', tup!(-1), try_arr!["test", "heya"].unwrap()),
    };

    // Null

    let null = Value::Null;
    test_eq!(null.get_type(), Type::Null);

    // Bool

    test_eq!(obj.get("bool").unwrap().get_type(), Type::Bool);

    // String

    test_eq!(obj.get("str").unwrap().get_type(), Type::Str);

    // Arr

    test_eq!(
        obj.get("arr_char").unwrap().get_type(),
        Type::Arr(Box::new(Type::Char))
    );

    test_eq!(
        obj.get("arr_arr").unwrap().get_type(),
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Bool))))
    );

    assert_ne!(
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Int)))),
        Type::Arr(Box::new(Type::Int))
    );

    // Tup

    let tup_type = Type::Tup(vec![
        Type::Char,
        Type::Tup(vec![Type::Int]),
        Type::Arr(Box::new(Type::Str)),
    ]);
    test_eq!(obj.get("tup").unwrap().get_type(), tup_type);

    // Misc

    assert_ne!(obj.get("bool").unwrap().get_type(), null.get_type());
    assert_ne!(
        obj.get("bool").unwrap().get_type(),
        obj.get("str").unwrap().get_type()
    )
}
