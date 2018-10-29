//! #hazel_rust
//!
//! hazel 의 rust porting
//! porting by Yeonho Jang <siabard@gmail.com>

pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

pub fn print() {
    println!("Welcome to hazel engine");
}
