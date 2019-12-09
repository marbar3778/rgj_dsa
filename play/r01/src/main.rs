// struct Person {
//     name: String,
//     age: u32,
// }

// impl std::fmt::Display for Person {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         write!(fmt, "{} ({} years old)", self.name, self.age)
//     }
// }

// fn main() {
//     let alice = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };
//     println!("Person: {}", alice );
// }

// ----- numeric types

// fn main() {
//     let mut i = 1;

//     loop {
//         println!("i == {}", i);
//         if i >= 10 {
//             break;
//         } else {
//             i += 1;
//         }
//     }
// }

// fn main() {
//     for i in 1..11 {
//         println!("i == {}", i);
//     }
// }

// ------ Fizzbuzz

fn main() {
    // for i in 1..100 {
    //     if i % 3 == 0 &&  i % 5 == 0 {
    //         println!("fizzbuzz {} ", i)
    //     } else if i % 3 == 0 {
    //         println!("fizz, {}", i)
    //     } else if i % 5 == 0 {
    //         println!("buzz")
    //     }
    // }

    // better solution

    for i in 1..100 {
        match( i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (false, false) => println!("{}", i),
        }
    }
}