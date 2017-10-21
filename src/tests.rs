//! Tests.

use error::OverError;
use fraction::Fraction;
use obj::Obj;
use types::Type;
use value::Value;

// Test setting and getting values, both directly and through parents.
#[test]
fn set_and_get() {
    let mut obj = Obj::new();

    // Null

    obj.set("null", Value::Null);
    assert_eq!(obj.get("null").unwrap(), Value::Null);
    assert!(obj.get("null").unwrap().is_null());

    // Bool

    obj.set("bool", true.into());
    assert_eq!(obj.get("bool").unwrap(), true);
    assert_eq!(obj.get("bool").unwrap().get_bool(), Ok(true));

    // Int

    obj.set("int", (-5).into());
    assert_eq!(obj.get("int").unwrap().get_int(), Ok(-5));

    // Frac

    obj.set("frac", Fraction::new_neg(1u8, 1u8).into());
    assert_eq!(
        obj.get("frac").unwrap().get_frac(),
        Ok(Fraction::new_neg(1u8, 1u8))
    );

    // Char

    obj.set("char", 'x'.into());
    assert_eq!(obj.get("char").unwrap().get_char(), Ok('x'));

    // String

    obj.set("str1", "hello".into());
    obj.set("str2", "yo".into());

    assert_eq!(obj.get("str1").unwrap(), "hello");
    assert_eq!(obj.get("str2").unwrap(), String::from("yo"));

    // Arr

    let arr = arr_vec![-5, 0, 1];
    obj.set("arr", arr.clone().into());
    assert_eq!(obj.get("arr").unwrap(), arr);

    // Tup

    // Obj

    // Errors

    let res = obj.get("bool").unwrap().get_str();
    assert_eq!(res, Err(OverError::TypeMismatch));

    assert_eq!(obj.get(""), None);
    assert_eq!(obj.get("cool"), None);
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

    obj.set("bool1", true.into());

    def1.set("bool1", true.into());
    def1.set("bool2", false.into());

    def2.set("bool2", true.into());
    def2.set("bool3", true.into());

    assert_eq!(obj.get("bool1").unwrap().get_bool(), Ok(true));
    assert_eq!(obj.get("bool2").unwrap(), false);
    assert_eq!(obj.get("bool3").unwrap(), true);

    // String

    let str1 = String::from("hi");
    let str2 = String::from("bye");

    obj.set("test1", str1.clone().into());

    def1.set("test2", str2.clone().into());

    assert_eq!(obj.get("test1").unwrap(), str1);
    assert_eq!(obj.get("test2").unwrap(), str2);
}

#[test]
fn types() {
    let mut obj = Obj::new();

    // Null

    let null = Value::Null;
    assert_eq!(null.get_type(), Type::Null);

    // Bool

    obj.set("bool", true.into());
    assert_eq!(obj.get("bool").unwrap().get_type(), Type::Bool);

    // String

    obj.set("str", "".into());
    assert_eq!(obj.get("str").unwrap().get_type(), Type::Str);

    // Arr

    obj.set("arr_char", arr_vec!['w', 'o', 'w'].into());
    assert_eq!(
        obj.get("arr_char").unwrap().get_type(),
        Type::Arr(Box::new(Type::Char))
    );

    obj.set(
        "arr_arr",
        try_arr_vec![arr_vec![], arr_vec![true, false]]
            .unwrap()
            .into(),
    );
    assert_eq!(
        obj.get("arr_arr").unwrap().get_type(),
        Type::Arr(Box::new(Type::Arr(Box::new(Type::Bool))))
    );

    // Tup

    let tup_type = Type::Tup(vec![
        Type::Char,
        Type::Tup(vec![Type::Int]),
        Type::Arr(Box::new(Type::Str)),
    ]);
    obj.set(
        "tup",
        tup_vec!['!', tup_vec![-1], try_arr_vec!["test", "heya"].unwrap()].into(),
    );
    assert_eq!(obj.get("tup").unwrap().get_type(), tup_type);

    // Misc

    assert_ne!(obj.get("bool").unwrap().get_type(), null.get_type());
    assert_ne!(
        obj.get("bool").unwrap().get_type(),
        obj.get("str").unwrap().get_type()
    )
}
