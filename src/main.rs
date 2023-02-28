#[macro_use]
extern crate rocket;

mod domain;
mod infrastructure;
mod usecase;

use std::sync::Arc;

use domain::repository::user_repo::UserRepository;
use infrastructure::{controller, repository};

pub struct Repositories {
    pub user_repo: Arc<dyn domain::repository::user_repo::UserRepository>,
}

#[launch]
async fn rocket() -> _ {
    let user_repo: Arc<dyn UserRepository> =
        Arc::new(repository::user_inmemory_repo::UserInMemoryRepository::new());

    rocket::build().manage(user_repo).mount(
        "/users",
        routes![
            controller::create_user_ctrl::execute,
            controller::get_all_users_ctrl::execute
        ],
    )
}
