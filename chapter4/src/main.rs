fn main() 
{
    let mut s = String::from("Hello");
    s.push_str(", World!"); // push_str() appends a literal to a String

    println!("{}", s);

    // ------------ Ways variables and data interact: "MOVE variables"
    let s1 = String::from("Hello");
    let _s2 = s1;
    //println!("{}, world!", s1); // s1 is no longer valid

    /* NOTE: String variable s1 is no longer valid because it was moved to s2. This occur because Rust calls the drop function automatically to delete de s1 variable in order to don't affect the memory deleting two times the same direction in which the pointer is. */

    // ------------- Ways variables and data interact: "CLONE variables"

    let c1 = String::from("Arroz");
    let c2 = c1.clone();

    println!("c1 = {}, c2 = {}", c1, c2);

    /* NOTE: With the clone() method is it copied the TOTALLY of the variable, even the data, with that, another variable y created in the memory, but this is expensive in the memory */

    // ------------- Ownership and functions
    /* NOTE: When we pass a variable to a function, the ownership of the variable is passed to the function. And the variable passed is moved if is a String. Is copied if the variable is Copy type (like integers, booleans and so on...) */

    let st = String::from("Leche"); // s comes into scope 

    takes_ownership(st); // Value of s moves into the function, whereby the s variable is drop from the program.

    let x = 5; // x comes into scope 

    makes_copy(x); // x would move into the function, but type i32 is Copy type so it's okay to still and can be used again until it is out of scope.

    return_values_and_scope();
    dangling_reference();
    slices();
}

fn takes_ownership(some_fucking_string: String) 
{
    // some_fucking_string comes into scope
    println!("{}", some_fucking_string);
}   // Here some_fucking_string is out of scope and drop function is called.

fn makes_copy(some_fucking_integer: i32) 
{
    // Here is the same thing with the integer, I've already get the idea.
    println!("{}", some_fucking_integer);
}

fn return_values_and_scope() 
{
    // Returning values can also transfer ownership.

    let _r1 = gives_ownership(); // gives_ownership moves its return value into r1

    let r2 = String::from("Hello"); // r2 comes into scope

    let _r3 = takes_and_gives_back(r2); // r2 is moved into takes_and_gives_back, which also moves its return value into r3

    // We can return multiple values using a tuple
    let another_string = String::from("Hello");
    let (r4, len) = calculate_length(another_string);

    println!("The length of '{}' (without reference) is {}.", r4, len);

    // --------------- References and borrowing
    /* NOTE: References are like pointers, but they don't have to point to a valid object. Instead, they point to a particular location in memory that contains some data. */

    let s = String::from("Hello");
    let len = calculate_length_reference(&s);
    println!("The length of '{}' (with reference) is {}.", s, len);

    /* NOTE 2: We cannot modify the references borrowed by the functions since they are immutable by default like the variables. To solve this, we have to make the variable mutable and also the reference borrowed by the function */
    let mut s1 = String::from("Hello");

    change_reference(&mut s1);
}

fn gives_ownership() -> String 
{
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // Value returned
}

fn takes_and_gives_back(a_string: String) -> String
{
    a_string
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize
{
    s.len()
}

fn change_reference(some_string: &mut String) 
{
    some_string.push_str(", World"); // push_str() appends a literal to a String

    println!("{}", some_string); // Value returned
}

// ------------- Dangling Reference --------------------
fn dangling_reference() 
{
    let _reference_to_nothing = dangle();
}

fn dangle() -> String
{
    // Dangle -> &String
    // let s = String::from("Milú");
    // &s

    // No dangle -> String
    let s = String::from("Milú");
    s
}

// -------------- Slices ----------------
fn slices()
{
    let mut s = String::from("Arroz con pollo");
    let _word = first_word(&s);
    s.clear();

    string_slices();
}

// -> First word without string slices
// fn first_word(s: &String) -> usize
// {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate()
//     {
//         if item == b' '
//         {
//             return i;
//         }
//     }

//     s.len()
// }

// -> First word with string slices
fn first_word(s: &String) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices()
{
    let s = String::from("Arroz con pollo");
    let arroz = &s[0..5];
    let pollo = &s[10..15];

    println!("Las palabras extraídas son {} y {}", arroz, pollo);
}
//107 chapter 5