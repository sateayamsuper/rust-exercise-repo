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
    type Item; // type -> give things one at a time
    fn next(&mut self) -> Option<Self::Item>;
}
/////
// Basic IntoIterator example
struct Friends { // struct -> container that held informations
	names: Vec<String>, // Vec -> vector (vector is a list of things)
}

impl IntoIterator for Friends { // implement IntoIterator for Friends
	type Item = String;
	type IntoIter = 
	std::vec::IntoIter<Self::Item>; // self -> currenct object that are being worked on

	fn into_iter(self) -> Self::IntoIter {
		self.names.into_iter()
	}
}
// Borrow 
impl<'a> IntoIterator for &'a Friends { // borrow -> give things one at a time
	type Item = &'a String;
	type IntoIter = std::slice::Iter<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.names.iter()
	}
}
// Mutable Borrow 
impl<'a> IntoIterator for &'a mut Friends { // mutable borrow -> give things one at a time
	type Item = &'a mut String;
	type IntoIter = std::slice::IterMut<'a, String>;

	fn into_iter(self) -> Self::IntoIter {
		self.names.iter_mut()
	}
}
// Iteration 
let names = vec![
	"Albert".to_owned(),
	"Sara".to_owned(),
];

let mut friends = Friends { names };

for f in friends {
	println!("{:?}", f);
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

impl<'a> Iterator for ColorIter<'a> { // Impl Iterator - Borrow
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

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
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
// Summary (Working Code)

// Defining the Color struct
struct Color {
	r: u8,
	g: u8, // Unsigned becuase rgb naturally cannot be negative
	b: u8
}

// Define ColorIteration struct
struct ColorIter<'a> {
	color: &'a Color,
	position: u8,
}

// Implement Iterator for ColorIter
impl<'a> Iterator for ColorIter<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		let next = match self.position {
			0 => Some (self.color.r),
			1 => Some (self.color.g),
			2 => Some (self.color.b),
			_ => None,
			
		};
		self.position += 1;
		next
	}
}
// Implement IntoIterator for Color reference
impl<'a> IntoIterator for &'a Color {
	type Item = u8;
	type IntoIter = ColorIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		ColorIter {
			color: self,
			position: 0
		}
	}
}

// Implement methods for Color
impl Color {
	fn iter (&self) -> impl Iterator<Item = u8> + '_ {
		self.into_iter()
	}
	fn iter_mut(&mut self) -> impl Iterator <Item = &mut u8> + '_ {
		vec![&mut self.r, &mut self.g, &mut self.b].into_iter()
	}
}

fn main() {
	// Creating a new color 
	let mut color = Color {
		r: 255,
		g: 128,
		b: 64,
	};
	// Method 1: Using IntoIterator 
	println!("Using IntoIterator:");
	for component in &color {
		println!("Color component: {}", component);
	}
	// Method 2: Using iter() method
	println!("\nUsing iter() method:");
	for component in color.iter() {
		println!("Color component: {}", component);
	}
	// Method 3: Using iter_mut() method
	println!("\nUsing iter_mut() method:");
	for component in color.iter_mut() {
		println!("Color component: {}", component);
	}

	// Creating another color
	let mut another_color = Color {
		r: 10,
		g: 20,
		b: 30,
	};
	
	println!("\nIterating another color:");
	for c in &another_color {
		println!("Component value: {}", c);
	}

	// Example of mutable iteration
	println!("\nModyfing color components:");
	for component in another_color.iter_mut() {
		*component += 5; // Increase each component by 5
	}
	// Printing the modified color components
	println!("\nModified color components:");
	for c in &another_color {
		println!(" New component value: {}", c);
	}
}

// ENUMS (Enumeration)
// - Used to define a type that can be one of several variants
// - Each variant is distinct and can be matched against
enum Light {
    Bright,    // First variant
    Dull,      // Second variant
}

