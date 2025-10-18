fn main() {
    let mut age = 25;
    println!("I am {} years old.", age);

    age = 41;
    println!("I am {} years old.", age);

    const PI: f64 = 3.14;

    println!("The value of PI is {}.", PI);

    let apple = 10;

    let apple = apple + 5;
    println!("I have {} apples.", apple);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
