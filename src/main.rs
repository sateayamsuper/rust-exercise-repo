///////
fn main () {
    println!("Hello World!");
}
    // Integer example
    let two = 2;
    println!("Integer: {}", two);
    // String literal example
    let hello = "hello";
    println!("String: {}", hello);
    // Character example 
    let j = 'j';
    println!("Character: {}", j);
    // Float example
    let my_half = 0.5;
    println!("Float; {}", my_half);
    // Mutable variable example 
    let mut my_name = "Seta";
    println!("Original name: {}", my_name);
    my_name = "Seta"; // We can change this because it's mutable
    println!("New name: {}", my_name);
    // Boolean example 
    let quit_program = false;
    println!("Should quit: {}", quit_program);
    // Variable assignment example 
    let your_half = my_half;
    println!("Your half: {}", your_half);

fn add(a: i32 b: i32) -> i32  { // i = integer, 32 = 32 bits, a and b are for labelling boxes
    a + b
}
let x = add (1, 1);
let y = add (3,0);
let z = add(x, 1);

fn print_many(msg: &str, count: i32) { // data type
    for i in 0..count {
        println!("{}", msg);
    }
}
enum Mouse { // enum -> enumeration (can only choose one)
    LeftClick,
    RightClick,
    MiddleClick,
}
///////
fn main () { // function
    print_many("Hello!", 3);

let my_click = Mouse::RightClick; //var 
let numbers: Vec<i32> = vec![1, 2, 3]; //vec! = vector (a list of items)
let letters: Vec<char> = vec!['a', 'b'];
let clicks: Vec<Mouse> = vec![
    Mouse::LeftClick,
    Mouse::LeftClick,
    Mouse::RightClick,
];
match my_click { //choose from the var
    Mouse::LeftClick => println!("You clicked the left button!"),
    Mouse::RightClick => println!("You clicked the right button!"),
    Mouse::MiddleClick => println!("You clicked the middle button!"),
        }
}   
/////
fn main () { // if else
    let a = 250;
if a > 99 {
    if a > 200 {
        println!("Huge number");
    } else {
        println!("Big number");
    }
} else {
    println!("Small number");
    }
}

fn main () { // loop
    let mut a = 0; // mute -> allows the changeable
    loop {
        if a == 5 {
            break; // if a reaches 5, stop the loop
        }
        println!("{:?}", a); // tanya kak wira
        a = a + 1; // loop
    }
}
//////
fn main () { // match with boolean
    let some_bool = true;
    match some_bool {
        true => println!("It's true!"),
        false => println!("It's false!"),
    }
}
///////
fn main () { // match with int
    let some_int = 3;
    match some_int {
        1 => println!("It's 1!"),
        2 => println!("It's 2!"),
        3 => println!("It's 3!"),
        _ => println!("It's something else!"),
    }
}
trait Iterator { // trait -> list of rules, iterator -> give things one at a time
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
/////
// Basic IntoIterator example
struct Friends {
    names: Vec<String>,
}

impl IntoIterator for Friends {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

// Custom Iterator example with Color
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

impl<'a> Iterator for ColorIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };

        self.position += 1;
        next
    }
}

// Implementing IntoIterator for borrowed Color
impl<'a> IntoIterator for &'a Color {
    type Item = u8;
    type IntoIter = ColorIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ColorIter {
            color: &self,
            position: 0,
        }
    }
}

// Convention for exposing iteration methods
impl Color {
    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> + '_ {
        // Note: This is a simplified example. Implementing mutable iteration
        // is more complex and often requires unsafe code or collecting references
        vec![&mut self.r, &mut self.g, &mut self.b].into_iter()
    }
}

fn main() {
    // Example usage
    let color = Color { r: 255, g: 128, b: 64 };
    
    // Using IntoIterator
    for component in &color {
        println!("Color component: {}", component);
    }

    // Using conventional iter() method
    for component in color.iter() {
        println!("Color component: {}", component);
    }
}