// FUNCTIONS
// - Takes ownership of the Light parameter
// - 'light' parameter is a label for the input value
fn display_light(light: Light) {
    // MATCH Expression
    // - Like a switch statement but more powerful
    // - Must handle all possible variants
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

// MAIN Function
// - Entry point of the program
fn main() {
    // VARIABLE ASSIGNMENT
    // - Creates new variable 'dull' of type Light
    // - Uses Light::Dull variant
    let dull = Light::Dull;

    // FUNCTION CALLS
    // - First call moves ownership of 'dull' to display_light
    display_light(dull);
    // - Second call will fail because 'dull' was moved
    display_light(dull);  // Error: use of moved value 'dull'
}
// 1.4-slide.md
// // Enumeration
// Additional: Enums are used to define a type that can be one of several variants
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Additional: Pattern matching function that safely handles all enum variants
fn which_way(go: Direction) -> &'static str { // &'static str = string slice
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

// Structure
// Additional: Structs group related data fields together
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    // Additional: Instance creation with field initialization syntax
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    // Additional: Field access using dot notation
    let tall = my_box.height;
    println!("the box is {:?} units tall", tall);
}

// Tuples
// Additional: Simple enum with single variant
enum Access {
    Full
}

// Additional: Function returning a tuple of three integers
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    // Additional: Two ways to extract tuple values
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();  // Destructuring syntax
    println!("{:?}, {:?}", x, numbers.0); // 1
    println!("{:?}, {:?}", y, numbers.1); // 2
    println!("{:?}, {:?}", z, numbers.2); // 3

    // Additional: Tuple can hold different types
    let (employee, access) = ("Jake", Access::Full);
}

// Expressions
// Additional: Verbose if expression that could be simplified
let my_num = 3;
let is_lt_5 = if my_num < 5 {
    true
} else {
    false
};

// Additional: Simplified boolean expression
let is_lt_5 = my_num < 5;

// Additional: Match expression with default case (_)
let my_num = 3;
let message = match my_num {
    1 => "hello",
    _ => "goodbye"
};

// Additional: Menu items enum for a food ordering system
enum Menu {
    Burger,
    Fries,
    Drink,
}

// Additional: Complex match expression with nested logic
let paid = true;
let item = Menu::Drink;
let drink_type = water;
let order_placed = match item {
    Menu::Drink => {
        if drink_type == "water" {
            true
        } else {
            false
        }
    },
    _ => true,
};

// Option
// example 1
// Additional: Standard Option enum definition with generic type
enum Option<T> {
    Some(T),
    None,
}

// Additional: Customer struct with optional age field
struct Customer {
    age: Option<i32>,
    email: String,
}

// Additional: Creating customer with known age
let mark = Customer {
    age: Some(22),
    email: "mark@example.com".to_owned(),
};

// Additional: Creating customer with unknown age
let becky = Customer {
    age: None,
    email: "becky@example.com".to_owned()
};

// Additional: Safely handling Option with match
match becky.age {
    Some(age) => println!("customer is {} years old", age),
    None => println!("customer age not provided"),    
}

// example 2
// Additional: Struct for representing items in grocery inventory 
struct GroceryItem {
    name: String,
    qty: i32,
}

// Additional: Function that demonstrates Option for handling missing values
fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned(), qty: 4 },
        GroceryItem { name: "eggs".to_owned(), qty: 12 },
        GroceryItem { name: "bread".to_owned(), qty: 1 },
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }

    None
}

// Result
// Additional: Standard Result enum for error handling
Enum Result<T, E> { // Definition
    Ok(T),
    Err(E),
}

// Additional: Example of Result usage with sound system
struct SoundData {
    name: String,
}

// Additional: Implementation block with constructor
impl SoundData {
    fn new(name: &str) -> SoundData {
        SoundData {
            name: name.to_owned(),
        }
    }
}

// Additional: Function demonstrating Result for error handling
fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("unable to find sound data".to_owned())
    }
}

fn main() {
    // Additional: Pattern matching on Result type
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error: {}", e),
    }
}

