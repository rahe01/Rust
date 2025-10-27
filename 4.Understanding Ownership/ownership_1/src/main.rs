fn main(){

    let s = "Hello, world!";

    {
        let x = "Inner scope";
        println!("{}", x);
        println!("{}", s);
    }

    println!("{}", s);
    // println!("{}", x);

    {
        let x = "Another inner scope";
        println!("{}", x);
        println!("{}", s);
    }
    


   
}