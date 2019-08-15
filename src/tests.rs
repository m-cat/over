#![allow(clippy::cognitive_complexity)]

//! Tests.

use crate::{error::OverError, types::Type, value::Value, OverResult, ReferenceType};
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};
use std::convert::TryInto;

// Test setting and getting values.
#[test]
fn set_and_get() -> OverResult<()> {
    let obj = obj! {
        "null" => Value::Null,
        "bool" => true,
        "int" => -5,
        "frac" => frac!(1, 1),
        "str1" => "hello",
        "str2" => "yo",
        "tup" => tup!["hi", 2, false],
        "arr" => arr![-5, 0, 1],
    };

    // Null

    assert_eq!(obj.get("null").unwrap(), Value::Null);
    assert!(obj.get("null").unwrap().is_null());

    // Bool

    assert_eq!(obj.get_bool("bool")?, true);
    assert_eq!(obj.get_bool("bool"), Ok(true));

    // Int

    assert_eq!(obj.get_int("int"), Ok((-5).into()));

    // Frac

    assert_eq!(obj.get_frac("frac"), Ok(frac!(1, 1)));

    // String

    let yo = String::from("yo");
    assert_eq!(obj.get("str1").unwrap(), "hello");
    assert_eq!(obj.get("str2").unwrap(), yo);

    // Arr

    assert_eq!(obj.get("arr").unwrap(), arr![-5, 0, 1]);
    assert_eq!(
        obj.get_arr("arr"),
        vec![
            Value::Int((-5).into()),
            Value::Int(0.into()),
            Value::Int(1.into())
        ]
        .try_into()
    );

    // Tup

    assert_eq!(obj.get("tup").unwrap(), tup!["hi", 2, false]);
    assert_eq!(
        obj.get_tup("tup"),
        Ok(vec![
            Value::Str("hi".into()),
            Value::Int(2.into()),
            Value::Bool(false)
        ]
        .into())
    );

    // Errors

    let res = obj.get("bool").unwrap().get_str();
    assert_eq!(res, Err(OverError::TypeMismatch(Type::Str, Type::Bool)));

    assert_eq!(obj.get(""), None);
    assert_eq!(obj.get("cool"), None);

    Ok(())
}

// Test setting and getting values through parents.
#[test]
fn parents() -> OverResult<()> {
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

    let obj2 = obj! { "^" => def1.clone(), "bool1" => true, "test1" => "hi" };
    assert_eq!(obj, obj2);

    // Test reference counts.

    assert_eq!(def2.num_references(), 2);
    assert_eq!(def2.num_references(), 2);
    assert_eq!(obj.num_references(), 1);

    // Bool

    let (v, o) = obj.get_with_source("bool1").unwrap();
    assert_eq!(v, true);
    assert!(o.ptr_eq(&obj));

    let (v, o) = obj.get_with_source("bool2").unwrap();
    assert_eq!(v, false);
    assert!(o.ptr_eq(&def1));

    let (v, o) = obj.get_with_source("bool3").unwrap();
    assert_eq!(v, true);
    assert!(o.ptr_eq(&def2));

    // String

    assert_eq!(obj.get("test1").unwrap(), "hi");
    assert_eq!(obj.get("test2").unwrap(), "bye");

    Ok(())
}

#[test]
fn types() -> OverResult<()> {
    let arr_str = arr!["w", "o", "w"];
    let obj = obj! {
        "bool" => true,
        "str" => "",
        "arr_str" => arr_str.clone(),
        "arr_str_dup" => arr_str.clone(),
        "arr_arr" => try_arr![arr![], arr![true, false]]?,
        "tup" => tup!("!", tup!(-1), try_arr!["test", "heya"]?),
    };

    // Test reference counts.

    assert_eq!(arr_str.num_references(), 3);
    assert_eq!(obj.num_references(), 1);

    // Null

    let null = Value::Null;
    assert_eq!(null.get_type(), Type::Null);

    // Bool

    assert_eq!(obj.get("bool").unwrap().get_type(), Type::Bool);

    // String

    assert_eq!(obj.get("str").unwrap().get_type(), Type::Str);

    // Arr

    assert_eq!(
        obj.get("arr_str").unwrap().get_type(),
        Type::Arr(Box::new(Type::Str))
    );

    assert_eq!(
        obj.get("arr_arr").unwrap().get_type(),
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Bool))))
    );

    assert_ne!(
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Int)))),
        Type::Arr(Box::new(Type::Int))
    );

    // Tup

    let tup_type = Type::Tup(vec![
        Type::Str,
        Type::Tup(vec![Type::Int]),
        Type::Arr(Box::new(Type::Str)),
    ]);
    assert_eq!(obj.get("tup").unwrap().get_type(), tup_type);

    // Misc

    assert_ne!(obj.get("bool").unwrap().get_type(), null.get_type());
    assert_ne!(
        obj.get("bool").unwrap().get_type(),
        obj.get("str").unwrap().get_type()
    );

    Ok(())
}
