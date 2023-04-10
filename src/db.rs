use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::config::Config;
use tokio_postgres::NoTls;
use url::Url;

pub type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn create_pool(database_url: &str) -> PostgresPool {
    let url = Url::parse(database_url).unwrap();

    let mut config = tokio_postgres::Config::new();
    config.host(&url.host_str().unwrap());
    config.port(url.port().unwrap_or(5432));
    config.user(url.username());
    config.password(url.password().unwrap_or(""));
    config.dbname(&url.path()[1..]);

    let manager = PostgresConnectionManager::new(config, NoTls);
    Pool::builder().build(manager).await.unwrap()
}
