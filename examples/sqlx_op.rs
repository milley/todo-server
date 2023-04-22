use sqlx::mysql::MySqlPoolOptions;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Todo {
    id: i64,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@localhost/todo")
        .await?;
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let todos = sqlx::query_as::<_, Todo>("SELECT id, title FROM todo_list")
        .fetch_all(&pool)
        .await?;

    assert_eq!(todos.len(), 2);
    for todo in todos.iter() {
        println!("{:#?}", todo);
    }

    Ok(())
}
