fn main() {
    let a = 10;

    let guss: i128 = "-34".parse().expect("Not possible");

    println!("{}", guss);

    // Integer Overflow Handle করার ৪টি পদ্ধতি

    // 1. Wrapping (Wrapping around on overflow)

    let x: u8 = 224;
    let y = x.wrapping_add(1);
    println!("Wrapping add: {}", y);




    let a: u128 = 340282366920938463463374607431768211455; // u128 এর max value
    let b = a.wrapping_add(1);
    println!("Wrapping add u128: {}", b); // Output: 0





    // 2. Checked (Returns None on overflow)
    let x: u8 = 255;
    let y = x.checked_add(1); // None রিটার্ন করবে
    println!("Checked add: {:?}", y);






    // 3. Saturating (Clamps to the maximum value on overflow)
    let x: u8 = 255;
    let y = x.saturating_add(1); // 255 রিটার্ন করবে
    println!("Saturating add: {}", y);





    

    // 4. Overflowing (Returns a tuple with the result and a boolean indicating overflow)
    let x: u8 = 255;
    let (y, overflowed) = x.overflowing_add(1); // (0, true) রিটার্ন করবে
    println!("Overflowing add: {}, overflowed: {}", y, overflowed);
}
