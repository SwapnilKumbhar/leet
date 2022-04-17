// Perform the copying, templating etc.

use log::{error, info};
use std::fs::copy;
use std::path::{Path, PathBuf};

use crate::lconfig::LanguageTemplate;

use crate::error::ActionError;

fn copy_files(
    template: &LanguageTemplate,
    project_dir: &String,
    tdir_path: &String,
) -> Result<(), ActionError> {
    for file in template.files.iter() {
        let src = PathBuf::from(tdir_path).join(file);
        let dst = PathBuf::from(project_dir).join(file);
        info!(
            "Copy: {} -> {} ",
            src.to_str().unwrap_or("<Unknown>"),
            dst.to_str().unwrap_or("<Unknown>")
        );
        match copy(src.clone(), dst.clone()) {
            Ok(_) => (),
            Err(e) => {
                error!("Copy failed - {}", e);
                return Err(ActionError::CopyError {
                    src: src.to_str().unwrap_or("<Couldn't find src>").to_string(),
                    dst: dst.to_str().unwrap_or("<Couldn't decode dst>").to_string(),
                });
            }
        };
    }
    Ok(())
}

pub fn doit(
    choice: &String,
    template: &LanguageTemplate,
    tdir_path: &String,
    project_name: String,
) -> Result<(), ActionError> {
    // Ensure that the target project directory exists, else create it
    let cwd = match std::env::current_dir()?.into_os_string().into_string() {
        Ok(cwd) => cwd,
        Err(_) => {
            error!("Failed to convert CWD path to a string.");
            return Err(ActionError::OsStringError {});
        }
    };

    let project_dir = match PathBuf::from(cwd).join(project_name).to_str() {
        Some(s) => s.to_string(),
        None => {
            error!("Failed to convert Project Directory's path to a string");
            return Err(ActionError::GenericError {
                msg: "Couldn't convert PathBuf to str".to_string(),
            });
        }
    };

    info!("Creating project directory: {}", &project_dir);
    if Path::new(&project_dir).is_dir() {
        error!("Project directory already exists! Please delete this.");
        return Err(ActionError::DirectoryExistsError {
            dir_name: project_dir,
        });
    }
    std::fs::create_dir(&project_dir)?;

    let tdir = match PathBuf::from(tdir_path)
        .join(choice)
        .into_os_string()
        .into_string()
    {
        Ok(tdir) => tdir,
        Err(_) => return Err(ActionError::OsStringError {}),
    };

    info!("Copying files...");
    match copy_files(template, &project_dir, &tdir) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Delete the directory that we just created
            info!(
                "Cleaning up the created project directory: {}",
                &project_dir
            );
            std::fs::remove_dir(&project_dir)?;
            return Err(e);
        }
    }
}
