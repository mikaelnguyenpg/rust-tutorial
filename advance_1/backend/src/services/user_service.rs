use crate::{
    db::{DbTransaction, user_repository::UserRepository},
    models::user::{RequestUser, User},
};

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(tx: DbTransaction) -> Self {
        UserService {
            user_repo: UserRepository::new(tx),
        }
    }

    pub async fn create_user(&self, user: RequestUser) -> Result<i32, String> {
        self.user_repo.create(user).await.map_err(|e| e.to_string())
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        let result = self.user_repo.get_by_id(id).await;

        match result {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        self.user_repo.get_all().await
    }

    pub async fn update_user(&self, id: i32, updated: RequestUser) -> Result<(), String> {
        self.user_repo
            .update(id, updated)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), String> {
        self.user_repo.delete(id).await.map_err(|e| e.to_string())
    }
}
