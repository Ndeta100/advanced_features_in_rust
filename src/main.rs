use core::{fmt, slice};
use proc_macro;
use std::{
    fmt::Error,
    ops::{Add, Deref},
};

fn main() {
    //Unsafe rust
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
    //Unsafe function methods
    unsafe fn dangerous() {}
    unsafe { dangerous() }

    //Safe abstraction
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r_2 = &mut v[..];
    let (a, b) = r_2.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    //using extern to call external code
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("The absolute value of -3 is {}", abs(-3));
    }
    //calling rust fn from other languages
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a rust function from C");
    }
    //staric variables
    static HELLO_ME: &str = "Hi";
    println!("{}", HELLO_ME);
    //Unsafe traits
    unsafe trait Foo {
        //Mehtod go here
    }
    unsafe impl Foo for i32 {
        //Method
    }
    //      ADVANCED TRAITS
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {
        count: i32,
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count as u32)
            } else {
                None
            }
        }
    }
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human {}
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!")
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*Waving arms furiously");
        }
    }
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("Puppy")
        }
    }
    trait OutlinePoint: fmt::Display {
        fn outline_point(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("{}", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({} {})", self.x, self.y)
        }
    }
    impl OutlinePoint for Point {}
    let point_me = Point { x: 3, y: 7 };
    point_me.outline_point();
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w={}", w);
    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let w_vec = &*w;
    //  USING NEWTYPES, ADVANCED TYPES
    type Kilometer = i32;
    let distance: Kilometer = 3894;
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("Hi"));
    fn takes_long_type(f: Thunk) {
        //snip
    }
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    }
    //Won't compile
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called  `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }
    fn generics<T>(t: T) {
        //
    }
    fn generics_1<T: ?Sized>(t: &T) {}
    //ADVANCED FUNCTIONS AND CLOSURES
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is : {}", answer);
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    enum Status {
        Value(u32),
        Stop,
    }
    //Won't compile, Use the Sized trait tofix
    // fn return_closure() ->< dyn Fn(i32) -> i32> {
    //     |x| x1
    // }
    //Will compile
    fn return_closure_1() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    //MACROS
    let V: Vec<u32> = vec![1, 2, 3];
    #[macro_export]
    macro_rules! vec  {
        ($($x:expr),*) => {
            {
                let mut temp_vec=Vec::new();
                $(
                    temp_vec.push($x);
                )*
                //The expression above expands to this
                //temp_vec.push(1)
               // temp_vec.push(2)
                //temp_vec.push(3)
                temp_vec
            }
        };
    }
    // #[some_attribute]
    //  pub fn some_name(input: TokenStream) -> TokenStream {}
}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let pointer = slice.as_mut_ptr();
    assert!(mid < len);
    unsafe {
        (
            slice::from_raw_parts_mut(pointer, mid),
            slice::from_raw_parts_mut(pointer.add(mid), mid),
        )
    }
}