// Vector
// example 1
// Additional: Vector initialization with macro
let my_numbers = vec![1, 2, 3];

// Additional: Manual vector construction with push operations
let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);
my_numbers.pop();
let length = my_numbers.len(); // this is 2

let two = my_numbers[1];

// example 2
// Additional: Vector iteration example
let my_numbers = vec![1, 2, 3];

for num in my_numbers {
    println!("{:?}", num);
}

// String and &str
// example - Pass to function
// Additional: Function demonstrating string slice parameter
fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // Additional: Different ways to create and pass strings
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);
}

// Example - Will not work (wrong)
// Additional: Incorrect use of string reference lifetime
struct Employee {
    name: &'static str,
}

fn main() {
    let emp_name = "Jayson".to_owned();
    let emp = Employee {
        name: emp_name,
    };
}

// Example - Works!
// Additional: Correct approach using owned String
struct Employee {
    name: String,
}

fn main() {
    let emp_name = "Jayson".to_owned();
    let emp = Employee {
        name: emp_name,
    };
}

// Hashmap
// Additional: Example of HashMap usage with string keys and integer values
use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Cathy", 22);
    people.remove("Susan");

    // Additional: Safe value access with match
    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age),
        None => println!("not found"),
    }
}

// Additional: Different HashMap iteration methods
for (person, age) in people.iter() {
    println!("person = {:?}, age = {:?}", person, age);
}

for person in people.keys() {
    println!("person = {:?}", person);
}

for age in people.values() {
    println!("age = {:?}", age);
}

// 2.1-slides.md

// Traits
trait Noise { // trait
    fn make_noise(&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main () {
    hello(Person {}):
    hello(Dog {});
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof");
    }
}

// example 2 
trait Race {
    fn go(&self);
    fn is_ready(&self) -> bool;
    fn checkpoint(&self, position: i32);    
}

// example with generic function
trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("slither to ({},{})", x, y);
    }
}

struct Grasshopper;
impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("sliteher to ({},{})", x, y);
    }
}

// Review ()
trait Move {
    fn move_to(&self, x: i32, y: i32);
}

fn make_move(thing: impl Move, x: i32, y: i32) {
    thing.move_to(x, y);
}

let python = Snake {};
make_move(python, 1, 1);

// Output:
// slither to (1,1)

// Genetric Syntax
fn function<T: Trait1, U: Trait2>(param1: T, param2: U) {
    /* body */
}
fn function<T, U>(param1: T, param2: U) 
    where T: Trait1 + Trait2,
          U: Trait1 + Trait2 + Trait3,
{
    /* body */
}

// Generic Example
fn make_move(thing: impl Move, x:i3, y: i32) {　// impl Trait
    thing.move_to(x, y);
}

fn make_move<T: Move>(thing: T, x: i32, y: i32) { // Generic Parameter
    thing.move_to(x, y);
}

fn make_move<T>(thing: T, x: i32, y: i32) // Where Clause
where
    T: Move,
{
    thing.move_to(x, y);
}
// Choosing a syntax 

fn function(param1: impl Trait1, param2: impl Trait2) {
    /* body */
}

impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("hop to ({},{})", x, y);
    }
}
fn function(param1: impl Trait1, param2: impl Trait2) {
    /* body */
}

fn function<T: Trait1, U: Trait2>(param1: T, param2: U) {
    /* body */
}

fn function<T, U>(param1: T, param2: U)
where
    T: Trait1 + Trait2,
    U: Trait1 + Trait2 + Trait3,
{
    /* body */
}

// Monomorphization
trait Move {
    fn move_to(&self, x:i32, y:i32);
}

fn make_move<T: Move>(thing: T, x: i32, y: i32) {
    thing.move_to(x, y);
}

make_move(Snake {}, 1, 1);
make_move(Grasshopper {}, 3, 3);

