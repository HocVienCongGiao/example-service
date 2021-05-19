use pg_embed::postgres::PgEmbed;
use std::path::PathBuf;
use std::sync::Once;

mod common;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn integration_works() {
    initialise();
    println!("is it working?");
    let mut pg: PgEmbed = common::embedded::start_postgres().await;
    let _ = db_postgres::main().await;
    let pool = db_postgres::create_connection_pool().await;
    for i in 1..10 {
        let client = pool.get().await.unwrap();
        let stmt = client.prepare("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
        println!("Looping for value {} of index {}", value, i);
    }
    assert_eq!(2 + 2, 4);
    println!("finished integration test");
    let _ = pg.stop_db();
}

// #[tokio::test]
// async fn db_tests() {
//     println!("start db_tests");
//     initialise();
//     let mut pg = crate::embedded::start_postgres().await;
//     main().await.expect("Failing for no reason");
//     println!("Finished main() in test 2");
//     assert_eq!(2 + 2, 4);
//     println!("finished test2");
//     let _ = pg.stop_db();
// }
