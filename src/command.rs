use std::{
    error::Error,
    ffi::OsStr,
    fmt::Display,
    io,
    process::{Command, ExitStatus, Stdio},
    str,
};

use anyhow::Context;

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

    pub fn execute(mut self) -> Result<ExitStatus, ExeProgramError> {
        self.cmd
            .spawn()
            .exe_err(&self.cmd)?
            .wait()
            .exe_err(&self.cmd)
    }

    pub fn execute_and_grub_output(mut self) -> anyhow::Result<String> {
        let output = self
            .cmd
            .stderr(std::io::stderr())
            .output()
            .exe_err(&self.cmd)?;
        let output = str::from_utf8(&output.stdout).with_context(|| {
            format!(
                "{}: Failed to take command output",
                &self.cmd.get_program().to_string_lossy()
            )
        })?;
        Ok(output.trim().to_owned())
    }

    pub fn execute_and_grub_lines(self) -> anyhow::Result<Vec<String>> {
        Ok(self
            .execute_and_grub_output()?
            .split("\n")
            .map(ToOwned::to_owned)
            .collect())
    }
}

trait IoResultExt<T> {
    fn exe_err(self, cmd: &Command) -> Result<T, ExeProgramError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn exe_err(self, cmd: &Command) -> Result<T, ExeProgramError> {
        self.map_err(|source| ExeProgramError::new(cmd, source))
    }
}

#[derive(Debug)]
pub struct ExeProgramError {
    command_name: String,
    source: io::Error,
}
impl ExeProgramError {
    fn new(command: &Command, source: io::Error) -> Self {
        let command_name = command.get_program().to_string_lossy().to_string();
        Self {
            command_name,
            source,
        }
    }
}
impl Display for ExeProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{program}: Failed to execute program: {source}",
            program = self.command_name,
            source = self.source
        )
    }
}
impl Error for ExeProgramError {}
