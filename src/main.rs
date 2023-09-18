fn main() {
    icepocha_api::main();
}

async fn run() -> Result<(), DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL을 설정하세요.");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME을 설정하세요.");
    let db = Database::connect(database_url.clone()).await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name),
    ))
    .await?;

    let url = format!("{}/{}", database_url, database_name);
    let db = Database::connect(&url).await?;

    Migrator::refresh(&db).await?;

    Ok(())
}