use crate::models::group::Group;

pub struct GroupServiceImpl {
}

pub trait GroupService {
    fn find_group_by_name(name: String) -> Option<Group>;
    fn is_user_in_group(username: String, group_name: String) -> bool;
    fn is_admin(username: String, group_name: String) -> bool;
    fn count_admins(group_name: String) -> usize;
}

impl GroupService for GroupServiceImpl {
    fn find_group_by_name(name: String) -> Option<Group> {
        None
    }

    fn is_user_in_group(username: String, group_name: String) -> bool {
        false
    }

    fn is_admin(username: String, group_name: String) -> bool {
        false
    }

    fn count_admins(group_name: String) -> usize {
        0
    }
}
