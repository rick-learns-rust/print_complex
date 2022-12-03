use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:b} y: {:b}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    r: f64,
    i: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}

fn main() {
    let min_max = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3, y: 7 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let c = Complex { r: 3.3, i: 1.7 };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    println!("What does Point2D look like in binary: {:b}?", point);
}
