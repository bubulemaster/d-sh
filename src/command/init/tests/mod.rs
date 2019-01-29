///
/// Module to tests module init.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use config::{create_config_filename_path, get_config_filename};
use super::{init, INIT};
use docker::tests::TestContainerHelper;
use std::path::Path;
use std::collections::HashMap;
use command::CommandExitCode;
use download::tests::TestDownloadHelper;

#[test]
fn unable_to_create_configfile_if_exists() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            // Create file
            io_helper.files.borrow_mut().insert(cfg_file, String::from("toto"))
        },
        None => panic!("Unable to get config filename for test")
    };

    let result = init(&INIT, &args, io_helper, dck_helper, dl_helper, None);

    assert_eq!(result, CommandExitCode::ConfigFileExits);
}

#[test]
fn create_configfile_if_not_exists() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    let result = init(&INIT, &args, io_helper, dck_helper, dl_helper, None);

    assert_eq!(result, CommandExitCode::Ok);

    match get_config_filename() {
        Some(cfg_file) => {
            let f = io_helper.files.borrow_mut();
            let v = f.get(&cfg_file);

            match v {
                Some(c) => assert_eq!(c, &format!("---\ndownload_dir: \"toto\"\napplications_dir: \"titi\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n")),
                None => panic!("The config file was not created")
            };
        },
        None => panic!("Unable to get config filename for test")
    };

    let f = io_helper.files.borrow_mut();

    let dockerfile_list: HashMap<&str, &str> = [
        (super::DOCKERFILE_BASE_FILENAME, super::DOCKERFILE_BASE),
        (super::ENTRYPOINT_FILENAME, super::ENTRYPOINT)]
        .iter().cloned().collect();

    // Create all docker file
    for (filename, content) in &dockerfile_list {
        match create_config_filename_path(filename) {
            Some(dockerfile_name) => {
                let v = f.get(&dockerfile_name);

                match v {
                    Some(c) => assert_eq!(c, content),
                    None => panic!(format!("The dockerfile {} file was not created", filename))
                };

            },
            None => panic!("Unable to get your home dir!")
        }
    }
}

#[test]
fn create_configfile_but_cannot_write() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            io_helper.files_error.borrow_mut().insert(cfg_file, true);
        },
        None => panic!("Unable to get config filename for test")
    };

    let result = init(&INIT, &args, io_helper, dck_helper, dl_helper, None);

    assert_eq!(result, CommandExitCode::CannotWriteConfigFile);
}

#[test]
fn create_configfile_but_cannot_create_parent_folder() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            let path = Path::new(&cfg_file);

            if let Some(parent) = path.parent() {
                io_helper.files_error.borrow_mut().insert(String::from(parent.to_str().unwrap()), true);
            }
        },
        None => panic!("Unable to get config filename for test")
    };

    let result = init(&INIT, &args, io_helper, dck_helper, dl_helper, None);

    assert_eq!(result, CommandExitCode::CannotCreateFolderForConfigFile);
}
