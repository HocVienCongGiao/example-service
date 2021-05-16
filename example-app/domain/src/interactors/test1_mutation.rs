use crate::boundaries;

pub struct Test1Mutator {}
impl boundaries::Test1MutationInputBoundary for Test1Mutator {
    fn test1() {
        println!("test 1 input boundary");
    }
}
