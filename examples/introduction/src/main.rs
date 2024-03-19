use std::env::args;
fn makeBox(a: i32) -> Box<(i32,i32)> { //The arrow is the way to tell the program the type of return value we will have.
    let r = Box::new( (a,1) );
    return r;
}

struct S(i32);

impl S {
    fn display (&self) {
        println!("Sono S e contengo {} @ {:p}", self.0, self);
    }
}

impl Drop for S {
    fn drop(&mut self){
        println!("Dropping S{} @ {:p}", self.0, self);
    }
}

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

    let b = Box::new(v); //Creating a Box, a pointer with possession which then has the capability of deallocating memory.
    //We can access using *b

    fn useBox(){
        let i = 4;
        let mut b = Box::new( (5,2) ); // (5,2) is a tuple allocated in the heap, the pointer *b is in the stack
        (*b).1 = 7; //Accessing a single element of the box.
        println!("{:?}", *b); // (5,7)
        println!("{:?}", b); // (5,7)
    } //When this scope ends, i and b are freed up. Freeing b means also releasing it's content thanks to the Drop trait.

    fn printingTrait(){
        let x = &42;
        println!("{x:p}");
    }

    let b = makeBox(5); //This example shows how to make a variable live longer than its creator, this instruction in particular
    //consists in a transfer of the address from r (makeBox) to b, and with that the possession is passed too.ù
    //The difference with other programming languages is that in RUST through the Drop trait the memory is deallocated later on, so there is no leakage.
    let x = b.0 + b.1;

    //Copy vs Clone
    let x:u8 = 123;
    let y = x;

    println!("x={}, y={}", x, y);
    let v: Vec<u8> = vec![1, 2, 3];
    let w = v; //v becomes unusable
    println!("w={:?}",w);

    let s1 = S(1);
    let s2 = Box::new(S(2));

    s1.display();
    s2.display(); //It is noticeable how the struct is saved in the stack and the Box is saved in the heap (look at addresses)

    //Array
    let a: [i32; 4] = [1, 2, 3, 4];
    let b = [0; 5]; //5 elements all zero
    let l = b.len();
    let e = a[3]; //index has to be unsigned, if an 'Index out of Bounds' happens the compiler tells us if it happens during compiling.
    //If it happens during runtime PANIC happens, and the program terminates.

    //Slicing
    let s1: &[i32] = &a; //s1 contains 1, 2, 3, 4
    let s2 = &a[0..2]; //s2 contains 1, 2
    let s3 = &a[2..]; //s3 contains 3, 4

    //Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    let mut s = &mut v;
    s[1] = 8;

    //Strings
    let s = "Hello, "; //static memory allocation (stack), immutable
    let mut p = String::new();
    p.push_str(s);
    p.push_str(" World!"); //whole String allocated in heap.
    let s1 = String::from("some text"); //Other way to define a string
    let s2 = "some text".to_string(); //Another one (cool)

    let s3 = s2.as_str(); //To retrieve a str type object.

    //Using args
    let args: Vec<String> = args().skip(1).collect();
    if args.len() > 0 {
        println!("{}", args[0]);
    }
}
