use std::sync::Mutex;

use crate::{
    db::user_repository::UserRepository,
    models::user::{RequestUser, User},
};

static USER_STORAGE: Mutex<Vec<User>> = Mutex::new(vec![]);

pub async fn create_user(user: RequestUser) -> Result<i32, String> {
    let user_repo = UserRepository::new();
    let result = user_repo.create(user).await;

    match result {
        Ok(user_id) => Ok(user_id),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_user(id: i32) -> Option<User> {
    let user_repo = UserRepository::new();
    let result = user_repo.get_by_id(id).await;

    match result {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn get_all_users() -> Vec<User> {
    USER_STORAGE.lock().unwrap().clone()
}

pub fn update_user(id: usize, name: Option<String>, email: Option<String>) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    let user = storage
        .iter_mut()
        .find(|u| id == u.id as usize)
        .ok_or(format!("User with id {} not found", id))?;

    if let Some(name) = name {
        user.name = name;
    }
    if let Some(email) = email {
        user.email = Some(email);
    }
    Ok(())
}

pub fn delete_user(id: usize) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    let pos = storage
        .iter()
        .position(|u| id == u.id as usize)
        .ok_or(format!("User with id {} not found", id))?;

    storage.remove(pos);
    Ok(())
}
