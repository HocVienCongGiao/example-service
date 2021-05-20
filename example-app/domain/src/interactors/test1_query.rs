use crate::boundaries;
use crate::boundaries::{Test1DbGateway, Test1SimpleQueryRequest, Test1SimpleQueryResponse};
use futures::executor::block_on;

pub struct Test1SimpleQueryInteractor {
    db_gateway: Box<dyn Test1DbGateway>,
}

impl boundaries::Test1SimpleQueryInputBoundary for Test1SimpleQueryInteractor {
    fn get_test1(&self, request: Test1SimpleQueryRequest) -> Test1SimpleQueryResponse {
        println!("test1 simple mutation input boundary {}", request.name);
        let status: u16;
        if block_on((*self).db_gateway.exists_by_name(request.name.clone())) {
            println!("user found");
            status = 200;
        } else {
            println!("user not found");
            status = 404;
        }
        Test1SimpleQueryResponse { status }
    }
}

impl Test1SimpleQueryInteractor {
    pub fn new(db_gateway: Box<dyn Test1DbGateway>) -> Self {
        Test1SimpleQueryInteractor { db_gateway }
    }
}
