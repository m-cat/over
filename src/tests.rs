//! Tests.

use object::*;
use value::Value;
use error::Error;

#[test]
fn obj_get() {
    let mut obj = Object::new();
    let mut def1 = Object::new();
    let mut def2 = Object::new();

    // Basic

    obj.set("basic1", "hi");
    def1.set("basic2", "bye");

    assert_eq!(obj_get!(Str, "basic1", obj => def1), Ok("hi"));
    assert_eq!(obj_get!(Str, "basic2", obj => def1), Ok("bye"));

    // Bool

    obj.set("bool1", true);

    def1.set("bool1", true);
    def1.set("bool2", false);

    def2.set("bool2", true);
    def2.set("bool3", true);

    assert_eq!(obj_get!(Bool, "bool1", obj).unwrap(), true);
    assert_eq!(obj_get!(Bool, "bool1", obj => def1 => def2).unwrap(), true);
    assert_eq!(obj_get!(Bool, "bool2", obj => def1 => def2).unwrap(), false);
    assert_eq!(obj_get!(Bool, "bool3", obj => def1 => def2).unwrap(), true);

    // String

    obj.set("str1", "hello");
    obj.set("str2", String::from("yo"));

    assert_eq!(obj_get!(Str, "str1", obj).unwrap(), "hello");
    assert_eq!(obj_get!(Str, "str2", obj).unwrap(), "yo");

    // Errors

    // assert_eq!(obj_get!(Str, "bool1", obj), Err(Error::WrongTypeFound));

    // assert_eq!(obj_get!(Bool, "bool2", obj), Err(Error::FieldNotFound));
    // assert_eq!(obj_get!(Bool, "bool4", obj => def1 => def2), Err(Error::FieldNotFound));
}
