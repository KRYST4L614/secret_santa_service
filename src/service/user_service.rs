use crate::models::user::User;

pub struct UserServiceImpl {}

pub trait UserService {
    fn create_user(name: String);
    fn get_user_by_name(name: String) -> Option<User>;
}


impl UserService for UserServiceImpl {
    fn create_user(name: String) {}

    fn get_user_by_name(name: String) -> Option<User> {
        None
    }
}
