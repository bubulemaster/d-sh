///
/// Module to build one image.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use command::{CommandError, CommandExitCode, CommandParameter};
use command::build::{BuildOptions, generate_dockerfile};
use command::build::dockerfile::DockerfileParameter;
use io::convert_path;
use config::{Config, get_filename, get_config_application, ConfigApplication};
use std::error::Error;

///
/// Download file with curl.
///
fn download_file(cmd_param: &CommandParameter, app: &str,
    config_application: &ConfigApplication, options: &BuildOptions,
    config: &Config) -> Result<(), CommandError> {
    // Check if file already downloaded
    let app_dwn_filename = get_filename(&config.download_dir,
        &config_application.download_filename, None);
    let app_dwn_filename = convert_path(&app_dwn_filename);

    if cmd_param.io_helper.file_exits(&app_dwn_filename) {
        // Download file with curl
        if ! options.skip_redownload
            && ! cmd_param.dl_helper.download_if_update(&config_application.url,
                &app_dwn_filename) {
            return Err(CommandError {
                msg: vec![format!("Unable to download application '{}'!", app)],
                code: CommandExitCode::UnableDownloadApplication
            });
        }
    } else {
        // Download file with curl
        if ! cmd_param.dl_helper.download(&config_application.url, &app_dwn_filename) {
            return Err(CommandError {
                msg: vec![format!("Unable to download application '{}'!", app)],
                code: CommandExitCode::UnableDownloadApplication
            });
        }
    }

    Ok(())
}

///
/// Build one application.
///
/// Return false if application build fail.
///
pub fn build_one_application(cmd_param: &CommandParameter, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config, app: &str) -> Result<(), CommandError> {

    let app_filename = convert_path(&get_filename(&config.applications_dir, app, Some(&".yml")));

    let dockerfile = DockerfileParameter::new(tmp_dir);

    let config_application;

    match get_config_application(cmd_param.io_helper, &app_filename) {
        Ok(r) => config_application = r,
        Err(err) => return Err(CommandError {
                msg: vec![
                    format!("Unable to find application '{}' or something is wrong in file! {}",
                        app, err.description())
                    ],
                code: CommandExitCode::ApplicationFileNotFound
            })
    }

    if let Err(err) = download_file(cmd_param, app, &config_application, options, config) {
        return Err(err);
    }

    // Now build
    let data = json!({
        "dockerfile_from": config.dockerfile.tag.to_owned(),
        "dockerfile_base": false,
        "application_filename": config_application.download_filename.to_owned()
    });

    if let Err(err) = generate_dockerfile(cmd_param.io_helper, &dockerfile.docker_filename,
        &data) {
        return Err(err);
    }

    // Copy file to temporary folder
    let app_dwn_filename = convert_path(&get_filename(&config.download_dir,
        &config_application.download_filename, None));

    if let Err(err) = cmd_param.io_helper.hardlink_or_copy_file(&app_dwn_filename,
        &format!("{}/{}", &dockerfile.docker_context_path, &config_application.download_filename)) {
        return Err(CommandError {
            msg: vec![
                format!("Unable copy '{}' to '{}'!", &app_dwn_filename,
                    &dockerfile.docker_context_path),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotCopyFile
        });
    }

    // Build
    let mut build_args = Vec::new();

    if options.force {
        build_args.push(String::from("--no-cache"));
    }

    if ! cmd_param.dck_helper.build_image(&dockerfile.docker_filename,
        &dockerfile.docker_context_path, &config_application.image_name,
        Some(&build_args)) {
        return Err(CommandError {
            msg: vec![format!("Cannot build application {}!", app)],
            code: CommandExitCode::DockerBuildFail
        });
    }

    return Ok(());
}
