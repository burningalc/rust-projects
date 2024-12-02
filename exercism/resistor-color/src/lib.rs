use int_enum::IntEnum;
use enum_iterator::{Sequence, all};

#[repr(u32)] // https://doc.rust-lang.org/reference/type-layout.html#reprattribute
#[derive(Debug, PartialEq, Eq, Sequence, IntEnum, Clone, Copy)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    // 1. if we are sure that value will not be out of range
    // format!("{:?}", ResistorColor::from_int(value).unwrap())

    // 2. else we will need to handle error like this
    match ResistorColor::from_int(value) {
        Ok(val) => format!("{:?}", val),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
