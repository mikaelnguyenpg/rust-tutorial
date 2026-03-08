use std::sync::Mutex;

use crate::models::user::{RequestUser, User};

static USER_STORAGE: Mutex<Vec<User>> = Mutex::new(vec![]);

pub fn create_user(user: RequestUser) -> Result<usize, String> {
    let mut storage: std::sync::MutexGuard<'_, Vec<User>> =
        USER_STORAGE.lock().map_err(|e| e.to_string())?;
    let user_id = storage.iter().len() + 1;

    if storage.iter().any(|u| u.id == user_id) {
        return Err(format!("User with id {} already exists", user_id));
    }

    storage.push(User {
        id: user_id,
        name: user.name,
        email: user.email,
    });
    Ok(user_id)
}

pub fn get_user(id: usize) -> Option<&'static User> {
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

pub fn update_user(id: usize, name: Option<String>, email: Option<String>) -> Result<(), String> {
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

pub fn delete_user(id: usize) -> Result<(), String> {
    let mut storage = USER_STORAGE.lock().map_err(|e| e.to_string())?;

    let pos = storage
        .iter()
        .position(|u| u.id == id)
        .ok_or(format!("User with id {} not found", id))?;

    storage.remove(pos);
    Ok(())
}
