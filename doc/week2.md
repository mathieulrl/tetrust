## struct

### References

* [Rust book](https://doc.rust-lang.org/book/ch05-00-structs.html)

A struct is a composite data type that allows you to define your own custom data structure by combining different types into a single unit. It is similar to a struct in C or a class in other languages.

To define a struct in Rust, you use the `struct` keyword followed by the name of the struct and a block that contains the fields of the struct. Each field is defined with a name and its corresponding type. Here's an example:

```rust
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}
```

In the above example, we define a `Person` struct with three fields: `name` of type `String`, `age` of type `u32` (unsigned 32-bit integer), and `is_student` of type `bool`.

Once you've defined a struct, you can create instances of that struct by using the `struct_name { field: value, ... }` syntax. Here's an example:

```rust
let person1 = Person {
    name: String::from("Alice"),
    age: 25,
    is_student: true,
};
```

In this case, we create a `person1` instance of the `Person` struct and provide values for each field.

You can access the fields of a struct instance using dot notation. For example:

```rust
println!("Name: {}", person1.name);
println!("Age: {}", person1.age);
println!("Is Student: {}", person1.is_student);
```

### Implement a struct

Structs in Rust can also have associated functions, or "methods". Methods are defined using the `impl` block, and can access or modify the data of the struct.

```rust
impl Person {
    fn new(name: String, age: u32, is_student: bool) -> Person {
        Person {
            name,
            age,
            is_student,
        }
    }

    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Is Student: {}", self.is_student);
    }

    fn toggle_student(&mut self) {
        self.is_student = !self.is_student;
    }

    fn print_and_drop(self) {
        println!("Dropping person: {} ({} years old)", self.name, self.age);
        // Here, the Person instance is dropped and its resources are freed
    }
}
```

In the example above, we define an associated function `new` that creates a new `Person` instance, and a method `display_info` that displays the information of a `Person` instance.

Note that `display_info` only takes an immutable reference while `toggle_student` takes a mutable reference. `print_and_drop` takes ownership of the object and doesn't return anything, effectively destroying it.

We can find many examples of this in our code, even the `Game` itself is [a struct]():

```rust
struct Game {
    board: Board,
    piece_bag: PieceBag,
    piece: Piece,
    piece_position: Point,
}
```

Note that each member of `Game` is also a struct which implements its own methods, which is a common way to structure our code. 

## enum

### References

* [Rust book](https://doc.rust-lang.org/book/ch06-00-enums.html)

An enum, short for "enumeration," is a data type in Rust that represents a fixed set of possible values. It allows you to define a type by enumerating its variants, each of which can have different data associated with it. Enums are a powerful tool for expressing concepts that have a distinct set of possible states or options.

In Rust, an enum is defined using the `enum` keyword, followed by the name of the enum and a list of its variants. Here's a [simple example]():

```rust
enum Color {
    Black,
    Cyan,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
}
```

Contrary to other languages, Rust's enums can also have associated data with their variants. For example:

```rust
enum GameUpdate {
    KeyPress(Key),
    Tick,
}
```

Here `GameUpdate` has 2 variants, `KeyPress` and `Tick`. This is important because it means that there are 2 events that will modify our Tetris game: when the player press a key, and when some amount of time is elapsed. 

Of course the player can press different keys, so we can pass a `Key` to the event `KeyPress`.

We can handle conditional over enums with `if let` statements:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Quarter;

    if let Coin::Quarter = coin {
        println!("It's a quarter!");
    } else {
        println!("It's not a quarter!");
    }
}
```

Or matching statements if we want to cover all the variants:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Quarter;

    println!("value in cents: {}", value_in_cents(coin)); // 25
}
```

Exercice: Write a new enum with all the game over conditions (`LockOut`, `BlockOut`, `TopOut`).

## Option and Result

### References

* [Result in the Book](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
* [Options in Rust by examples](https://doc.rust-lang.org/rust-by-example/std/option.html)

Both are enums with 2 variants:

* `Option`: `Some` or `None`
* `Result`: `Ok` or `Error`

This is the basis of error management in Rust, it is often used as a return value of a function. 

Those enums implement different methods to handle them for the caller.
In Rust, `Option` and `Result` types have several methods that can be used to unwrap values, each with different behaviors.

**1. unwrap():**

`unwrap()` will return the value if it is `Some` or `Ok`. If it is `None` or `Err`, it will panic.

```rust
let opt = Some(5);
let value = opt.unwrap(); // value is 5

let res: Result<i32, &str> = Ok(5);
let value = res.unwrap(); // value is 5

let none = None;
let value = none.unwrap(); // panic
```

**2. unwrap_or(default):**

`unwrap_or(default)` will return the value if it is `Some` or `Ok`. If it is `None` or `Err`, it will return the default value that you pass in.

```rust
let opt = None;
let value = opt.unwrap_or(0); // value is 0

let res: Result<i32, &str> = Err("error");
let value = res.unwrap_or(0); // value is 0
```

**3. unwrap_or_else(func):**

`unwrap_or_else(func)` will return the value if it is `Some` or `Ok`. If it is `None` or `Err`, it will call the function you pass in and return the result.

```rust
let opt = None;
let value = opt.unwrap_or_else(|| {
    println!("Was None");
    0
}); // Prints "Was None", value is 0

let res: Result<i32, &str> = Err("error");
let value = res.unwrap_or_else(|e| {
    println!("Error: {}", e);
    0
}); // Prints "Error: error", value is 0
```

**4. unwrap_err():**

`unwrap_err()` will return the error if the `Result` is `Err`. If it is `Ok`, it will panic. (This is not available on `Option`.)

```rust
let res: Result<i32, &str> = Err("error");
let error = res.unwrap_err(); // error is "error"
```

**5. expect(message):**

`expect(message)` is similar to `unwrap()`, but it lets you specify a message that will be printed if it panics.

```rust
let opt = None;
let value = opt.expect("Expected a value"); // Panics with the message "Expected a value"

let res: Result<i32, &str> = Err("error");
let value = res.expect("Expected a result"); // Panics with the message "Expected a result"
```

**6. if let Some():**

This allow for variable declaration with an `Option`:

```rust
let option = Some("Ulysse");
// let option = None;

if let Some(name) = option {
    println!("My name is {}", name);
} else {
    println!("My name is Nobody");
}
```

Exercice: Rewrite all the `match ... { Some() => ..., None => ... }` in the code with `if let Some()` syntax.

## Iterators

### Reference

* [Iterators in the Book](https://doc.rust-lang.org/book/ch13-02-iterators.html)

Iterators provide a uniform interface for traversing and processing elements within a collection. It represents a sequence of elements that can be iterated over, and abstracts away the details of accessing elements and provides useful methods for transformation and manipulation. 

Iterators follow the iterator pattern, which consists of two main traits: `Iterator` and `IntoIterator`.

The `Iterator` trait defines a set of methods that allow you to work with the elements of a collection sequentially. Some commonly used methods from the `Iterator` trait include `map`, `filter`, `fold`, and `collect`.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Creating an iterator from a vector using the `iter` method
    let mut iter = numbers.iter();

    // Iterating over the elements using a `for` loop
    for number in iter {
        println!("Number: {}", number);
    }

    // Using the `map` method to transform each element and collect the results into a new vector
    let doubled_numbers: Vec<_> = numbers.iter().map(|&x| x * 2).collect();

    println!("Doubled numbers: {:?}", doubled_numbers);

    // Using the `filter` method to only keep elements that satisfy a condition
    let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();

    println!("Even numbers: {:?}", even_numbers);
}
```

We use an iterator when [clearing lines]() in `clear_lines()`:

```rust
fn clear_lines(&mut self) -> u32 {
    ...
    while !self.cells[row].iter().any(|x| *x == None) {
                    cleared_lines += 1;
                    self.cells[row] = self.cells[row - cleared_lines];
                    self.cells[row - cleared_lines] = [None; BOARD_WIDTH as usize];
                }
    ...
}
```

Here `any()` returns `true` if any element in the iterator satisfies the condition expressed in the closure.

## What's a trait?

### References

* [in the Book](https://doc.rust-lang.org/book/ch10-02-traits.html)
* [supertrait in the Book](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait)

A trait is set of shared methods between different types, that usually implement them differently. 

It is similar to interfaces in object-oriented languages like Java or C#. Here's an example in Java to illustrate the concept:

```java
// Interface definition
public interface Drawable {
    void draw();
}

// Class implementing the interface
public class Circle implements Drawable {
    @Override
    public void draw() {
        System.out.println("Drawing a circle");
    }
}

// Class using the interface
public class DrawingApp {
    public static void main(String[] args) {
        Drawable drawable = new Circle();
        drawable.draw();
    }
}
```

In this example, the `Drawable` interface defines a contract that any implementing class must adhere to. It specifies that any class implementing the `Drawable` interface must provide an implementation for the `draw` method. The `Circle` class implements the `Drawable` interface and provides its own implementation of the `draw` method.

The `DrawingApp` class demonstrates how the interface can be used. It creates an instance of `Circle` but stores it in a variable of type `Drawable`, which is the interface type. This allows us to treat the `Circle` object as a `Drawable` and call the `draw` method on it.

Similarly, in Rust, traits define a set of methods that types can implement. Here's an equivalent example in Rust using traits:

```rust
// Trait definition
trait Drawable {
    fn draw(&self);
}

// Struct implementing the trait
struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

// Function using the trait
fn main() {
    let drawable: &dyn Drawable = &Circle;
    drawable.draw();
}
```

In this Rust example, the `Drawable` trait defines the `draw` method. The `Circle` struct implements the `Drawable` trait by providing its own implementation of the `draw` method.

The `main` function demonstrates how the trait can be used. It creates a variable of type `dyn Drawable` (a trait object) and assigns it the value of a `Circle` instance. This allows us to treat the `Circle` object as a `Drawable` and call the `draw` method on it.

Both Java's interfaces and Rust's traits enable polymorphism and define a contract that types must adhere to. However, there are some differences between the two. Rust's traits can also include associated types and provide default implementations for methods, which makes them more powerful than Java interfaces. Additionally, Rust's trait objects provide dynamic dispatch, allowing for runtime polymorphism, whereas Java interfaces use static dispatch.

Let's illustrate this by adding another type that implements `Drawable`, `Rectangle`:
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {}", self.width, self.height);
    }
}

fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 7.0 };

    draw_shape(&circle);
    draw_shape(&rectangle);
}
```

### Trait bound

It's also possible to define constraints on the types that can implement a trait, generally by specifying some other traits that it must implement first (this is known as trait bound).

```rust
trait Shape {
    fn area(&self) -> f64;
}

trait Draw {
    fn draw(&self);
}

// We declare that we will implement Draw for types T that already implement Shape.
impl<T: Shape> Draw for T {
    fn draw(&self) {
        println!("Drawing a shape with area: {}", self.area());
    }
}

struct Circle {
    radius: f64,
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

fn main() {
    let my_circle = Circle { radius: 2.0 };

    // Because Circle implements Shape, it also now implements Draw
    my_circle.draw(); // This will print: "Drawing a shape with area: 12.56636"
}
```

You certainly noticed that we didn't implement `Draw`, it comes for free as we implemented `Shape`. This is called blanket implementation.
