//! Tests.

use arr::Arr;
use error::OverError;
use fraction::Fraction;
use obj::Obj;
use tup::Tup;
use types::Type;
use value::Value;

// Test setting and getting values, both directly and through parents.
#[test]
fn set_and_get() {
    let mut obj = Obj::new();

    // Null

    obj.set("null", Value::null());
    assert_eq!(obj.get("null").unwrap(), Value::null());
    assert!(obj.get("null").unwrap().is_null());

    // Bool

    obj.set("bool", Value::new_bool(true));
    assert_eq!(obj.get("bool").unwrap().get_bool(), Ok(true));

    // Int

    obj.set("int", Value::new_int(-5));
    assert_eq!(obj.get("int").unwrap().get_int(), Ok(-5));

    // Frac

    obj.set("frac", Value::new_frac(Fraction::new_neg(1u8, 1u8)));
    assert_eq!(
        obj.get("frac").unwrap().get_frac(),
        Ok(Fraction::new_neg(1u8, 1u8))
    );

    // Char

    obj.set("char", Value::new_char('x'));
    assert_eq!(obj.get("char").unwrap().get_char(), Ok('x'));

    // String

    obj.set("str1", Value::new_str("hello"));
    obj.set("str2", Value::new_str("yo"));

    assert_eq!(obj.get("str1").unwrap(), "hello");
    assert_eq!(obj.get("str2").unwrap(), String::from("yo"));

    // Arr

    let arr = arr_vec![Int; -5, 0, 1];
    obj.set("arr", Value::new_arr(arr.clone()));
    assert_eq!(obj.get("arr").unwrap(), arr);

    // Tup

    // Obj

    // Errors

    let res = obj.get("bool").unwrap().get_str();
    assert_eq!(res, Err(OverError::TypeMismatch));

    assert_eq!(obj.get(""), Err(OverError::FieldNotFound("".into())));
    assert_eq!(
        obj.get("cool"),
        Err(OverError::FieldNotFound("cool".into()))
    );
}

// Test setting and getting values through parents.
#[test]
fn parents() {
    let mut obj = Obj::new();
    let mut def1 = Obj::new();
    let mut def2 = Obj::new();

    obj.set_parent(&def1).unwrap();

    def1.set_parent(&def2).unwrap();

    // Bool

    obj.set("bool1", Value::new_bool(true));

    def1.set("bool1", Value::new_bool(true));
    def1.set("bool2", Value::new_bool(false));

    def2.set("bool2", Value::new_bool(true));
    def2.set("bool3", Value::new_bool(true));

    assert_eq!(obj.get("bool1").unwrap().get_bool(), Ok(true));
    assert_eq!(obj.get("bool2").unwrap(), false);
    assert_eq!(obj.get("bool3").unwrap(), true);

    // String

    let str1 = String::from("hi");
    let str2 = String::from("bye");

    obj.set("test1", Value::new_str(&str1));

    def1.set("test2", Value::new_str(&str2));

    assert_eq!(obj.get("test1").unwrap(), str1);
    assert_eq!(obj.get("test2").unwrap(), str2);
}

#[test]
fn types() {
    let mut obj = Obj::new();

    // Null

    let null = Value::null();
    assert_eq!(null.get_type(), Type::Null);

    // Bool

    obj.set("bool", Value::new_bool(true));
    assert_eq!(obj.get("bool").unwrap().get_type(), Type::Bool);

    // String

    obj.set("str", Value::new_str(""));
    assert_eq!(obj.get("str").unwrap().get_type(), Type::Str);

    // Arr

    obj.set("arr_char", Value::new_arr(arr_vec![Char; 'w', 'o', 'w']));
    assert_eq!(
        obj.get("arr_char").unwrap().get_type(),
        Type::Arr(Box::new(Type::Char))
    );

    obj.set(
        "arr_arr",
        Value::new_arr(
            try_arr_vec![Arr;
                     arr_vec![],
                     arr_vec![Bool; true, false]
            ].unwrap(),
        ),
    );
    assert_eq!(
        obj.get("arr_arr").unwrap().get_type(),
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Bool))))
    );

    // Tup

    let tup_type = Type::Tup(vec![
        Type::Char,
        Type::Tup(vec![]),
        Type::Arr(Box::new(Type::Str)),
    ]);
    obj.set(
        "tup",
        Value::new_tup(Tup::from_vec(vec![
            Value::new_char('!'),
            Value::new_tup(Tup::from_vec(vec![])),
            Value::new_arr(
                Arr::from_vec(
                    vec![Value::new_str("test"), Value::new_str("heya")],
                ).unwrap()
            ),
        ])),
    );
    assert_eq!(obj.get("tup").unwrap().get_type(), tup_type);

    // Misc

    assert_eq!(
        obj.get("not there"),
        Err(OverError::FieldNotFound("not there".into()))
    );
}