fn make_move(thing: Snake, x: i32, y: i32) {
    thing.move_to(x, y);
}

fn make_move(thing: Grasshopper, x: i32, y: i32) {
    thing.move_to(x, y);
}

// Generic Structures 
// store data of any type within a structure

println!("Integer: {}", integer_container.value);
println!("String: {}", string_container.value);

// Conceptual Example

struct Name<T: Trait1 + Trait2, U: Trait3> { // Syntax 1
    field1: T,
    field2: U,
}

struct Name<T, U> // Syntax 2
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}

trait Seat { // Example - Definition
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

struct VIPSeat;
impl Seat for VIPSeat {
    fn show(&self) {
        println!("VIP Seat");
    }
}

struct RegularSeat;
impl Seat for RegularSeat {
    fn show(&self) {
        println!("Regular Seat");
    }
}

fn main() {
    let vip_ticket = Ticket { location: VIPSeat };
    let regular_ticket = Ticket { location: RegularSeat };

    vip_ticket.location.show();
    regular_ticket.location.show();
}
vip_ticket.location.show(); // Output: VIP Seat
regular_ticket.location.show(); // Output: Regular Seat

// Example - Types of seating

#[derive(Clone, Copy)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}

impl Seat for ConcertSeat {
    fn show(&self) { /* ... */ }
}

#[derive(Clone, Copy)]
enum AirlineSeat {
    BusinessClass,
    Economy,
    FirstClass,
}

impl Seat for AirlineSeat {
    fn show(&self) { /* ... */ }
}

// Example - Usage with single type

trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

fn ticket_info(ticket: Ticket<AirlineSeat>) {
    ticket.location.show();
}

let airline = Ticket { location: AirlineSeat::FirstClass };
ticket_info(airline);

// Example - Usage with generic type

trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}

let airline = Ticket { location: AirlineSeat::FirstClass };
let concert = Ticket { location: ConcertSeat::FrontRow };
ticket_info(airline);
ticket_info(concert);

// Details 

struct Ticket<T: Seat> {
    location: T,
}

fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}

let airline = Ticket { location: AirlineSeat::FirstClass };
let concert = Ticket { location: ConcertSeat::FrontRow };
ticket_info(airline);
ticket_info(concert);

// Details - Behind the scenes

struct AirlineTicket {
    location: AirlineSeat,
}

struct ConcertTicket {
    location: ConcertSeat,
}

fn airline_ticket_info(ticket: AirlineTicket) {
    ticket.location.show();
}

fn concert_ticket_info(ticket: ConcertTicket) {
    ticket.location.show();
}

// Details Heterogenous vector 
// Hetergenous vector allows to store different types of data in a vector 
// as long as they implement the same trait 
let airline = Ticket { location: AirlineSeat::FirstClass };
let concert = Ticket { location: ConcertSeat::FrontRow };
ticket_info(airline);
ticket_info(concert);

let tickets = vec![airline, concert];

// Details - Heterogenous vector
// Error: mismatched types
let tickets = vec![airline, concert];

// Implementing Functionality
// Generic implementation
impl<T: Seat> Ticket<T> {
    fn show(&self) {
        self.location.show();
    }
}

// Concrete implementation - Setup
trait Game {
    fn name(&self) -> String;
}

enum BoardGame {
    Chess,
    Monopoly,
}

impl Game for BoardGame {
    // Implementation would go here
}

enum VideoGame {
    playStation,
    Xbox,
}

impl Game for VideoGame {
    // Implementation would go here
}

// Concrete implementation - Usage
struct PlayRoom<T: Game> {
    game: T,
}
impl PlayRoom<BoardGame> {
    pub fn cleanup(&self) { /* ... */ }
}

// The rest of the code is using the PlayRoom struct with different types of games.
let video_room = PlayRoom {
    game: VideoGame::Xbox,
};

let board_room = PlayRoom {
    game: BoardGame::Monopoly,
};
// Assuming similar cleanup method implementation for VideoGame as for BoardGame
board_room.cleanup();
video_room.cleanup();

