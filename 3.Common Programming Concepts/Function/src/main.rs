fn main() {
    println!("Hello, world!");

    another_function(20, "Iphone");
    let y = 20;
    let x = returnvalue(y);
    println!("{}" , x);
}


fn another_function(x:i32 , product_name: &str){
    println!("Another function");
    println!("This is a parameter {} {}" , x, product_name);




    let y ={
        let  x = 7;
        x+1

    };
    println!("{}" , y);
}




fn returnvalue(x:i32) -> i32{
   x+ 375634
}