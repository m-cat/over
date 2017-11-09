//! Tests.

use error::OverError;
use obj::Obj;
use types::Type;
use value::Value;

// Test setting and getting values.
#[test]
fn set_and_get() {
    let mut obj = Obj::new();

    // Null

    obj.set("null", Value::Null);
    assert_eq!(obj.get("null").unwrap(), Value::Null);
    assert!(obj.get("null").unwrap().is_null());

    // Bool

    obj.set("bool", true.into());
    assert_eq!(obj.get_bool("bool").unwrap(), true);
    assert_eq!(obj.get_bool("bool"), Ok(true));

    // Int

    obj.set("int", (-5).into());
    assert_eq!(obj.get_int("int"), Ok((-5).into()));

    // Frac

    obj.set("frac", frac!(1, 1).into());
    assert_eq!(obj.get_frac("frac"), Ok(frac!(1, 1)));

    // Char

    obj.set("char", 'x'.into());
    assert_eq!(obj.get_char("char"), Ok('x'));

    // String

    obj.set("str1", "hello".into());
    obj.set("str2", "yo".into());

    assert_eq!(obj.get("str1").unwrap(), "hello");
    assert_eq!(obj.get("str2").unwrap(), String::from("yo"));

    // Arr

    let arr = arr![-5, 0, 1];
    obj.set("arr", arr.clone().into());
    assert_eq!(obj.get("arr").unwrap(), arr);

    // Tup

    // TODO:

    // Obj

    // TODO:

    // Errors

    let res = obj.get("bool").unwrap().get_str();
    assert_eq!(res, Err(OverError::TypeMismatch(Type::Bool, Type::Str)));

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

    // Test object equality when parents are involved.

    assert_ne!(obj, def1);
    assert_ne!(def1, def2);
    assert_ne!(obj, def2);

    let mut obj2 = Obj::new();
    obj2.set_parent(&def1).unwrap();

    assert_eq!(obj, obj2);

    obj2.set("test", true.into());
    assert_ne!(obj, obj2);

    // Bool

    obj.set("bool1", true.into());

    def1.set("bool1", true.into());
    def1.set("bool2", false.into());

    def2.set("bool2", true.into());
    def2.set("bool3", true.into());

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

    obj.set("arr_char", arr!['w', 'o', 'w'].into());
    assert_eq!(
        obj.get("arr_char").unwrap().get_type(),
        Type::Arr(Box::new(Type::Char))
    );

    obj.set(
        "arr_arr",
        try_arr![arr![], arr![true, false]].unwrap().into(),
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
        tup!('!', tup!(-1), try_arr!["test", "heya"].unwrap()).into(),
    );
    assert_eq!(obj.get("tup").unwrap().get_type(), tup_type);

    // Misc

    assert_ne!(obj.get("bool").unwrap().get_type(), null.get_type());
    assert_ne!(
        obj.get("bool").unwrap().get_type(),
        obj.get("str").unwrap().get_type()
    )
}
