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

    pub fn _execute(&mut self) -> Result<ExitStatus, io::Error> {
        self.cmd.spawn()?.wait()
    }

    pub fn execute(mut self) -> Result<ExitStatus, ExeProgramError> {
        self._execute()
            .map_err(|source| ExeProgramError::new(&self.cmd, source))
    }

    pub fn execute_and_grub_output(mut self) -> anyhow::Result<(ExitStatus, String)> {
        let output = self
            .cmd
            // .stderr(std::io::stderr())
            .output()
            .map_err(|source| ExeProgramError::new(&self.cmd, source))?;
        let string = str::from_utf8(&output.stdout).with_context(|| {
            format!(
                "{}: Failed to take command output",
                &self.cmd.get_program().to_string_lossy()
            )
        })?;
        Ok((output.status, string.trim().to_owned()))
    }

    pub fn execute_and_grub_lines(self) -> anyhow::Result<(ExitStatus, Vec<String>)> {
        let (status, output) = self.execute_and_grub_output()?;
        let lines = output.split("\n").map(ToOwned::to_owned).collect();
        Ok((status, lines))
    }

    pub fn execute_and_grub_lines_ignore_status(self) -> anyhow::Result<Vec<String>> {
        let (_, lines) = self.execute_and_grub_lines()?;
        Ok(lines)
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
