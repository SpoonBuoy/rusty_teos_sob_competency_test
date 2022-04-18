//sample code for storing, loading, and updating USERS
use crate::dbm::DBM;
use std::collections::{HashMap};
let db = DBM{};
db.new();
db.store_user(user_id, user_info);
let mut all_users = HashMap::new();
all_users = db.load_all_users();
db.update_user(user_id, user_info);

