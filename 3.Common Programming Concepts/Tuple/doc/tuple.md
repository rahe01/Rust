# 🦀 Rust Tuple Type Notes

## 🔹 Tuple কি?

Tuple হলো Rust-এর একটি **compound type**, যা একসাথে **বিভিন্ন ধরনের value** এক জায়গায় রাখতে পারে।  

- Tuples-এ **mixed type** থাকতে পারে, যেমন `i32`, `f64`, `u8` একসাথে।  
- Tuple-এ **fixed length** থাকে — একবার tuple বানালে এর size পরিবর্তন করা যায় না।  

---



## 🔹 Tuple তৈরি করা
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
tup হলো একটি tuple

Type annotation: (i32, f64, u8)

Value position-wise:

0 → 500 (i32)

1 → 6.4 (f64)

2 → 1 (u8)






🔹 Tuple থেকে value বের করা (Destructuring)

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {y}");
}
let (x, y, z) = tup; → Tuple ভাঙে এবং আলাদা variable x, y, z বানায়

Output: The value of y is: 6.4

এই পদ্ধতিকে বলা হয় destructuring




🔹 Tuple এ direct access করা


fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);
}



Tuple element-এ access করতে হয় index দিয়ে (x.0, x.1, x.2)

Index শুরু হয় 0 থেকে

Output: 500, 6.4, 1




🔹 Special Tuple: Unit
Empty tuple () → unit type

ব্যবহার হয় যেখানে কোনো value return করা হয় না





fn print_hello() {
    println!("Hello");
} // implicitly returns ()





🔹 Summary
Feature	Description
Mixed types	Tuple একসাথে বিভিন্ন ধরনের value রাখতে পারে
Fixed length	Tuple-এর size change করা যায় না
Access methods	1) Destructuring: let (x, y, z) = tup
2) Index: tup.0, tup.1
Unit type	Empty tuple () → empty value বা return type