// SPDX-License-Identifier: GPL-3.0-only

use std::ffi::OsStr;
use std::io;
use std::process::{Command, ExitStatus, Stdio};
use std::str::{self, Utf8Error};

use derive_more::Display;

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

    pub fn execute_(&mut self) -> std::result::Result<ExitStatus, io::Error> {
        self.cmd.spawn()?.wait()
    }

    pub fn execute(mut self) -> Result<ExitStatus> {
        self.execute_()
            .map_err(|source| Error::execute(&self.cmd, source))
    }

    pub fn execute_and_grub_output(mut self) -> Result<String> {
        let output = self
            .cmd
            .output()
            .map_err(|source| Error::execute(&self.cmd, source))?;

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
            .split('\n')
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{command_name}: {kind}")]
pub struct Error {
    pub command_name: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    #[display("Failed to execute program: {source}")]
    Execute { source: io::Error },
    #[display("Parse output failed: {source}")]
    Parse { source: Utf8Error },
    #[display("Command ended with error: {exit_status}")]
    EndedWithNonZero { exit_status: ExitStatus },
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
        Self {
            command_name: command.name(),
            kind: ErrorKind::Execute { source },
        }
    }

    fn parse(command: &Command, source: Utf8Error) -> Self {
        Self {
            command_name: command.name(),
            kind: ErrorKind::Parse { source },
        }
    }

    fn ended_with_non_zero(command: &Command, exit_status: ExitStatus) -> Self {
        Self {
            command_name: command.name(),
            kind: ErrorKind::EndedWithNonZero { exit_status },
        }
    }
}

type Result<T> = std::result::Result<T, Error>;
