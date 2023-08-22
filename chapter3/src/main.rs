fn main() 
{
    //-----------Variables-------------
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //-----------Tuples-------------
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("\nThe values of tup are: {}, {}, {}", five_hundred, six_point_four, one);

    //-----------Functions-------------
    let f = plus_one(5);
    println!("\nThe value of f(5) is: {}\n", f);

    //-----------Loops-------------
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5
    {
        println!("The value of element (while) is: {}", a[index]);
        index += 1;
    }

    println!("");

    for element in a.iter() 
    {
        println!("The value of element (for) is: {}", element);
    }
}

fn plus_one(x: i32) -> i32
{
    x + 1
}
