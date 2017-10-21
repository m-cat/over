extern crate over;

use over::obj::Obj;
use over::value::Value;

#[test]
fn basic() {
    let obj = Obj::from_file("tests/test_files/basic.over").unwrap();

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
}
