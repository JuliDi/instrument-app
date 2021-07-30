use crate::devices::Command;

/// Returns an SCPI command as string from all available arguments
/// In the future, this will error if a required argument is missing or not available
pub fn make_scpi_command(cmd: Command, channel: u8, arg: &str, freetext: &str) -> Result<String, String> {
    let mut scpi = cmd.scpi.replace("<CH>", &*channel.to_string());
    scpi.push_str(arg);
    scpi = scpi.replace("<TXT>", &*freetext.to_string());
    scpi.push_str("\n");

    return Ok(scpi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_scpi_command() {

        let cmd = Command {
            channel: true,
            name: "TestCommand".to_string(),
            scpi: "OUT<CH> ".into(),
            values: vec!["on".into(), "off".into()]
        };

        let res = make_scpi_command(cmd, 1, "on".into());
        println!("{}", res.unwrap());

    }
}