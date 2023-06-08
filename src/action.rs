// Perform the copying, templating etc.

use handlebars;
use log::{error, info};
use serde::Serialize;
use std::fs::{remove_dir_all, File};
use std::path::{Path, PathBuf};

use crate::lconfig;

use crate::error::ActionError;
use crate::leetcode::LeetcodeQuestion;

pub struct Action<'a> {
    question: LeetcodeQuestion,
    template: &'a String,
    config: &'a lconfig::Config,
    project_dir: String,
}

impl<'a> Action<'a> {
    /// Creates the folder where the templated project files will be saved to
    ///
    /// It expects that there exists no folder of the same name, in order to ensure that we are not
    /// overwriting files that already exists (dangerous)
    fn create_project_dir_folder(&self) -> Result<(), ActionError> {
        info!("Creating project directory: {}", &self.project_dir);
        if Path::new(&self.project_dir).is_dir() {
            return Err(ActionError::DirectoryExistsError {
                dir_name: self.project_dir.clone(),
            });
        }
        std::fs::create_dir(&self.project_dir)?;

        Ok(())
    }

    /// Renders the individual templates.
    pub fn render_templates(&self) -> Result<(), ActionError> {
        // TODO: Handle this better than unwrap.
        let chosen_template = self.config.get_template(&self.template).unwrap();

        let code_snippet = match self.question.code_snippets.iter().find(|&x| {
            *x.lang == String::from(&chosen_template.language)
                || *x.langSlug == String::from(&chosen_template.language)
        }) {
            Some(cs) => cs,
            None => {
                return Err(ActionError::TemplateError {
                    msg: format!("Language {} not found", &chosen_template.language),
                })
            }
        };

        #[derive(Serialize)]
        struct TemplateVars<'a> {
            question_id: &'a String,
            code_snippet: &'a String,
            question_title: &'a String,
            example_test_cases: &'a String,
        }

        let template_vars = TemplateVars {
            question_id: &self.question.question_id,
            code_snippet: &code_snippet.code,
            question_title: &self.question.question_title,
            example_test_cases: &self.question.example_test_cases,
        };

        let mut template = handlebars::Handlebars::new();

        for template_file in chosen_template.files.iter() {
            let template_file_path = Path::new(&self.config.data.settings.template_dir)
                .join(&self.template)
                .join(&template_file);

            info!(
                "Rendering template for {} using file: {}",
                &template_file,
                template_file_path.to_str().unwrap()
            );

            // Create the file
            let output_path = Path::new(&self.project_dir).join(&template_file);
            let mut rendered_file = File::create(output_path).unwrap();

            // Maybe there's a slightly shorter way to write this.
            match template.register_template_file(&template_file, &template_file_path) {
                Ok(x) => x,
                Err(_) => {
                    return Err(ActionError::TemplateError {
                        msg: "Failed to register handlebar template".into(),
                    })
                }
            };

            // TODO: Disable HTML encoding
            let mut render_ctx = handlebars::RenderContext::new(Some(&self.project_dir));
            render_ctx.set_disable_escape(true);

            match template.render_to_write(&template_file, &template_vars, &mut rendered_file) {
                Ok(x) => x,
                Err(_) => {
                    return Err(ActionError::TemplateError {
                        msg: "Failed writing rendered template to disk".into(),
                    })
                }
            };
        }

        Ok(())
    }

    /// Runs the action.
    ///
    /// This involves the following tasks --
    /// 1. Create a folder for the template project files.
    /// 2. Render and save the actual templates within the created folder.
    pub fn run(&self) -> Result<(), ActionError> {
        // Create a new folder
        self.create_project_dir_folder()?;
        // Populate templates mustache with vars and save
        //self.create_mustache_template()?;
        self.render_templates()?;

        Ok(())
    }

    pub fn clean_up(&self) -> Result<(), std::io::Error> {
        remove_dir_all(&self.project_dir)
    }

    /// Returns a new Action object
    pub fn new(
        question: LeetcodeQuestion,
        template: &'a String,
        config: &'a lconfig::Config,
    ) -> Result<Action<'a>, ActionError> {
        let cwd = match std::env::current_dir()?.into_os_string().into_string() {
            Ok(cwd) => cwd,
            Err(_) => {
                error!("Failed to convert CWD path to a string.");
                return Err(ActionError::OsStringError {});
            }
        };

        let project_dir = match PathBuf::from(cwd)
            .join(&question.question_title_no_spaces)
            .to_str()
        {
            Some(s) => s.to_string(),
            None => {
                error!("Failed to convert Project Directory's path to a string");
                return Err(ActionError::GenericError {
                    msg: "Couldn't convert PathBuf to str".to_string(),
                });
            }
        };

        Ok(Action {
            question,
            template,
            config,
            project_dir,
        })
    }
}
