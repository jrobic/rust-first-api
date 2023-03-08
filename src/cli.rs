#[macro_use]
extern crate rocket;

mod domain;
mod infrastructure;
mod usecase;

use crossterm::terminal::enable_raw_mode;
use dotenv::dotenv;
use std::{io, sync::Arc};
use tui::{backend::CrosstermBackend, Terminal};

use domain::repository::user_repo::UserRepository;
use infrastructure::cli;
use infrastructure::repository::user_supabase_repo;

pub struct Repositories {
    pub user_repo: Arc<dyn domain::repository::user_repo::UserRepository>,
}

fn main() -> Result<(), io::Error> {
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

    let stdout = io::stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|rect| cli::ui::draw(rect))?;
    }

    Ok(())
}
