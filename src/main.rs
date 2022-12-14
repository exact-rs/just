mod cli;
mod go;
mod helpers;
mod loader;
mod logger;
mod macros;
mod ops;
mod project;
mod registry;
mod runtime;

use clap::{Parser, Subcommand};
use exact_panic::setup_panic;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Print version information
    #[arg(short, long)]
    version: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup for executing external modules
    Setup,
    /// Initialize a new package.yml
    Init,
    /// Save an auth token for the registry locally
    Login,
    /// Remove the local auth token for the registry
    Logout,
    /// Verify account on registry to publish
    Verify,
    /// Package and upload this package to the registry
    Publish,
    /// Install all dependencies defined in package.yml
    Install,
    /// Add a new dependency
    Add {
        #[command()]
        name: String,
    },
    /// Remove a dependency
    Remove {
        #[command()]
        name: String,
    },
    /// Remove unused dependencies
    Clean,
    /// Initialize a new project
    Create,
    /// Run a task defined in project.yml
    Task {
        #[command()]
        task: String,
    },
    /// List all tasks in project.yml
    Tasks,
    /// Run a test defined in project.yml
    Test {
        #[command()]
        test: String,
    },
    /// List all tests in project.yml
    Tests,
    /// Start the index script
    Start {
        #[arg(short, long)]
        silent: bool,
    },
    /// Eval a JavaScript string
    Eval {
        #[arg(short, long)]
        silent: bool,
        #[command()]
        code_content: String,
    },
    /// Run a JavaScript program
    Run {
        #[arg(short, long)]
        silent: bool,
        #[command()]
        file_name: String,
    },
    /// Static file serving
    Serve {
        #[clap(short, long, default_value_t = String::from("localhost"))]
        address: String,
        #[clap(short, long, default_value_t = 3000)]
        port: u64,
        #[command()]
        dir_name: String,
    },
}

fn main() {
   setup_panic!();
   
    let cli = Cli::parse();

    if cli.version {
        println!("{}", cli::get_version(false));
        std::process::exit(0);
    }

    match &cli.command {
        /* essentials */
        Some(Commands::Setup) => cli::setup(),
        Some(Commands::Init) => cli::create_project_yml(),
        Some(Commands::Create) => project::create::download_template(),

        /* registry */
        Some(Commands::Login) => registry::auth::login(),
        Some(Commands::Logout) => registry::auth::logout(),
        Some(Commands::Verify) => registry::auth::verify(),
        Some(Commands::Publish) => registry::package::publish(),

        /* task runner */
        Some(Commands::Tasks) => cli::list_tasks(),
        Some(Commands::Task { task }) => cli::run_task(task),

        /* misc */
        Some(Commands::Serve { address, port, dir_name }) => cli::serve(port, address, dir_name),

        /* testing */
        Some(Commands::Test { test }) => cli::run_test(test),
        Some(Commands::Tests) => cli::list_tests(),

        /* package management */
        Some(Commands::Clean) => registry::manager::clean(),
        Some(Commands::Install) => registry::manager::install(),
        Some(Commands::Add { name }) => registry::manager::add(name, true),
        Some(Commands::Remove { name }) => registry::manager::remove(name),

        /* runtime */
        Some(Commands::Run { silent, file_name }) => cli::run_exec(file_name, *silent, ""),
        Some(Commands::Eval { silent, code_content }) => cli::run_exec("", *silent, code_content),
        Some(Commands::Start { silent }) => cli::run_exec(&project::package::read().info.index, *silent, ""),

        None => cli::run_repl(),
    }
}
