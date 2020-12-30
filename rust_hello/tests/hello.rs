use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::result::Result;

#[test]
fn test_print() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("{number:>width$}", number = 1, width = 6); //     1
    println!("{number:>0width$}", number = 1, width = 6); //000001

    println!("This struct `{:?}` won't print...", DebugPrintable(3));
}

struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);
#[derive(Debug)]
struct Deep(DebugPrintable);
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[test]
fn test_debug() {
    println!("{:?} months a year.", 12);

    println!(
        "{1:?} {0:?} {2:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", DebugPrintable(3));

    println!("Now {:?} will print!", Deep(DebugPrintable(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

struct Structure(i32);

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "( {}, {})", self.0, self.1)
    }
}
struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        let vec = &self.0;
        write!(f,"[")?;
        for(count ,v) in vec.iter().enumerate(){
            if count != 0 {write!(f,", ")?};
            write!(f,"{}",v)?;
        }
        write!(f,"]")
    }
}

#[test]
fn test_display() {
    let minmax = MinMax(1, 2);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "the big is {big}, the small is {small}",
        big = big_range,
        small = small_range
    );
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
