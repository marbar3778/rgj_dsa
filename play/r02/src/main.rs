// compiler pragme, for some traits the compiler can already provide a impl if you ask
#[derive(Debug)] 
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a foobar: {:?}", self)
    }
}

fn uses_foobar(foobar: Foobar) {
    // {:?} use debug trait to display this
    println!("I consumed a Foobar: {:?}", foobar); 
}

// fn main() {
//     let x = Foobar(1);
//     println!("Before uses_foobar");
//     uses_foobar(x);
//     println!("After uses_foobar");
// }

// Output
// Before uses_foobar
// I consumed a Foobar: Foobar(1)
// Dropping a Foobar: Foobar(1)
// After uses_foobar

// ---- E1

fn main() {
    println!("Before x");
    let _x = Foobar(1);
    println!("After x");
    {
        println!("Before y");
        let _y = Foobar(2);
        println!("After y");
    }
    println!("End of main");
}

// Before x
// After x
// Before y
// After y
// Dropping a Foobar: Foobar(2)
// End of main
// Dropping a Foobar: Foobar(1)

//  ---Borrows/references (immutable)

// #[derive(Debug)]
// struct Foobar(i32);

// impl Drop for Foobar {
//     fn drop(&mut self) {
//         println!("Dropping a Foobar: {:?}", self);
//     }
// }

// fn uses_foobar(foobar: &Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

// fn main() {
//     let x = Foobar(1);
//     println!("Before uses_foobar");
//     uses_foobar(&x); // reference here
//     uses_foobar(&x); // reference here
//     println!("After uses_foobar");
// }


// ---- E2

// impl Foobar {
//     fn use_it(&self) {
//         println!("I consumed a Foobar: {:?}", self)
//     }
// }

// fn main() {
//     let x = Foobar(1);
//     println!("Before uses_foobar");
//     x.use_it();
//     x.use_it();
//     println!("After uses_foobar");
// }

// -- Multiple Live References

// fn main() {
//     let x: Foobar = Foobar(1);
//     let y: &Foobar = &x; // borrowing x here
//     println!("Before uses_foobar");
//     uses_foobar(&x);
//     std::mem::drop(x); // dropping x here
//     uses_foobar(y); // borrow later used here
//     println!("After uses_foobar");
// }

// output
// error[E0505]: cannot move out of `x` because it is borrowed
//   --> src/main.rs:98:20
//    |
// 95 |     let y: &Foobar = &x;
//    |                      -- borrow of `x` occurs here
// ...
// 98 |     std::mem::drop(x);
//    |                    ^ move out of `x` occurs here
// 99 |     uses_foobar(y);
//    |                 - borrow later used here

// Challenge

// Try to guess which lines in the code below will trigger a compilation error:

// #[derive(Debug)]
// struct Foobar(i32);

// fn main() {
//     let x = Foobar(1);

//     foo(x);
//     foo(x); // comp error - value used after move

//     let mut y = Foobar(2);

//     bar(&y);
//     bar(&y); // comp error - incorrect, its being passed by reference

//     let z = &mut y;
//     bar(&y); // comp error - immutable error occurs here
//     baz(&mut y);// comp error - second mutable borrow here
//     baz(z);
// }

// // move
// fn foo(_foobar: Foobar) {}

// // read only reference
// fn bar(_foobar: &Foobar) {}

// // mutable reference
// fn baz(_foobar: &mut Foobar) {}

// -- copy trait

// Copy - Rust has a special trait, Copy, which indicates that a type is so cheap that it can automatically be passed-by-value. 
// That’s exactly what happens with i32. You can explicitly do this with the Clone trait if desired

// #[derive(Debug, Clone)] // added clone trait
// struct Foobar(i32);

// impl Drop for Foobar {
//     fn drop(&mut self) {
//         println!("Dropping: {:?}", self);
//     }
// }

// fn uses_foobar(foobar: Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

// fn main() {
//     let x = Foobar(1);
//     uses_foobar(x.clone());
//     uses_foobar(x);
// }

// challenge e3 -----

// #[derive(Debug, Clone, Copy)]
// struct Foobar(i32);

// fn uses_foobar(foobar: Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

// fn main() {
//     let x = Foobar(1);
//     uses_foobar(x);
//     uses_foobar(x);
// }

// Lifetimes:
// The term that goes along most with ownership is lifetimes. Every value needs to be owned, and its owned for a certainly lifetime until it’s dropped. So far, everything we’ve looked at has involved implicit lifetimes.
//  However, as code gets more sophisticated, we need to be more explicit about these lifetimes. 


// Add an implementation of the double function to get this code to compile, run, and output the number 2:

// #[derive(Debug)]
// struct Foobar(i32);

// fn double(foobar: Foobar) -> Foobar {
//     Foobar(foobar.0 * 2)
// }

// // or 

// // fn double(mut foobar: Foobar) -> Foobar {
// //     foobar.0 *= 2;
// //     foobar
// // }

// fn main() {
//     let x: Foobar = Foobar(1);
//     let y: Foobar = double(x);
//     println!("{}", y.0);
// }

// Sturcts and enums 

// structs are known as product types, which means they contain multiple values. Rust also provides enums, which are sum types, or tagged unions. These are alternatives, where you select one of the options. A simple enum would be:

// enum Color {
//     Red,
//     Blue,
//     Green,
// }
