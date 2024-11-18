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
            .run_err_map(&self.cmd)?
            .wait()
            .run_err_map(&self.cmd)
    }

    pub fn execute_and_grub_output(mut self) -> anyhow::Result<String> {
        let output = self
            .cmd
            .stderr(std::io::stderr())
            .output()
            .run_err_map(&self.cmd)?;
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

trait IoResultExt<T> {
    fn run_err_map(self, cmd: &Command) -> Result<T, RunProgramError>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn run_err_map(self, cmd: &Command) -> Result<T, RunProgramError> {
        self.map_err(|source| RunProgramError::new(cmd, source))
    }
}

#[derive(Debug)]
pub struct RunProgramError {
    command_name: String,
    source: io::Error,
}
impl RunProgramError {
    fn new(command: &Command, source: io::Error) -> Self {
        let command_name = command.get_program().to_string_lossy().to_string();
        Self {
            command_name,
            source,
        }
    }
}
impl Display for RunProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{program}: Failed to run program: {source}",
            program = self.command_name,
            source = self.source
        )
    }
}
impl Error for RunProgramError {}
