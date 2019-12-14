// use std::env::args;

// fn main() {
    // let mut args = args(); // needed for impl 1 & 2
    // impl 1
    // loop {
    //     // match args.next() {
    //     //     none => break,
    //     //     Some(arg) => println!("{}", arg),
    //     // }
    //     // -- or
    //     if let Some(arg) = args.next() {
    //         println!("{}", arg);
    //     } else {
    //         break;
    //         // return would work too, but break is nicer
    //         // here, as it is more narrowly scoped
    //     }
    // }

    // impl 2
    // while let Some(arg) => args.next(){
    //     println!("{}", arg)
    // }

    // impl 3
    // for arg in args() {
    //     println!("{}", arg)
    // }
    // A for loop will automate the calling of next(). 
    // It also hides away the fact that there’s some mutable state at play, at least to some extent.
    // This is a powerful concept, and allows a lot of code to end up with a more functional style
// }

// Skipping

// fn main() {
//     let mut args = args();
//     // let _ = args.next(); // droping the first variable, poor design
//     // args implments the itererator trait so we can use the skip 
//     for arg in args().skip(1) { 
//         println!("{}", arg);
//     }
// }

/*
# E1 -----

try to get the below code to compile without running the sompiler first:

use std::env::{args, Args};
use std::iter::Skip;

fn main() {
    let args: Skip<Args> = args().skip(1);
    for arg in args {
        println!("{}", arg);
    }
}
 ----- Answer below
*/
// use std::env::{args, Args};
// use std::iter::Skip;

// fn main() {
//     let args: Args = args();
//     for arg in args.skip(1) {
//         println!("{}", arg);
//     }
// }

// Parsing integers

// use std::env::args;

// fn main() {
//     for arg in args().skip(1) {
//         println!("{:?}", arg.parse::<u32>());
//     }
// }

/* output 
➜ cargo run one 2 three four 5 6 7
   Compiling r03 v0.1.0 (/Users/markobaricevic/code/marbar3778/rgj_dsa/play/r03)
    Finished dev [unoptimized + debuginfo] target(s) in 0.82s
     Running `target/debug/r03 one 2 three four 5 6 7`
Err(ParseIntError { kind: InvalidDigit })
Ok(2)
Err(ParseIntError { kind: InvalidDigit })
Err(ParseIntError { kind: InvalidDigit })
Ok(5)
Ok(6)
Ok(7)
*/

// ----

// Parsing CLI
// #[derive(Debug)]
// struct Frame {
//     width: u32,
//     height: u32,
// }

// #[derive(Debug)]
// enum ParseError {
//     TooFewArgs,
//     TooManyArgs,
//     InvalidInt(String),
// }

// fn parse_args() -> Result<Frame, ParseError> {
//     use self::ParseError::*; // bring variants into our namespace

//     let mut args = std::env::args().skip(1);

//     match args.next() {
//         None => Err(TooFewArgs),
//         Some(width_str) => {
//             match args.next() {
//                 None => Err(TooFewArgs),
//                 Some(height_str) => {
//                     match args.next() {
//                         Some(_) => Err(TooManyArgs),
//                         None => {
//                             match width_str.parse() {
//                                 Err(_) => Err(InvalidInt(width_str)),
//                                 Ok(width) => {
//                                     match height_str.parse() {
//                                         Err(_) => Err(InvalidInt(height_str)),
//                                         Ok(height) => Ok(Frame {
//                                             width,
//                                             height,
//                                         }),
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// V2

// use std::env::Args;

// fn parse_args() -> Result<Frame, ParseError> {

//     let mut args = std::env::args();

//     require_arg(&mut args)?;

//     let width_str = require_arg(&mut args)?;
//     let height_str = require_arg(&mut args)?;
//     require_no_args(&mut args)?;
//     let width = parse_u32(width_str)?;
//     let height = parse_u32(height_str)?;

//     Ok(Frame {
//         width,
//         height,
//     })
// }

// fn require_arg(args: &mut Args) -> Result<String, ParseError> {
//     match args.next() {
//         None => Err(ParseError::TooFewArgs),
//         Some(s) => Ok(s),
//     }
// }

// fn require_no_args(args: &mut Args) -> Result<(), ParseError> {
//     match args.next() {
//         Some(_) => Err(ParseError::TooManyArgs),
//         None => Ok(()),
//     }
// }

// fn parse_u32(s: String) -> Result<u32, ParseError> {
//     match s.parse() {
//         Err(_) => Err(ParseError::InvalidInt(s)),
//         Ok(x) => Ok(x),
//     }
// }

// v3 ----


#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(s) => Ok(s),
        }
    }
    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok(()),
        }
    }
}
fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s)),
        Ok(x) => Ok(x),
    }
}

fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // skip the command name
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;
    args.require_no_args()?;
    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    Ok(Frame { width, height })
}



fn main() {
    println!("{:?}", parse_args())
}
