// SPDX-License-Identifier: GPL-3.0-only

use std::{
    ffi::OsStr,
    io,
    process::{Command, ExitStatus, Stdio},
    str::{self, Utf8Error},
};

pub struct Cmd {
    cmd: Command,
}
impl Cmd {
    pub fn new(bin: &str) -> Self {
        let cmd = Command::new(bin);
        Self { cmd }
    }

    pub fn arg<S>(mut self, arg: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        self.cmd.arg(arg);
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd.args(args);
        self
    }

    pub fn hide_output(mut self) -> Self {
        self.cmd.stderr(Stdio::null()).stdout(Stdio::null());
        self
    }

    pub fn pipe_stderr(mut self) -> Self {
        self.cmd.stderr(std::io::stderr());
        self
    }

    pub fn _execute(&mut self) -> std::result::Result<ExitStatus, io::Error> {
        self.cmd.spawn()?.wait()
    }

    pub fn execute(mut self) -> Result<ExitStatus> {
        self._execute().map_execute_err(&self.cmd)
    }

    pub fn execute_and_grub_output(mut self) -> Result<String> {
        let output = self.cmd.output().map_execute_err(&self.cmd)?;

        let string =
            str::from_utf8(&output.stdout).map_err(|source| Error::parse(&self.cmd, source))?;

        if !output.status.success() {
            return Err(Error::ended_with_non_zero(&self.cmd, output.status));
        }

        Ok(string.trim().to_owned())
    }

    pub fn execute_and_grub_lines(self) -> Result<Vec<String>> {
        Ok(self
            .execute_and_grub_output()?
            .split("\n")
            .map(ToOwned::to_owned)
            .collect())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{command_name}: Failed to execute program: {source}")]
    Execute {
        source: io::Error,
        command_name: String,
    },
    #[error("{command_name}: Parse output failed: {source}")]
    Parse {
        source: Utf8Error,
        command_name: String,
    },
    #[error("{command_name}: Command ended with error: {exit_status}")]
    EndedWithNonZero {
        exit_status: ExitStatus,
        command_name: String,
    },
}

trait CommandName {
    fn name(&self) -> String;
}

impl CommandName for Command {
    fn name(&self) -> String {
        self.get_program().to_string_lossy().to_string()
    }
}

impl Error {
    fn execute(command: &Command, source: io::Error) -> Self {
        let command_name = command.name();
        Self::Execute {
            source,
            command_name,
        }
    }

    fn parse(command: &Command, source: Utf8Error) -> Self {
        let command_name = command.name();
        Self::Parse {
            command_name,
            source,
        }
    }

    fn ended_with_non_zero(command: &Command, exit_status: ExitStatus) -> Self {
        let command_name = command.name();
        Self::EndedWithNonZero {
            exit_status,
            command_name,
        }
    }
}

trait ErrorExtension<T> {
    fn map_execute_err(self, command: &Command) -> std::result::Result<T, Error>;
}

impl<T> ErrorExtension<T> for std::result::Result<T, io::Error> {
    fn map_execute_err(self, command: &Command) -> std::result::Result<T, Error> {
        self.map_err(|source| Error::execute(command, source))
    }
}

type Result<T> = std::result::Result<T, Error>;
