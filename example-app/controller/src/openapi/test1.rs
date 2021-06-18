pub use domain::boundaries::Test1SimpleQueryResponse;
pub use hvcg_example_openapi_entity::models::Pet;

pub fn create_test1() {
    println!("Creating Test1 in Controller OpenApi test1.rs")
}

impl ToOpenApi<Pet> for Test1SimpleQueryResponse {
    fn to_openapi(&self) -> hvcg_example_openapi_entity::models::Pet {
        Pet {
            id: None,
            category: None,
            name: "No Name for status ".to_string() + self.status.to_string().as_str(),
            photo_urls: vec![],
            tags: None,
            status: Option::from(self.status.to_string()),
        }
    }
}

pub trait ToOpenApi<T> {
    fn to_openapi(&self) -> T;
}
