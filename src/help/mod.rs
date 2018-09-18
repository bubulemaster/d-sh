///
/// Module to display help.
///
/// Release under MIT License.
///
use super::command::Command;
use super::io::InputOutputHelper;

///
/// Display help of D-SH.
///
/// `commands` parameter is list of all avaible command
///
pub fn help(commands: &[Command], io_helper: &mut InputOutputHelper) {
    io_helper.println(&format!(""));
    io_helper.println(&format!("Usage: d-sh COMMAND"));
    io_helper.println(&format!(""));
    io_helper.println(&format!("A tool to container all your life"));
    io_helper.println(&format!(""));
    io_helper.println(&format!("Options:"));
    io_helper.println(&format!("  -h, --help               Print this current help"));
    io_helper.println(&format!("  -v, --version            Print version information and quit"));
    io_helper.println(&format!(""));
    io_helper.println(&format!("Commands:"));

    for cmd in commands {
        io_helper.println(&format!("  {:<width$}{}", cmd.name, cmd.description, width = 9));
    }
}

///
/// Display current version.
///
/// `args` parameter is command line arguments of D-SH.
///
pub fn version(args: &[String], io_helper: &mut InputOutputHelper) {
    let version = env!("CARGO_PKG_VERSION");

    io_helper.println(&format!("{} version {}", args[0], version));
    io_helper.println(&format!("Copyleft Emeric MARTINEAU (c) 2018"));
}

#[cfg(test)]
mod tests {
    use super::super::io::InputOutputHelper;
    use super::super::io::tests::TestInputOutputHelper;
    use super::version;
    use super::help;
    use super::super::command::Command;

    #[test]
    fn display_version() {
        let io_helper = &mut TestInputOutputHelper::new();

        let args = [String::from("ttt")];

        version(&args, io_helper);

        assert_eq!(io_helper.stdout.len(), 2);
    }

    fn test_help(_command: &Command, _args: &[String], io_helper: &mut InputOutputHelper) -> i32 {
        io_helper.println(&format!("Coucou !"));
        0
    }

    #[test]
    fn display_help() {
        let io_helper = &mut TestInputOutputHelper::new();

        let one_cmd = Command {
            name: "test",
            description: "It's a test",
            short_name: "tst",
            min_args: 0,
            max_args: 0,
            usage: "",
            need_config_file: false,
            exec_cmd: test_help
        };

        let commands = &[one_cmd];

        help(commands, io_helper);

        assert_eq!(io_helper.stdout.len(), 11);

        match io_helper.stdout.get(10) {
            Some(s) => assert_eq!(s, "  test     It's a test"),
            None => panic!("Help is not valid")
        }
    }
}
