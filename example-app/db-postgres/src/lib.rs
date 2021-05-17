use domain::test_func;
use tokio_postgres::{Error, NoTls};

mod embedded {
    use refinery::{embed_migrations, Error};
    embed_migrations!("migrations");
}

// #[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    println!("Start of main()");
    // connect to the DB
    let (mut client, connection) =
        tokio_postgres::connect("postgresql://postgres:p%40ssword@localhost/test", NoTls).await?;
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
    embedded::migrations::runner()
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

    #[tokio::test]
    async fn it_works() {
        println!("Calling main()");
        main().await.expect("Failing for no reason");
        println!("Finished main()");
        assert_eq!(2 + 2, 4);
    }
}
