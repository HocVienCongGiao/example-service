use domain::test_func;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, NoTls};

pub mod config;

mod embedded {
    use refinery::{embed_migrations, Error};
    embed_migrations!("migrations");
}

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
    use pg_embed::fetch;
    use pg_embed::fetch::{Architecture, FetchSettings, OperationSystem, PG_V13};
    use pg_embed::postgres::{PgEmbed, PgSettings};
    use std::time::Duration;

    async fn init_embedded_postgres() -> PgEmbed {
        /// Postgresql settings
        let pg_settings = PgSettings {
            /// Where to store the postgresql executables
            executables_dir: "data/postgres".to_string(),
            /// Where to store the postgresql database
            database_dir: "data/db".to_string(),
            port: 6543,
            user: "postgres".to_string(),
            password: "password".to_string(),
            /// If persistent is false clean up files and directories on drop, otherwise keep them
            persistent: false,
            start_timeout: Duration::from_secs(15),
            /// If migration sql scripts need to be run, the directory containing those scripts can be
            /// specified here with `Some(path_to_dir), otherwise `None` to run no migrations.
            /// To enable migrations view the **Usage** section for details
            migration_dir: None,
        };

        let config = crate::config::Config::new();
        let postgres_os: OperationSystem;
        match config.os_type.as_str() {
            "linux" => postgres_os = OperationSystem::Linux,
            "darwin" => postgres_os = OperationSystem::Darwin,
            _ => postgres_os = OperationSystem::Windows,
        }

        /// Postgresql binaries download settings
        let fetch_settings = FetchSettings {
            host: "https://repo1.maven.org".to_string(),
            operating_system: if config.os_type == "windows" {
                OperationSystem::Windows
            } else {
                OperationSystem::Linux
            },
            architecture: Architecture::Amd64,
            version: PG_V13,
        };

        /// Create a new instance
        let mut pg = PgEmbed::new(pg_settings, fetch_settings);

        // async block only to show that these methods need to be executed in an async context
        // async {
        /// Download, unpack, create password file and database cluster
        let is_setup = pg.setup().await;
        println!("is_setup {:?}", is_setup);
        /// start postgresql database
        let started = pg.start_db().await;
        println!("isStarted {:?}", started);

        /// create a new database
        /// to enable migrations view the **Usage** section for details
        pg.create_database("database_name").await;

        // /// drop a new database
        // /// to enable migrations view [Usage] for details
        // /// to enable migrations view the **Usage** section for details
        // pg.drop_database("database_name").await;

        /// check database existence
        /// to enable migrations view [Usage] for details
        /// to enable migrations view the **Usage** section for details
        let exists = pg.database_exists("database_name").await;
        println!("DB Exists? {:?}", exists);

        // /// run migration sql scripts
        // /// to enable migrations view [Usage] for details
        // /// to enable migrations view the **Usage** section for details
        // pg.migrate("database_name").await;
        // };
        /// get the base postgresql uri
        /// `postgres://{username}:{password}@localhost:{port}`
        let pg_uri: &str = &pg.db_uri;
        println!("pg_uri is {}", pg_uri);
        /// get a postgresql database uri
        /// `postgres://{username}:{password}@localhost:{port}/{specified_database_name}`
        let pg_db_uri: String = pg.full_db_uri("database_name");
        println!("full_db_uri is {}", pg_db_uri);

        // stop postgresql database
        // pg.stop_db();
        return pg;
    }

    #[tokio::test]
    async fn it_works() {
        dotenv::dotenv().ok();
        println!("testing env {}", std::env::var("HELLO").unwrap());
        println!("Calling main()");
        let mut pg = init_embedded_postgres().await;
        main().await.expect("Failing for no reason");
        println!("Finished main()");
        assert_eq!(2 + 2, 4);
        pg.stop_db();
    }
}
