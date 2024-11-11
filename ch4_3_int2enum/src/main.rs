// use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

// #[derive(FromPrimitive)]
// enum MyEnum {
//     A = 1,
//     B,
//     C,
// }

// fn main() {
//     let x = 2;

//     match FromPrimitive::from_i32(x) {
//         Some(MyEnum::A) => println!("Got A"),
//         Some(MyEnum::B) => println!("Got B"),
//         Some(MyEnum::C) => println!("Got C"),
//         None            => println!("Couldn't convert {}", x),
//     }
// }

// use std::convert::TryFrom;

// impl TryFrom<i32> for MyEnum {
//     type Error = ();

//     fn try_from(v: i32) -> Result<Self, Self::Error> {
//         match v {
//             x if x == MyEnum::A as i32 => Ok(MyEnum::A),
//             x if x == MyEnum::B as i32 => Ok(MyEnum::B),
//             x if x == MyEnum::C as i32 => Ok(MyEnum::C),
//             _ => Err(()),
//         }
//     }
// }

use std::convert::TryInto;

fn main() {
    let x = MyEnum::C as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}

#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    enum MyEnum {
        A = 1,
        B,
        C,
    }
}


