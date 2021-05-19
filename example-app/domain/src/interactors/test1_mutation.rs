use crate::boundaries;
use crate::boundaries::{Test1DbGateway, Test1SimpleMutationRequest, Test1SimpleMutationResponse};

pub struct Test1SimpleMutator {
    db_gateway: Box<dyn Test1DbGateway>,
}

impl boundaries::Test1SimpleMutationInputBoundary for Test1SimpleMutator {
    fn create_test1(&self, request: Test1SimpleMutationRequest) -> Test1SimpleMutationResponse {
        println!("test1 simple mutation input boundary {}", request.name);
        if (*self).db_gateway.exists_by_name(request.name) {
            println!("user with this name already exists");
        } else {
            println!("new user, all is good");
        }
        Test1SimpleMutationResponse {}
    }
}

impl Test1SimpleMutator {
    pub fn new(db_gateway: Box<dyn Test1DbGateway>) -> Test1SimpleMutator {
        Test1SimpleMutator { db_gateway }
    }
}
