use std::{fs::File, io::Write, path::Path};

use anyhow::{Context, Result};

use crate::app;
use crate::manifest;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("failed to initialize because {0}")]
    InitError(#[from] InitError),
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("already exist manifest file")]
    AlreadyExist,

    #[error("failed to serialize manifest")]
    SerializeError(#[from] toml::ser::Error),

    #[error("failed to create manifest file")]
    FileError(#[from] std::io::Error),

    #[error("unexpected application error occurred : {0}")]
    Crash(#[from] anyhow::Error),
}

pub struct Core {
    term: console::Term,
}

impl Core {
    pub fn new() -> Self {
        Self {
            term: console::Term::stdout(),
        }
    }

    pub fn execute(&self, args: app::Cli) -> Result<()> {
        let cmd = args.command;

        let result = match cmd {
            app::Command::Init => self.init().map_err(CommandError::from),
            app::Command::Apply { manifest_path: _ } => unimplemented!(),
        };

        if let Err(err) = result {
            self.term
                .write_line(&format!("{}", err))
                .context("failed to write line")?;
        }
        Ok(())
    }

    fn init(&self) -> core::result::Result<(), InitError> {
        // TODO: 絶対パスでファイル作成を行うように変更する
        let manifest_path = Path::new("manifest.toml");

        if manifest_path.exists() {
            return Err(InitError::AlreadyExist);
        }

        let manifest = manifest::new_boilerplate();
        let content = toml::to_string_pretty(&manifest)?;
        let mut file = File::create(manifest_path)?;

        content
            .lines()
            .try_for_each(|line| writeln!(file, "# {}", line))?;
        file.flush()?;

        let msg = format!("Created manifest file in {}", manifest_path.display());
        self.term.write_line(&msg).context("failed to write line")?;
        Ok(())
    }
}
