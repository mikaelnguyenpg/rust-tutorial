use crate::models::user::{RequestUser, User};
use sqlx::postgres::PgPoolOptions;

pub struct UserRepository {
    url: String,
    max_connection: u32,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            url: dotenv::var("DATABASE_URL").expect("DATABASE_URL must defined"),
            max_connection: 3,
        }
    }

    pub async fn create(&self, user: RequestUser) -> Result<i32, sqlx::Error> {
        // connect to db
        let db = PgPoolOptions::new()
            .max_connections(self.max_connection)
            .connect(&self.url)
            .await?;

        // execute sql to insert user to user table
        let row = sqlx::query!(
            "INSERT INTO users_demo (name, email) VALUES ($1, $2) RETURNING id",
            user.name,
            user.email
        )
        .fetch_one(&db)
        .await?;

        // return user_id
        Ok(row.id)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let db = PgPoolOptions::new()
            .max_connections(self.max_connection)
            .connect(&self.url)
            .await?;

        let user = sqlx::query_as::<_, User>("SELECT * FROM users_demo WHERE id = $1")
            .bind(id)
            .fetch_one(&db)
            .await?;

        Ok(user)
    }
}
