extern crate fraction;
extern crate num;
#[macro_use]
extern crate over;

use fraction::BigFraction;
use num::ToPrimitive;
use over::OverError;
use over::obj::Obj;
use over::value::Value;

fn get_int(obj: &Obj, field: &str) -> i64 {
    obj.get_int(field).unwrap().to_i64().unwrap()
}

// Test parsing of empty file.
#[test]
fn empty() {
    let obj = Obj::from_file("tests/test_files/empty.over").unwrap();

    assert_eq!(obj.len(), 0);
}

// Test reading basic Ints, Strs, Bools, and Null.
// Also test that whitespace and comments are correctly ignored.
#[test]
fn basic() {
    let obj = Obj::from_file("tests/test_files/basic.over").unwrap();

    assert_eq!(get_int(&obj, "a1"), 1);
    assert_eq!(get_int(&obj, "a2"), 2);
    assert_eq!(get_int(&obj, "aa"), 0);
    assert_eq!(obj.get("b").unwrap(), "Smörgåsbord");
    assert_eq!(get_int(&obj, "c"), 10);
    assert_eq!(get_int(&obj, "d"), 20);
    assert_eq!(get_int(&obj, "eee"), 2);
    assert_eq!(get_int(&obj, "f"), 3);
    assert_eq!(get_int(&obj, "g_"), 4);
    assert_eq!(obj.get("Hello").unwrap(), "Hello");
    assert_eq!(obj.get("i_robot").unwrap(), "not #a comment");
    assert_eq!(get_int(&obj, "j"), 4);
    assert_eq!(obj.get("k").unwrap(), "hi");
    assert_eq!(obj.get("l").unwrap(), "$\\\"");
    assert_eq!(obj.get("m").unwrap(), "m");
    assert_eq!(obj.get("n").unwrap(), true);
    assert_eq!(obj.get("o").unwrap(), false);
    assert_eq!(obj.get("p").unwrap(), "Hello");
    assert_eq!(get_int(&obj, "q"), 0);
    assert_eq!(obj.get("r").unwrap(), Value::Null);
    assert_eq!(obj.get("s").unwrap(), '\'');
    assert_eq!(obj.get("t").unwrap(), '\n');
    assert_eq!(obj.get("u").unwrap(), ' ');
}

// Test the example from the README.
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

// Test parsing of sub-Objs.
#[test]
fn obj() {
    let obj = Obj::from_file("tests/test_files/obj.over").unwrap();

    assert_eq!(obj.get_obj("empty").unwrap().len(), 0);
    assert_eq!(obj.get_obj("empty2").unwrap().len(), 0);

    assert!(!obj.contains("bools"));
    let mut bools = Obj::new();
    bools.set("t", true.into());
    bools.set("f", false.into());

    let outie = obj.get_obj("outie").unwrap();
    assert_eq!(outie.get_parent().unwrap(), bools);
    assert_eq!(get_int(&outie, "z"), 0);
    let inner = outie.get_obj("inner").unwrap();
    assert_eq!(get_int(&inner, "z"), 1);
    let innie = inner.get_obj("innie").unwrap();
    assert_eq!(get_int(&innie, "a"), 1);
    assert_eq!(inner.get("b").unwrap(), tup!(1, 2,));
    assert_eq!(get_int(&outie, "c"), 3);
    assert_eq!(outie.get("d").unwrap(), Obj::new());

    let obj_arr = obj.get_obj("obj_arr").unwrap();
    assert_eq!(obj_arr.get("arr").unwrap(), arr![1, 2, 3]);
}

// Test that globals are referenced correctly and don't get included as fields.
#[test]
fn globals() {
    let obj = Obj::from_file("tests/test_files/globals.over").unwrap();

    let sub = obj.get_obj("sub").unwrap();

    assert_eq!(sub.get_int("a").unwrap(), int!(1));
    assert_eq!(get_int(&sub, "b"), 2);
    assert_eq!(sub.len(), 2);

    assert_eq!(get_int(&obj, "c"), 2);
    assert_eq!(obj.len(), 2);
}

