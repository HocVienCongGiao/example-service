use domain::test_func;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, NoTls};

pub mod config;

mod embedded;
mod migration;

// #[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    println!("Start of main()");
    // connect to the DB
    let (mut client, connection) = tokio_postgres::connect(
        "user=postgres password=password host=localhost port=6543 dbname=database_name",
        //         tokio_postgres::connect("postgresql://postgres:password@localhost/test", NoTls).await?;
        NoTls,
    )
    .await?; // p%40ssword
             // let mut obj = db_pool.get().await?;
             // let client_refinery = obj.deref_mut().deref_mut();
    test_func();
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("start migrations");
    migration::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Hey why did I fail?");
    // embedded::migrations::runner().run_async(&mut client);
    println!("finished migrations");

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
        )
        .await;

    let query = client.query_one(
        "
            SELECT * FROM author_initial      
            WHERE id = $1        
    ",
        &[&2],
    );

    let result = query.await;

    // use std::io::{stdin, stdout, Write};
    // let mut s = String::new();
    // print!("Please enter some text: ");
    // let _ = stdout().flush();
    // stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");
    // if let Some('\n') = s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r') = s.chars().next_back() {
    //     s.pop();
    // }
    // println!("You typed: {}", s);

    let row = result.unwrap();
    let author_name = row.get::<&str, &str>("name");
    println!("got row {}", author_name);
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::main;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialise() {
        let mut init = false;
        INIT.call_once(|| {
            dotenv::dotenv().ok();
            println!("testing env {}", std::env::var("HELLO").unwrap());
        });
    }

    #[tokio::test]
    async fn it_works() {
        initialise();
        assert_eq!(2 + 2, 4);
        println!("finished test1");
    }

    #[tokio::test]
    async fn db_tests() {
        println!("start db_tests");
        initialise();
        let mut pg = crate::embedded::start_postgres().await;
        main().await.expect("Failing for no reason");
        println!("Finished main() in test 2");
        assert_eq!(2 + 2, 4);
        println!("finished test2");
        pg.stop_db();
    }
}
