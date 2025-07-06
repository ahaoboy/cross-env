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

pub fn parse_input<T: AsRef<str>>(args: &[T]) -> (Vec<(&str, &str)>, usize) {
    let mut idx = 0;
    let mut v = vec![];
    for i in args {
        if let Some(pair) = i.as_ref().split_once('=') {
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

#[cfg(test)]
mod test {
    use crate::parse_input;

    #[test]
    fn test_parse_input() {
        for (args, expect_envs, expect_idx) in [
            (vec!["a=1"], vec![("a", "1")], 1),
            (vec!["a=1", "b=2"], vec![("a", "1"), ("b", "2")], 2),
        ] {
            let (envs, idx) = parse_input(&args);
            assert_eq!(envs, expect_envs);
            assert_eq!(idx, expect_idx);
        }
    }
}
