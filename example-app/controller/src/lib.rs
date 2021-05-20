use domain::boundaries::{
    Test1SimpleQueryInputBoundary, Test1SimpleQueryRequest, Test1SimpleQueryResponse,
};
use domain::test_func;

pub mod openapi;

pub fn get_test1() -> Test1SimpleQueryResponse {
    let client = db_postgres::connect().await;
    let client = db_postgres::main(client).await.unwrap();
    let test1_repository = Test1SimpleRepository { client };

    domain::interactors::test1_query::Test1SimpleQueryInteractor::new(test1_repository).get_test1(
        Test1SimpleQueryRequest {
            name: "Ngo Dinh Nhu".to_string(),
        },
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