// Test parsing of numbers.
#[test]
fn numbers() {
    let obj = Obj::from_file("tests/test_files/numbers.over").unwrap();

    assert_eq!(get_int(&obj, "neg"), -4);
    assert_eq!(obj.get_frac("pos").unwrap().to_f32(), Some(4f32));
    assert_eq!(obj.get_frac("neg_zero").unwrap().to_f32(), Some(0f32));
    assert_eq!(obj.get_frac("pos_zero").unwrap().to_f32(), Some(0f32));

    assert_eq!(obj.get("frac_from_dec").unwrap(), frac!(13, 10));
    assert_eq!(obj.get("neg_ffd").unwrap(), frac!(-13, 10));
    assert_eq!(obj.get("pos_ffd").unwrap(), BigFraction::new(13u8, 10u8));

    assert_eq!(obj.get("add_dec").unwrap(), BigFraction::new(3u8, 1u8));
    assert_eq!(obj.get("sub_dec").unwrap(), BigFraction::new_neg(3u8, 1u8));

    let frac = obj.get_frac("big_frac").unwrap();
    assert!(frac > BigFraction::new(91_000_000u64, 1u8));
    assert!(frac < BigFraction::new(92_000_000u64, 1u8));

    assert_eq!(obj.get("frac1").unwrap(), BigFraction::new(1u8, 2u8));
    assert_eq!(obj.get("frac2").unwrap(), BigFraction::new(1u8, 2u8));
    assert_eq!(obj.get("frac3").unwrap(), BigFraction::new(0u8, 10u8));
    assert_eq!(obj.get("frac4").unwrap(), BigFraction::new_neg(5u8, 4u8));
    assert_eq!(obj.get("frac5").unwrap(), BigFraction::new(1u8, 1u8));

    assert_eq!(obj.get("whole_frac").unwrap(), BigFraction::new(3u8, 2u8));
    assert_eq!(
        obj.get("neg_whole_frac").unwrap(),
        BigFraction::new_neg(21u8, 4u8)
    );
    assert_eq!(obj.get("dec_frac").unwrap(), BigFraction::new(1u8, 2u8));
    assert_eq!(
        obj.get("dec_frac2").unwrap(),
        BigFraction::new_neg(1u8, 2u8)
    );

    assert_eq!(
        obj.get("array").unwrap(),
        arr![
            obj.get_frac("whole_frac").unwrap(),
            BigFraction::new_neg(1u8, 2u8),
            BigFraction::new(3u8, 2u8),
            BigFraction::new(1u8, 1u8),
        ]
    );

    assert_eq!(
        obj.get("tup").unwrap(),
        tup!(
            BigFraction::new_neg(1u8,2u8),
            obj.get_frac("whole_frac").unwrap(),
            BigFraction::new(1u8,1u8),
            BigFraction::new(3u8,2u8),
        )
    );

    assert_eq!(obj.get("var_frac").unwrap(), BigFraction::new_neg(1u8, 2u8));
}

// TODO: Test includes.over

// TODO: Test multi-line.over (need substitution)

// Test writing objects to files.
#[test]
fn write() {
    let path1 = "tests/test_files/basic.over";
    let path2 = "tests/test_files/write.over";
    let obj1 = Obj::from_file(path1).unwrap();

    obj1.write_to_file(path2).unwrap();

    let obj2 = Obj::from_file(path2).unwrap();

    assert_eq!(obj1, obj2);
}

// Test fuzz files; just make sure there was no error in parsing.
#[test]
fn fuzz() {
    let _ = Obj::from_file("tests/test_files/fuzz1.over").unwrap();
}

// Test that parsing malformed .over files results in correct errors being returned.
#[test]
fn errors() {
    macro_rules! error_helper {
        ( $filename:expr, $error:expr ) => {
            {
                match Obj::from_file(&format!("tests/test_files/errors/{}", $filename)) {
                    Err(OverError::ParseError(s)) => {
                        if s != $error {
                            panic!("Error in {}: {:?}", $filename, s);
                        }
                    }
                    res => panic!("No error occurred in {}: {:?}", $filename, res),
                }
            }
        }
    }

    error_helper!(
        "arr_types.over",
        "Arr inner types do not match: found Arr(Tup(Int, Char)), \
         expected Arr(Tup(Int, Int)) at line 2, col 37"
    );
    error_helper!("decimal.over", "Invalid numeric value at line 1, column 10");
    error_helper!(
        "deep.over",
        "Exceeded maximum depth (128) for a container at line 1, column 142"
    );
    error_helper!(
        "dup_global.over",
        "Duplicate global \"@global\" at line 2, column 1"
    );
    error_helper!(
        "empty_field.over",
        "Invalid character \':\' for field at line 1, column 1"
    );
    error_helper!(
        "empty_number.over",
        "Invalid character \'\\n\' for value at line 1, column 7"
    );
    error_helper!(
        "field_true.over",
        "Invalid field name \"true\" at line 1, column 1"
    );
    error_helper!(
        "field_whitespace.over",
        "No whitespace after field at line 1, column 6"
    );
    error_helper!(
        "fuzz1.over",
        "Invalid closing bracket \')\' at line 20, column 1; expected \']\'"
    );
    error_helper!(
        "fuzz2.over",
        "Invalid closing bracket \')\' at line 22, column 2; expected none"
    );
    error_helper!(
        "fuzz3.over",
        "Invalid closing bracket \')\' at line 8, column 4; expected \']\'"
    );
    error_helper!("fuzz4.over", "Duplicate field \"M\" at line 22, column 1");
    error_helper!(
        "fuzz5.over",
        "Invalid character \'(\' for value at line 27, column 4"
    );
    error_helper!(
        "fuzz6.over",
        "Arr inner types do not match: found Frac, expected Int at line 22, col 1"
    );
    error_helper!(
        "fuzz7.over",
        "Invalid character \'\\n\' for field at line 8, column 0"
    );
    error_helper!(
        "fuzz8.over",
        "Invalid character \'\"\' for value at line 34, column 3"
    );
    error_helper!(
        "fuzz9.over",
        "Type mismatch: found Null, expected Obj at line 18, col 4"
    );
    error_helper!("fuzz10.over", "Unexpected end when reading value at line 1");
    error_helper!(
        "op_end.over",
        "Invalid character \'\\n\' for value at line 3, column 9"
    );
    error_helper!(
        "op_error.over",
        "Could not apply operator + on types Str and Int at line 1, column 16"
    );
    error_helper!(
        "unexpected_end1.over",
        "Unexpected end when reading value at line 2"
    );
    error_helper!(
        "unexpected_end2.over",
        "Unexpected end when reading value at line 3"
    );
    error_helper!("value_amp.over", "Invalid value \"@\" at line 1, column 8");
}
