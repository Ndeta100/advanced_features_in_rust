pub trait HelloMacro {
    fn hello_macro();
}
struct Pancakes;
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("My name is Pancakes!")
    }
}
#[cfg(test)]
mod test {}
