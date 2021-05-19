use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

mod mutation;
mod query;

pub struct Test1SimpleRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::Test1DbGateway for Test1SimpleRepository {
    async fn exists_by_name(&self, name: String) -> bool {
        let stmt_future = (*self)
            .client
            .prepare("SELECT * FROM author_initial WHERE name = $1");
        println!("First block_on for stmt");
        let stmt = stmt_future.await.unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&name];
        let result = (*self).client.query_one(&stmt, name_param).await;
        println!("second block_on for row");
        if result.is_err() {
            return false;
        }
        let row = result.unwrap();
        let name_found: String = row.get("name");
        println!("ROW IS {}", name);
        name_found == name
    }
}
