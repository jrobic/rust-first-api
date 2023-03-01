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

    let repositories = Arc::new(Repositories { user_repo });

    rocket::build().manage(repositories).mount(
        "/users",
        routes![
            controller::users_ctrl::create_user_ctrl,
            controller::users_ctrl::get_all_users_ctrl,
            controller::users_ctrl::get_user_ctrl,
            controller::users_ctrl::update_user_ctrl,
        ],
    )
}
