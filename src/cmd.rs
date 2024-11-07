use std::{error::Error, fmt::Display, io, process::Command};

pub fn execute(cmd: &mut Command) -> Result<(), RunProgramError> {
    let mut child = cmd
        .spawn()
        .map_err(|source| RunProgramError::new(cmd, source))?;
    child
        .wait()
        .map_err(|source| RunProgramError::new(cmd, source))?;
    Ok(())
}

pub fn execute_without_output(cmd: &mut Command) -> Result<(), RunProgramError> {
    cmd.output()
        .map_err(|source| RunProgramError::new(cmd, source))?;
    Ok(())
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
        const ERROR: &str = "failed to run program";
        if let Some(program) = &self.command_name {
            write!(f, "{program}:")?;
        }
        write!(f, "{ERROR}: {source}", source = self.source)
    }
}
impl Error for RunProgramError {}
