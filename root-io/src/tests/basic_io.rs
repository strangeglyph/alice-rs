use std::rc::Rc;

use crate::core::*;

#[test]
fn list_of_rules() {
    let s = &[
        64, 0, 1, 130, 255, 255, 255, 255, 84, 76, 105, 115, 116, 0, 64, 0, 1, 116, 0, 5, 0, 1, 0,
        0, 0, 0, 2, 0, 64, 0, 11, 108, 105, 115, 116, 79, 102, 82, 117, 108, 101, 115, 0, 0, 0, 2,
        64, 0, 0, 182, 255, 255, 255, 255, 84, 79, 98, 106, 83, 116, 114, 105, 110, 103, 0, 64, 0,
        0, 163, 0, 1, 0, 1, 0, 0, 0, 0, 3, 0, 0, 0, 150, 116, 121, 112, 101, 61, 114, 101, 97, 100,
        32, 115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 61, 34, 84, 84, 114, 101, 101, 34,
        32, 116, 97, 114, 103, 101, 116, 67, 108, 97, 115, 115, 61, 34, 84, 84, 114, 101, 101, 34,
        32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 91, 45, 49, 54, 93, 34, 32, 115, 111, 117,
        114, 99, 101, 61, 34, 34, 32, 116, 97, 114, 103, 101, 116, 61, 34, 102, 68, 101, 102, 97,
        117, 108, 116, 69, 110, 116, 114, 121, 79, 102, 102, 115, 101, 116, 76, 101, 110, 34, 32,
        99, 111, 100, 101, 61, 34, 123, 32, 102, 68, 101, 102, 97, 117, 108, 116, 69, 110, 116,
        114, 121, 79, 102, 102, 115, 101, 116, 76, 101, 110, 32, 61, 32, 49, 48, 48, 48, 59, 32,
        125, 34, 32, 0, 64, 0, 0, 152, 128, 0, 55, 57, 64, 0, 0, 144, 0, 1, 0, 1, 0, 0, 0, 0, 3, 0,
        0, 0, 131, 116, 121, 112, 101, 61, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101, 67,
        108, 97, 115, 115, 61, 34, 84, 84, 114, 101, 101, 34, 32, 116, 97, 114, 103, 101, 116, 67,
        108, 97, 115, 115, 61, 34, 84, 84, 114, 101, 101, 34, 32, 118, 101, 114, 115, 105, 111,
        110, 61, 34, 91, 45, 49, 56, 93, 34, 32, 115, 111, 117, 114, 99, 101, 61, 34, 34, 32, 116,
        97, 114, 103, 101, 116, 61, 34, 102, 78, 67, 108, 117, 115, 116, 101, 114, 82, 97, 110,
        103, 101, 34, 32, 99, 111, 100, 101, 61, 34, 123, 32, 102, 78, 67, 108, 117, 115, 116, 101,
        114, 82, 97, 110, 103, 101, 32, 61, 32, 48, 59, 32, 125, 34, 32, 0, 0,
    ];
    // Should not be needed; just some dummy
    let context = Context {
        source: Rc::new(LocalDataSource::new("".parse().unwrap())),
        offset: 0,
        s: &[],
    };
    use nom::HexDisplay;
    println!("{}", s.to_hex(16));
    let (_, (_name, obj)) = class_name_and_buffer(s, &context).unwrap();
    println!("{}", obj.to_hex(16));
    let (obj, _ci) = classinfo(obj).unwrap();
    println!("{:?}", _ci);
    println!("{}", obj.to_hex(16));
    // let (_obj, l) = tlist(obj, &context).unwrap();
    // assert_eq!(l.name, "listOfRules");
    // assert_eq!(l.len, 2);
}
