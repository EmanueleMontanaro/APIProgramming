fn main() {
    let v: i32 = 123;
    //v isn't reassignable
    let mut w = v;
    w = -5; //w is reassignable

    let y = 1.3278f32; //f32 is a way to define a constant

    let one_million = 1_000_000; //_ can be used as delimiters

    let i: i32 = -12;
    println!("Number {i}");
    let i = "Hi mom!"; //As we can se, it is possible to redefine a variable
    println!("String {i}");

    let mut x = 5;
    x = x + 1;
    {
        let x = x * 2; //This is called "Shadowing" and it can change type (mut cannot)
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}"); //Rust doesn't forget about the previous x

    let y = {
        let x = 3;
        x + 1 //Expression -> The value OF THE LAST EXPRESSION is returned automatically in y
    };
    println!("The value of y is: {y}");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let t: (i32, bool) = (123, false); //Defining a tuple containing a integer and a boolean
    let mut u = (3.14, 2.71); //u is a reassignable tuple containing two double
    let i = t.0; //i contains 123
    u.1 = 0.0; //u is now (3.14, 0.0)

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let v = 32;
    let p = &v;
    let pp = &p;
    let ppp = &pp;
    let str = ppp.to_string();
    println!("{str}");

    let mut i = 32;
    let r = &i;
    println!("{}",*r);
    /*i = i+1; -> This instruction is NOT VALID
    println!("{}",*r);*/

    let r = &mut i;
    // println!("{}",i); This isn't allowed because there is a RefMut alive.
    *r = *r+1;
    println!("{}",*r);
}
