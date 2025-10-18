fn main() {


    let mark = 34.5;

    let markk: f32 = 243.4;

    println!("{}", mark);
    println!("{}", markk);



    
    let a: f32 = 3.141592653589793;
    let b: f64 = 3.141592653589793;

    println!("f32 value: {:.15}", a);
    println!("f64 value: {:.15}", b);


    



    NumOperation();


    let t = true;
    let f : bool = false;













}




fn NumOperation(){
    
    // addition
    let sum = 5 + 10;
    println!("{}" , sum);

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

}