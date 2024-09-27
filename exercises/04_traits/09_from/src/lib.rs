// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<i16> for WrappingU32 {
    fn from(value: i16) -> Self {
        if value < 0 {
            panic!("input value must be >= 0")
        }
        WrappingU32 {
            value: u32::from(value)
        }
    }
}
impl From<i32> for WrappingU32 {
    fn from(value: i32) -> Self {
        if value < 0 {
            panic!("input value must be >= 0")
        }
        WrappingU32 {
            value: u32::from(value)
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