// Concrete Implementation - Error
struct PlayRoom<T: Game> {
    // ...
}

// ...

video_room.cleanup();
//     ^^^^^^^ method not found in `PlayRoom<VideoGame>`

// Generic Implementation 
struct Name<T: Trait1 + Trait2, U: Trait3> { // Syntax 1
    field1: T,
    field2: U,
}

impl<T: Trait1 + Trait2, U: Trait3> Name<T, U> {
    fn func(&self, arg1: T, arg2: U) { }
}
struct Name<T , U> // Syntax 2
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}

// Generic Implementation - Example 
trait Game {
    fn name(&self) -> String;
}

struct PlayRoom<T: Game> {
    game: T,
}

impl<T: Game> PlayRoom<T> {
    pub fn game_info(&self) {
        println!("{}", self.game.name());
    }
}

// Generic Implementation - Usage 
impl<T: Game> PlayRoom<T> {
    pub fn game_info(&self) { /* ... */ }
}

let video_room = PlayRoom {
    game: VideoGame::Xbox,
};

let board_room = PlayRoom {
    game: BoardGame::Monopoly,
};

video_room.game_info();
board_room.game_info();

// Stack 
// Data placed sequentially
// Limited space
// All variables stored on the stack
// Not all data
// Very fast to work with
// Offsets to access

// Heap
// Data placed algorithmically
// Slower than stack
// Unlimited space (RAM/disk limits apply)
// Uses pointers
// Pointers are a fixed size
// usize data type
// Vectors & HashMaps stored on the heap
// All dynamically sized collections

// Trait Object
// Dynamically allocated object
// Allows Mixed types in a collection
// Small performance penalty

// Creating a Trait Object
trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("click clack");
    }
}

// Creating a Trait Object  
let keeb = Keyboard;
let keeb_obj: &dyn Clicky = &keeb;

let keeb: &dyn Clicky = &Keyboard;

let keeb: Box<dyn Clicky> = Box::new(Keyboard);

// Trait Object Parameter - Borrow 
fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

let keeb = Keyboard;
borrow_clicky(&keeb);

// Trait Object Parameter - Move 
fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

let keeb = Box::new(Keyboard);
move_clicky(keeb);

// With Heterogeneous Vector 
struct Mouse;

impl Clicky for Mouse {
    fn click(&self) {
        println!("click");
    }
}

let keeb: Box<dyn Clicky> = Box::new(Keyboard);
let mouse: Box<dyn Clicky> = Box::new(Mouse);
let clickers = vec![keeb, mouse];

fn make_clicks(clickies: Vec<Box<dyn Clicky>>) {
    for clicker in clickies {
        clicker.click();
    }
}

let keeb = Box::new(Keyboard);
let mouse = Box::new(Mouse);
let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mouse]; 

make_clicks(clickers);

// Ownership Review - Example

fn place_item(freezer: &mut Freezer, item: FrozenItem) {
    freezer.contents.push(item);
}

#[derive(Debug)]
enum FrozenItem {
    IceCube,
}

#[derive(Debug)]
struct Freezer {
    contents: Vec<FrozenItem>,
}

fn main() {
    let mut freezer  = Freezer { contents: vec![] };
    let cube = FrozenItem::IceCube;
    place_item(&mut freezer, cube);
    // cube no longer available

// Lifetime Example - struct
enum Part {
    Bolt,
    Panel,
}

struct RobotArm<'a> {
    part: &'a Part,
}

struct AssemblyLine {
    parts: Vec<Part>,
}

fn main() {
    let line = AssemblyLine {
        parts: vec![Part::Bolt, Part::Panel],
    }

    let arm = RobotArm {
        part: &line.parts[0],
    };

    // arm no longer exists
}

fn name<'a>(arg: &'a DataType) --> &'a DataType {}

// Smart Pointers 

use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    vin: String, 
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

