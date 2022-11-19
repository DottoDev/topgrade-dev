#![allow(dead_code)]
use crate::executor::RunType;
use crate::git::Git;
use crate::utils::require_option;
use crate::{config::Config, executor::Executor};
use anyhow::Result;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub struct ExecutionContext<'a> {
    run_type: RunType,
    sudo: &'a Option<PathBuf>,
    git: &'a Git,
    config: &'a Config,
    base_dirs: &'a BaseDirs,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(
        run_type: RunType,
        sudo: &'a Option<PathBuf>,
        git: &'a Git,
        config: &'a Config,
        base_dirs: &'a BaseDirs,
    ) -> ExecutionContext<'a> {
        ExecutionContext {
            run_type,
            sudo,
            git,
            config,
            base_dirs,
        }
    }
    #[cfg(test)]
    pub fn new_test_ctx() -> ExecutionContext<'a> {
        use crate::config::CommandLineArgs;
        use crate::execution_context;
        use crate::steps::git;
        use crate::{executor, utils};

        let run_type = executor::RunType::new(false);
        let sudo = utils::sudo();
        let git = git::Git::new();
        let opt = CommandLineArgs::new();
        let config = Config::new(opt).unwrap();
        let base_dirs = BaseDirs::new().unwrap();
        let ctx = execution_context::ExecutionContext::new(run_type, &sudo, &git, &config, &base_dirs);
        todo!()
    }

    pub fn execute_elevated(&self, command: &Path, interactive: bool) -> Result<Executor> {
        let sudo = require_option(self.sudo.clone(), "Sudo is required for this operation".into())?;
        let mut cmd = self.run_type.execute(&sudo);

        if sudo.ends_with("sudo") {
            cmd.arg("--preserve-env=DIFFPROG");
        }

        if interactive {
            cmd.arg("-i");
        }

        cmd.arg(command);
        Ok(cmd)
    }

    pub fn run_type(&self) -> RunType {
        self.run_type
    }

    pub fn git(&self) -> &Git {
        self.git
    }

    pub fn sudo(&self) -> &Option<PathBuf> {
        self.sudo
    }

    pub fn config(&self) -> &Config {
        self.config
    }

    pub fn base_dirs(&self) -> &BaseDirs {
        self.base_dirs
    }
}
