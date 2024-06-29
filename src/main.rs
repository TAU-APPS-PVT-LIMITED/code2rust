/*
    main function
*/
fn main() {
    println!("Hello, world!"); //println! is  macro
    println!("My name is {}", "Gaurav Gupta"); //{} substitution placeholder; same lines of python
    let name = "GGupta"; //constant syntax
    println!("My name is {}", name);
    //following is an example of shadowing, constant name being reused.
    let x = 5 + 3; //data type is i32
    let x = x * 2;
    let x = x - 6;
    let x = x / 2;
    println!("The answer is {}", x);
    //let n :i8,i16,i32,i64,i128, isize, usize, u8,u16,u32,u64,u128
    let _unit_literal: () = ();
    let println_result = println!("Generates unit literal as a reuslt");
    println!("{:?}", println_result); //prints () on console
                                      /*
                                      println!("{}", println_result);//generates the following error
                                      the trait `std::fmt::Display` is not implemented for `()`
                                      in format strings you may be able to use `{:?}` (or {:#?} for
                                      pretty-print) instead
                                      */
}