let car = Rc::new(Vehicle {
    vin: "123".to_owned(),
});

let left_door = Door {
    vehicle: Rc::clone(&car),
}

let right_door = Door {
    vehicle: Rc::clone(&car),
}

drop(car);

println!("vehicle = {:?}", left_door.vehicle);

// Metavariables 
// Used by the transcriber to make substitutions
// Metavariable will be substituted with the code provided by the invoker
// Starts with a dollar ($)

 $fn 
 $my_metavar
 $varname

 // Fragment specifiers 
 // Determines what kind of data is allowed in a metavariable 
 // Available specifiers are: item, block stmt, pat_param / pat, expr, ty, ident, path, tt, meta, lifetime, vis, literal

 // Creating a Macro

 macro_rules! your_macro_name {
     ($metavariable_name:fragment_specifier) => {
         // Can use $metavariable_name
     };
     ($a:ident, $b:literal, $c:tt) => {
         // Can use $a $b $c
     };
     () => {};
 }

 // Glyphs 

 macro_rules! your_macro_name {
     (_ wow! _ any | thing? yes.#meta (^.^)) => {
     };
 }

demo!(_ wow! _ any | thing? yes.#meta (^.^));

// Fragment Specifier: item
macro_rules! demo {
    ($i:item) => { $i };
}

demo!(const a: char = 'g');
demo! {fn hello()}
demo! {mod demo{}}
struct MyNum(i32);
demo! {
    impl myNum {
        pub fn demo(&self) { 
            println!("my num is {}", self.0);
        }
    }
}

// Fragment Specifier: block
macro_rules! demo {
    ($b:block) => { $b };
}
let num = demo!(
    {
        if 1 == 1 { 1 } else { 2}
    }
);

// Fragment Specifier: stmt
macro_rules! demo {
    ($s:stmt) => { $s };
}

demo!( let a = 5 );

// Fragment Specifier: pat / pat_param
macro_rules! demo {
    ($p:pat) => {{
        let num = 3;
        match num {
            $p => (),
            1 => (),
            _ => (),
        }
    }};
}

// Fragment Specifier: expr
macro_rules! demo {
    ($e:expr) => { $e };
}

demo!( loop {} );
demo!( 2 + 2 );
demo!({
    panic();
});

// Fragment Specifier: ty
macro_rules! demo {
    ($t:ty) => {{
        let d: $t = 4;
        fn add(lhs: $t, rhs: $t) -> $t {
            lhs + rhs
        }
    }}
}

demo!(i32);
demo!(usize);

// Fragment Specifier: ident
macro_rules! demo {
    ($i:ident, $i2:ident) => {
        fn $i() {
            println!("hello");
        }
        let $i2 = 5;
    }
}

demo!(say_hi, five);
say_hi();
assert_eq!(5, five);

// Fragment Specifier: path
macro_rules! demo {
    ($p:path) => {
        use $p;
    };
}

demo!(std::collections::HashMap);

// Fragment Specifier: tt
macro_rules! demo {
    ($t:tt) => {
        $t
    };
}

demo!(loop {});
demo!({ println!("Hello from inside a block!"); });
}

// Fragment Specifier:: meta

macro_rules demo {
    ($m:meta) => {
        #[derive($m)]
        struct MyNum(i32);
    }
}


demo!(Debug )

// Fragment Specifier: lifetime

macro_rules! demo {
    ($l:lifetime) => {
        let a: &$l str = "sample";
    };
}

