mod mutation;
mod query;

pub trait Test1SimpleRepository {}

impl domain::boundaries::Test1DbGateway for dyn Test1SimpleRepository {
    fn exists_by_name(&self, name: String) -> bool {
        todo!()
    }
}
