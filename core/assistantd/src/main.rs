mod agent;
mod app;
mod config;
mod context;
mod inference;
mod identity;
mod permissions;
mod request_context;
mod ipc;
mod storage;
mod task_analyzer;

use app::Application;
use config::AppConfig;

fn main() {
    let config = AppConfig::default();

    if std::env::args().any(|arg| arg == "--ipc") {
        if let Err(error) = ipc::server::run(&config) {
            eprintln!("IPC server error: {error}");
            std::process::exit(1);
        }
        return;
    }

    let application = match Application::new(config) {
        Ok(application) => application,
        Err(error) => {
            eprintln!("Startup error: {error}");
            std::process::exit(1);
        }
    };

    application.run();
}
