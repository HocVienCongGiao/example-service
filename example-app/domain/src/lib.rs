pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {
    use crate::boundaries::{
        Test1DbGateway, Test1SimpleMutationInputBoundary, Test1SimpleMutationRequest,
    };

    struct Test1DbGatewayStub {}
    impl Test1DbGateway for Test1DbGatewayStub {
        fn exists_by_name(&self, name: String) -> bool {
            if name == "existing" {
                return true;
            }
            false
        }
    }

    #[test]
    fn it_works() {
        let test1_simple_mutator =
            crate::interactors::test1_mutation::Test1SimpleMutationInteractor::new(Box::new(
                Test1DbGatewayStub {},
            ));
        test1_simple_mutator.create_test1(Test1SimpleMutationRequest {
            name: "existing".to_string(),
        });
        test1_simple_mutator.create_test1(Test1SimpleMutationRequest {
            name: "new".to_string(),
        });
        assert_eq!(2 + 2, 4);
    }
}
