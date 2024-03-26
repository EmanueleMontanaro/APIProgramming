#[derive(Debug)]
struct Player{
    name: String,
    health: i32,
    level: u8,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point{
    fn mirror(self) -> Self {
        Self{ x: self.y, y:self.x} //This function CONSUMES THE RECEIVER because it produces a new struct of the same type, discarding the previous one
    } //Self is the variable, self is the type.

    fn length(&self) -> f64 { //This function operate on a struct Point without possession nor editing it
        let a = self.x * self.x + self.y * self.y;
        let res = (a as f64).sqrt();
        res
    }

    fn scale(&mut self, s: i32){ //This function operates on a struct Point editing its content.
        self.x *= s;
        self.y *= s;
    }
}

struct Playground (String, u32, u32);
struct Empty; //doesn't allocate memory for this

//Defining enum types with methods
enum HttpResponse {
    Ok,
    NotFound(String),
    InternalError{
        desc: String,
        data: Vec<u8>},
}

fn divide (numerator: f64, denominator: f64) -> Option<f64> { //Defining an Option<T> functino
    if denominator == 0.0 {
        None
    }else {
        Some(numerator / denominator)
    }
}

fn divideBool(x: f64, y: f64) -> Result<f64, &'static str>{
    if y == 0.0 {
        //Returning an error if division by zero is attempted
        Err("Division by zero!")
    } else {
        //Returning the result of the division
        Ok(x / y);
    }
}

fn main() {
    let p1 = Player {name: "Mario".to_string(), health: 25, level: 1};
    let p2 = Player {name: "Giovanni".to_string(), ..p1};
    println!("{:?} \n{:?}", p1, p2);

    let mut f = Playground( "football".to_string(), 90, 45);
    let e = Empty;

    let result = divide (820.0, 15.2);
    match result { //Testing the option type
        Some(value) => println!("Division is {}.", value),
        None => println!("It isn't possible to execute the division."),
    }

    //Attempting division with Result type
    let dividend = 10.0;
    let divisor= 2.0;
    //Handling the result
    match divide(dividend, divisor) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
