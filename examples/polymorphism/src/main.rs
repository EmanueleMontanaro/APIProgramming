/*
*** Defining a new TRAIT ***

use SomeNamespace::SomeTrait;

trait SomeTrait { fn someOperation(&mut self) -> SomeResult;}

impl SomeTrait for SomeType {...}
*/

trait Salutabile {
    fn saluta(&self);
}

//Usage of a trait

struct Persona {nome: String }
struct Azienda{ nome: String, numero: i32 }

impl Salutabile for Persona {
    fn saluta(&self) {
        println!("Ciao, sono {}!",self.nome);
    }
}
impl Salutabile for Azienda {
    fn saluta(&self) {
        println!("Salve {} il tuo numero è {}!", self.nome,self.numero);
    }
}

trait HasArea {
    fn get_area(&self) -> f64;
}

struct Circle { radius: f64 }
impl HasArea for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Rectangle { width: f64, height: f64,}
impl HasArea for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

// Difference between derive and implement for a trait

#[derive(Default)]
struct Identity{
    name: String,
    age: u32,
}

trait Inizializzabile { //Defining a custom Default (USELESS IN THIS CASE)
    fn inizializza() -> Self;
}

struct Punto {
    x: i32,
    y: i32,
}

impl Inizializzabile for Punto {
    fn inizializza() -> Self {
        Punto {x: 0, y: 0}
    }
}

// Defining and using a trait for multiple types
trait Convertibile {
    type Output;
    fn converti(&self) -> Self::Output;
}

struct NumeroIntero { valore: i32 }
impl Convertibile for NumeroIntero {
    type Output = f64;
    fn converti(&self) -> Self::Output {
        self.valore as f64
    }
}

struct NumeroReale { valore: f64 }
impl Convertibile for NumeroReale {
    type Output = i32;
    fn converti(&self) -> Self::Output {
        self.valore as i32
    }
}

//Defining a DEFAULT method (we can redefine a method and not redefine another one to use the default one
trait Moltiplicabile {
    fn moltiplica(&self, factor: i32) -> i32;
    fn moltiplica_per_due(&self) -> i32 {
        self.moltiplica(2);
    }
}

struct Numero {valore: i32}

impl Moltiplicabile for Numero {
    fn moltiplica(&self, factor: i32) -> i32 {
        self.valore * factor
    }
}

//Implementing supertrait in subtrait

trait Supertrait {
    fn Sup(&self) {println!("In super");}
    fn g(&self) {}
}

trait Subtrait: Supertrait {
    fn Sub(&self) {println!("In sub");}
    fn g(&self) {}
}

struct SuperSubType;
impl Supertrait for SuperSubType {}
impl Subtrait for SuperSubType {}

//Using &dyn double-pointer to deal with trait-objects

trait Shape {
    fn area(&self) -> f64; }
trait Drawable: Shape { fn draw(&self);}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {self.width * self.height}
}
impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {}.",
                 self.width, self.height);
    }
}
struct Square { side: f64, }
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}.", self.side);
    }
}

fn draw_shape(shape: &dyn Drawable) { //This function wants a type which implements the Drawable trait
    shape.draw();
    println!("Area: {}", shape.area());
}

use std::ops::Deref;
#[derive(Debug)]
struct CustomStruct {
    number: i32,
    boxed_value: Box<i32>,
}

impl Deref for CustomStruct {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.number
    }
}

fn main() {
    let persona = Persona { nome: String::from("Mario") };
    let azienda = Azienda { nome: String::from("ItalComputer"), numero: 1247 };
    persona.saluta();
    azienda.saluta();

    let circle = Circle {radius: 3.0};
    let rectangle = Rectangle {width: 4.0, height: 5.0};
    println!("L'area del cerchio è: {}", circle.get_area());
    println!("L'area del rettangolo è: {}", rectangle.get_area());

    let default_person: Identity = Default::default();
    //deriving Default for my struct applies Default for every type present there
    println!("Name: {}", default_person.name); //Prints: Name:
    println!("Age: {}", default_person.age); //Prints: Age: 0

    let punto = Punto::inizializza();
    println!("Punto inizializzato : ({}, {})", punto.x, punto.y);

    let numero = NumeroIntero { valore: 42 };
    let valore_convertito: f64 = numero.converti();
    println!("Valore convertito: {}", valore_convertito);

    let numero = NumeroReale { valore: 42.3 };
    let valore_convertito: i32 = numero.converti();
    println!("Valore convertito: {}", valore_convertito);

    let s = SuperSubType;
    s.Sup();
    s.Sub();
    <SuperSubType as Supertrait>::g(&s); //To define which of the two trait to use

    let rectangle = Rectangle {width:3.0,height: 4.0};
    let square = Square { side: 2.0};
    draw_shape(&rectangle);
    draw_shape(&square);

    let boxed_value = Box::new(42);
    let custom_struct = CustomStruct { number: 10, boxed_value };
    let mut custom_struct_ref = &custom_struct;
    println!("Dereferencing custom_struct: {:?}", *custom_struct_ref);
    // Prints: Dereferencing custom_struct: 10
    println!("Dereferencing boxed_value: {:?}", *custom_struct_ref.boxed_value);
    // Prints: Dereferencing boxed_value: 42
}