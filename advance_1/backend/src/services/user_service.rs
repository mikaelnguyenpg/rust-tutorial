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
        let result = self.user_repo.create(user).await;

        match result {
            Ok(user_id) => Ok(user_id),
            Err(e) => Err(e.to_string()),
        }
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
        let result = self.user_repo.update(id, updated).await;

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
        // return Err(String::from("Test rollback"));
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), String> {
        let result = self.user_repo.delete(id).await;

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