demo!('static);

// Fragment Specifier: vis 

macro_rules! demo {
    ($v:vis) => {
        $v fn sample() {}
    };
}

demo!(pub);

// Fragment Specifier: literal

macro_rules! demo {
    ($l:literal) => {
        $l
    };
}

let five = demo!(5);
let hi = demo!("hello");

// Invoking a Macro Rule
// your_macro_name!(invocation);
// your_macro_name!{invocation};
// your_macro_name![invocation];

// Expression & Statement Positions
// Expressions
let nums = vec![1, 2, 3];

match vec![1, 2, 3].as_slice() {
    _ => format!("hello"),
}

// Statements
println!("Hello!");
dbg!(9_i64.pow(2));

// Pattern Position
macro_rules! pat {
    ($i:ident) => (Some($i))
}

// Patterns 
if let pat!(x) = Some(1) {
    assert_eq!(x, 1);
}

match Some(1) {
    pat!(x) => (),
    _ => (),
}

// Type Position
macro_rules! Tuple {
    ($A:ty, $B:ty) => { ($A, $B) };
}

// Types 
type N2 = Tuple!(i32, i32);

let nums: Tuple!(i32, char) = (1, 'a');

// Item Position
macro_rules! constant {
    ($name:ident) => { const $name: &'static str = "Jayson"; }
}

macro rules! newtype {
    ($name:ident, $typ:ty) => { struct $name($typ); }
}

// Items 
constant!(NAME);
assert_eq!(NAME, "Jayson");

newtype!(DemoStruct, usize);
let demo = DemoStruct(5);

// Associated Item Position
macro_rules! msg {
    ($msg:literal) => {
        pub fn msg() {
            println!("{}", $msg);
        }
    };
}

struct Demo;

// Associated item
impl Demo {
    msg!("demo struct");
}

// Associated Item Position
macro_rules! msg {
    ($msg:literal) => {
        pub fn msg() {
            println!("{}", $msg);
        }
    };
}

struct Demo;

// Associated item
impl Demo {
    msg!("demo struct");
}

// macro_rules transcribers
macro_rules! demo {
    () => {
        println!("{}", format!("demo{}", '!'));
    };
}

demo!();

// Overlapping Patterns

match slice {
    [first, ..] => (),
    [.., last] => (),
    [] => (),
}

// Preventing Overlapping Patterns

match slice {
    [] => (),
    [a, ..] => (),
    [a, b, ..] => (),
    [a, b, c, ..] => (),
    [a, b, c, d, ..] => (),
} // First two arms cover all cases, remaining will be ignored

match slice {
    [a, b, c, d, ..] => (),
    [a, b, c, ..] => (),
    [a, b, ..] => (),
    [a, ..] => (),
    [] => (),
} // All arms can be matched

// Guards
let nums = vec![7, 8, 9];
match nums.as_slice() {
    [first @ 1..=3, rest @ ..] => {
        // 'first' is always 1, 2 or 3
        // 'rest' is the remaining slice
    },
    [single] if single == &5 || single == &6 => (),
    [a, b] => (),
    [..] => (),
    [] => (),
}

// Typestates Patterns
// Leverage type system to encode state changes
// Example
struct BusTicket;
struct BoardedBusTicket;

impl BusTicket {
    fn board(self) -> BoardedBusTicket {
        BoardedBusTicket
    }
}

let ticket = BusTicket;
let boarded = ticket.board();

// Compile error
ticket.board();

// Example 2
struct File<'a>(&'a str);

impl<'a> File<'a> {
    fn read_bytes(&self) -> Vec<u8> {
        // ... read data ...
    }

    fn delete(self) {
        // ... delete file ...
    }
}

let file = File("data.txt");
let date = file.read_bytes();
file.delete();

// Compile error
let read_again = file.read_bytes();

// External Modules 
// Allows code to be compartmentalized
// Organized source code management
// Better collaboration
// More intuitive coding
// Quickly identify where imported code is used

// Module Details
// Can have any name, hierarchical organization, private by default, use pub keyword to make a module public
// External modules can be a directory or file

// Module are organized hierarchically
// Use super to go up one level
// Use crate to start from the top
// The as keyword can be used to create an alias for a module
// The mod keyword is used to declare a module
// No curly braces for external modules
// Modules can be re-exported with the use keyword
// pub indicates the module may be accessed from anywhere
// Omitting pub restricts access to only the containing module and sub-modules
