#[macro_use]
extern crate over;

use over::OverError;
use over::obj::Obj;
use over::value::Value;

// Test reading basic Ints, Strs, Bools, and Null.
// Also test that whitespace and comments are correctly ignored.
#[test]
fn basic() {
    let obj = Obj::from_file("tests/test_files/basic.over").unwrap();

    assert_eq!(obj.get("a1").unwrap(), 1);
    assert_eq!(obj.get("a2").unwrap(), 2);
    assert_eq!(obj.get("aa").unwrap(), 0);
    assert_eq!(obj.get("b").unwrap(), 1);
    assert_eq!(obj.get("c").unwrap(), 10);
    assert_eq!(obj.get("d").unwrap(), 20);
    assert_eq!(obj.get("eee").unwrap(), 2);
    assert_eq!(obj.get("f").unwrap(), 3);
    assert_eq!(obj.get("g_").unwrap(), 4);
    assert_eq!(obj.get("Hello").unwrap(), "Hello");
    assert_eq!(obj.get("i_robot").unwrap(), "not #a comment");
    assert_eq!(obj.get("j").unwrap(), 4);
    assert_eq!(obj.get("k").unwrap(), "hi");
    assert_eq!(obj.get("l").unwrap(), "$\\\"");
    assert_eq!(obj.get("m").unwrap(), "m");
    assert_eq!(obj.get("n").unwrap(), true);
    assert_eq!(obj.get("o").unwrap(), false);
    assert_eq!(obj.get("p").unwrap(), "Hello");
    assert_eq!(obj.get("q").unwrap(), 0);
    assert_eq!(obj.get("r").unwrap(), Value::Null);
    assert_eq!(obj.get("s").unwrap(), '\'');
    assert_eq!(obj.get("t").unwrap(), '\n');
    assert_eq!(obj.get("u").unwrap(), ' ');
}

// Test parsing of Obj.
#[test]
fn obj() {
    let obj = Obj::from_file("tests/test_files/obj.over").unwrap();

    assert_eq!(obj.get("empty").unwrap().get_obj().unwrap().len(), 0);
    assert_eq!(obj.get("empty2").unwrap().get_obj().unwrap().len(), 0);

    assert!(!obj.contains("bools"));
    let mut bools = Obj::new();
    bools.set("t", true.into());
    bools.set("f", false.into());

    let outie = obj.get("outie").unwrap().get_obj().unwrap();
    assert_eq!(outie.get_parent().unwrap(), bools);
    assert_eq!(outie.get("z").unwrap(), 0);
    let inner = outie.get("inner").unwrap().get_obj().unwrap();
    let innie = inner.get("innie").unwrap().get_obj().unwrap();
    assert_eq!(innie.get("a").unwrap(), 1);
    assert_eq!(inner.get("b").unwrap(), tup_vec!(1, 2));
    assert_eq!(outie.get("c").unwrap(), 3);
    assert_eq!(outie.get("d").unwrap(), Obj::new());

    let obj_arr = obj.get("obj_arr").unwrap().get_obj().unwrap();
    assert_eq!(obj_arr.get("arr").unwrap(), arr_vec![1, 2, 3]);
}

// Test that globals are referenced correctly and don't get included as fields.
#[test]
fn globals() {
    let obj = Obj::from_file("tests/test_files/globals.over").unwrap();

    let sub = obj.get("sub").unwrap().get_obj().unwrap();

    assert_eq!(sub.get("a").unwrap(), 1);
    assert_eq!(sub.get("b").unwrap(), 2);
    assert_eq!(sub.len(), 2);

    assert_eq!(obj.get("c").unwrap(), 2);
    assert_eq!(obj.len(), 2);
}

// Test that parsing malformed .over files results in correct errors being returned.
#[test]
fn errors() {
    // TODO: use a macro to cut down on code here
    macro_rules! error_helper {
        ( $filename:expr, $error:expr ) => {
            {
                match Obj::from_file($filename) {
                    Err(OverError::ParseError(s)) => {
                        if s != $error {
                            panic!("{:?}", s);
                        }
                    }
                    res => panic!("{:?}", res),
                }
            }
        }
    }

    error_helper!(
        "tests/test_files/errors/field_true.over",
        "Invalid field name \"true\" at line 1, column 1"
    );
    error_helper!(
        "tests/test_files/errors/value_amp.over",
        "Invalid value \"@\" at line 1, column 8"
    );
    error_helper!(
        "tests/test_files/errors/dup_global.over",
        "Duplicate global \"@global\" at line 2, column 1"
    );
    error_helper!(
        "tests/test_files/errors/arr_types.over",
        "Arr inner types do not match: found Arr(Tup(Int, Char)), \
                   expected Arr(Tup(Int, Int))"
    );
    error_helper!(
        "tests/test_files/errors/empty_field.over",
        "Invalid character \':\' for field at line 1, column 1"
    );
    // error_helper!("tests/test_files/errors/fuzz1.over", "test");
}
