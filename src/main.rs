use clap::Parser;

#[cfg(test)]
mod manifest_test;

mod app;
mod core;
mod manifest;

fn main() {
    let args = app::Cli::parse();
    let core = core::Core::new();
    let result = core.execute(args);

    // TODO: エラーの種類によって終了コードを切り替える
    if result.is_err() {
        std::process::exit(1);
    }
}
