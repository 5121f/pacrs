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

    pub fn execute(mut self) -> Result<ExitStatus, RunProgramError> {
        self.cmd
            .spawn()
            .map_err(|source| RunProgramError::new(&self.cmd, source))?
            .wait()
            .map_err(|source| RunProgramError::new(&self.cmd, source))
    }

    pub fn execute_and_grub_output(mut self) -> anyhow::Result<String> {
        let output = self
            .cmd
            .stderr(std::io::stderr())
            .output()
            .map_err(|source| RunProgramError::new(&self.cmd, source))?;
        let output = str::from_utf8(&output.stdout).with_context(|| {
            format!(
                "{}: Failed to take command output",
                &self.cmd.get_program().to_str().unwrap_or_default()
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

#[derive(Debug)]
pub struct RunProgramError {
    command_name: Option<String>,
    source: io::Error,
}
impl RunProgramError {
    fn new(command: &Command, source: io::Error) -> Self {
        let command_name = command.get_program().to_str().map(ToOwned::to_owned);
        Self {
            command_name,
            source,
        }
    }
}
impl Display for RunProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(program) = &self.command_name {
            write!(f, "{program}: ")?;
        }
        write!(f, "Failed to run program: {source}", source = self.source)
    }
}
impl Error for RunProgramError {}
