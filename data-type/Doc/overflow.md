# Rust Integer Overflow Handling Examples

এই প্রোজেক্টে আমরা Rust এ **Integer Overflow Handle করার ৪টি পদ্ধতি** দেখব। উদাহরণ হিসেবে আমরা `u8` এবং `u128` টাইপ ব্যবহার করেছি।  

---

## Table of Contents
1. [Parsing String to Integer](#parsing-string-to-integer)
2. [1. Wrapping](#1-wrapping)
3. [2. Checked](#2-checked)
4. [3. Saturating](#3-saturating)
5. [4. Overflowing](#4-overflowing)

---

## Parsing String to Integer

Rust এ যখন আমরা String থেকে Number এ রূপান্তর করি, তখন `parse()` ব্যবহার করি।  
যদি String টি integer এ রূপান্তরযোগ্য না হয়, তাহলে `expect()` দিয়ে error handle করা যায়।  

```rust
let guss: i128 = "-34".parse().expect("Not possible");
println!("{}", guss);
Output: -34

যদি String number না হয় → Not possible message দেখাবে।

1. Wrapping
Wrapping method এ overflow হলে wrap-around behavior হয়।

অর্থাৎ max value পার হলে শুরু থেকে শুরু হয়।

rust
Copy code
let x: u8 = 224;
let y = x.wrapping_add(1);
println!("Wrapping add: {}", y); // Output: 225

let a: u128 = 340282366920938463463374607431768211455; // max u128
let b = a.wrapping_add(1);
println!("Wrapping add u128: {}", b); // Output: 0
✅ Key Point:
Overflow হলে panic হয় না, wrap-around হয়।

2. Checked
Checked method overflow হলে None রিটার্ন করে।

Overflow না হলে Some(value) রিটার্ন হয়।

rust
Copy code
let x: u8 = 255;
let y = x.checked_add(1);
println!("Checked add: {:?}", y); // Output: None
✅ Key Point:

Safe method, runtime panic হয় না।

Option type রিটার্ন করে → Some(value) বা None।

3. Saturating
Saturating method overflow হলে সর্বোচ্চ (max) বা সর্বনিম্ন (min) মানে আটকে থাকে।

rust
Copy code
let x: u8 = 255;
let y = x.saturating_add(1);
println!("Saturating add: {}", y); // Output: 255
✅ Key Point:

Wrap-around হয় না।

Overflow হলে max/min এ value স্থিত থাকে।

4. Overflowing
Overflowing method tuple রিটার্ন করে (value, overflowed)।

Value = wrap-around result

Overflowed = true/false

rust
Copy code
let x: u8 = 255;
let (y, overflowed) = x.overflowing_add(1);
println!("Overflowing add: {}, overflowed: {}", y, overflowed); // Output: 0, true
✅ Key Point:

Wrap-around সহ overflow detect করতে সাহায্য করে।

Summary Table
Method	Overflow Behavior	Example Output (u8)
wrapping_add	Wrap-around	255 + 1 → 0
checked_add	None on overflow	255 + 1 → None
saturating_add	Clamp to max	255 + 1 → 255
overflowing_add	Wrap-around + boolean	255 + 1 → (0, true)