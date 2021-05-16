pub fn test_func() {
    println!("hello");
}

mod boundaries;
mod interactors;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
