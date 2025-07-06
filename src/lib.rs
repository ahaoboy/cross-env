use std::env;
use std::process::Command;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("key is empty")]
    EmptyKey,
    #[error("command error")]
    CommandError,
}

pub fn parse_input(args: &[String]) -> (Vec<(&str, &str)>, usize) {
    let mut idx = 0;
    let mut v = vec![];
    for i in args {
        if let Some(pair) = i.split_once('=') {
            v.push(pair);
            idx += 1;
        } else {
            break;
        }
    }
    (v, idx)
}

pub fn run(cmd: &str, cmd_args: &[String], envs: &Vec<(&str, &str)>) -> Result<(), Error> {
    let mut cmd = Command::new(cmd);

    for (key, value) in envs {
        if key.is_empty() {
            return Err(Error::EmptyKey);
        }
        cmd.env(key, value);
    }

    cmd.stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    cmd.args(cmd_args);

    cmd.output()?;
    Ok(())
}

pub fn cross_env() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let (envs, idx) = parse_input(&args);

    let cmd = &args[idx];
    let cmd_args = &args[idx + 1..];
    run(cmd, cmd_args, &envs)
}
