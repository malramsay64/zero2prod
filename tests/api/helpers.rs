use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::startup::{get_connection_pool, Application};

// Ensure the tracing stack is only initialised once with lazy_static
lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { ""};
        let subscriber = get_subscriber("test".into(), filter.into());
        init_subscriber(subscriber);
    };
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to execute request.")
    }
    }


pub async fn spawn_app() -> TestApp {
    // The first time we call initialize, the code is run. Subsequent invocations skip this
    // execution.
    lazy_static::initialize(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");

        // Create a new random database name to run the tests with
        c.database.database_name = Uuid::new_v4().to_string();

        c.application.port = 0;
        c
    };
    // Create and migrate the database
    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone()).await.expect("Failed to build the application.");

    let address = format!("http://127.0.0.1:{}", application.port());


    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database).await.expect("Failed to connect to the database."),
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate Database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
