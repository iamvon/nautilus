use std::process::{ Command, Stdio };

pub fn get_cli_configs() -> (&'static str, &'static str) {
    let (mut cmd_shell, mut cmd_prefix) = ("sh", "-c");
    if cfg!(target_os = "windows") {
        (cmd_shell, cmd_prefix) = ("cmd", "/C");
    };
    return (cmd_shell, cmd_prefix)
}

pub fn execute_command(args: &str) {
    let (cmd_shell, cmd_prefix) = get_cli_configs();
    let mut cmd = Command::new(cmd_shell)
        .args([cmd_prefix, args])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command.");
    cmd.wait().unwrap();
}