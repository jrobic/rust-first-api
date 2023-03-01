#[macro_use]
extern crate rocket;

mod domain;
mod infrastructure;
mod usecase;

use dotenv::dotenv;
use std::sync::Arc;

use domain::repository::user_repo::UserRepository;
use infrastructure::{controller, repository::user_supabase_repo};

pub struct Repositories {
    pub user_repo: Arc<dyn domain::repository::user_repo::UserRepository>,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let api_url = dotenv::var("SUPABASE_API_URL").expect("SUPABASE_API_URL must be set");
    let api_key = dotenv::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

    // let user_repo: Arc<dyn UserRepository> =
    // Arc::new(repository::user_inmemory_repo::UserInMemoryRepository::new());

    let user_supabase_repo: Arc<dyn UserRepository> = Arc::new(
        user_supabase_repo::UserSupabaseRepository::new(api_url, api_key),
    );

    let repositories = Arc::new(Repositories {
        user_repo: user_supabase_repo,
    });

    rocket::build()
        .manage(repositories)
        .mount(
            "/users",
            routes![
                controller::users_ctrl::create_user_ctrl,
                controller::users_ctrl::get_all_users_ctrl,
                controller::users_ctrl::get_user_ctrl,
                controller::users_ctrl::update_user_ctrl,
                controller::users_ctrl::remove_user_ctrl,
            ],
        )
        .register(
            "/",
            catchers![
                controller::catchers_ctrl::not_found_ctrl,
                controller::catchers_ctrl::catch_default_ctrl
            ],
        )
}
