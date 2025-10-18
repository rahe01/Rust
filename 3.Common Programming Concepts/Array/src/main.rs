fn main() {
    println!("Hello, world!");

    let arr= [1,2,5,2,1,2,1];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


    let a = [3; 5];
// The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
fn main() {
    let a = [1, 2, 3, 4, 5];

    // Invalid index ব্যবহার করা হচ্ছে
    let element = a[10]; // array এর size 5, কিন্তু 10 index নেওয়া হচ্ছে
    println!("This will not print: {}", element);
}



}
