use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

static USER_STORAGE: Mutex<Vec<User>> = Mutex::new(vec![]);

pub fn create_user(user: &User) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    if storage.iter().any(|u| u.id == user.id) {
        return Err(format!("User with id {} already exists", user.id));
    }

    storage.push(user.clone());
    Ok(())
}

pub fn get_user(id: u32) -> Option<&'static User> {
    // For returning owned data instead, see note below
    USER_STORAGE
        .lock()
        .unwrap()
        .iter()
        .find(|u| u.id == id)
        .map(|u| u as *const User)
        .map(|ptr| unsafe { &*ptr }) // safe: static storage lives forever
}

pub fn get_all_users() -> Vec<User> {
    USER_STORAGE.lock().unwrap().clone()
}

pub fn update_user(id: u32, name: Option<String>, email: Option<String>) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    let user = storage
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or(format!("User with id {} not found", id))?;

    if let Some(name) = name {
        user.name = name;
    }
    if let Some(email) = email {
        user.email = email;
    }
    Ok(())
}

pub fn delete_user(id: u32) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    let pos = storage
        .iter()
        .position(|u| u.id == id)
        .ok_or(format!("User with id {} not found", id))?;

    storage.remove(pos);
    Ok(())
}
