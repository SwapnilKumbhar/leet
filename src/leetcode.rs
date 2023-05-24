// Suppress warnings because of graphql derived types
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use cynic::http::SurfExt;
use cynic::{GraphQlError, QueryBuilder, QueryFragment, QueryVariables};
use log::{error, info};
use serde::Serialize;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////
/// Graphql stuff
////////////////////////////////////////////////////////////////////////////////
mod schema {
    cynic::use_schema!("src/gql/lc_schema.graphql");
}

#[derive(QueryFragment, Serialize)]
#[cynic(schema_path = "src/gql/lc_schema.graphql", schema_module = "schema")]
pub struct CodeSnippets {
    pub lang: String,
    pub langSlug: String,
    pub code: String,
}

#[derive(QueryFragment)]
#[cynic(schema_path = "src/gql/lc_schema.graphql", schema_module = "schema")]
struct QuestionData {
    codeSnippets: Vec<CodeSnippets>,
    questionFrontendId: String,
    exampleTestcaseList: Vec<String>,
    title: String,
}

#[derive(QueryVariables)]
struct QuestionVars {
    titleSlug: String,
}

#[derive(QueryFragment)]
#[cynic(
    schema_path = "src/gql/lc_schema.graphql",
    schema_module = "schema",
    graphql_type = "QueryRoot",
    variables = "QuestionVars"
)]
struct question {
    #[arguments(titleSlug: $titleSlug)]
    question: QuestionData,
}
////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
pub struct LeetcodeQuestion {
    pub question_id: String,
    pub code_snippets: Vec<CodeSnippets>,
    pub question_title: String,
    pub question_title_no_spaces: String,
    pub example_test_cases: String,
}

#[derive(Clone, Debug)]
pub struct Leetcode {
    endpoint: String,
}

fn convert_link_to_slug(link: &String) -> String {
    link.trim_end_matches('/').split("/").last().unwrap().into()
}

#[derive(Debug, Error)]
pub enum LeetcodeError {
    #[error("Surf failed while making a request")]
    SurfError,

    #[error("Graphql error")]
    ApiError { Errors: Vec<GraphQlError> },

    #[error("Unknown. Report this!")]
    Unknown,
}

impl Leetcode {
    pub async fn get_question_by_link(
        &self,
        link: &String,
    ) -> Result<LeetcodeQuestion, LeetcodeError> {
        let slug = convert_link_to_slug(link);
        info!("Derived slug from link: {}", slug);
        let query = question::build(QuestionVars { titleSlug: slug });

        let resp = match surf::post(&self.endpoint).run_graphql(query).await {
            Ok(resp) => resp,
            // TODO: Handle or bubble this error a little more meaningfully
            Err(e) => return Err(LeetcodeError::SurfError),
        };

        match resp.errors {
            Some(e) => {
                return Err(LeetcodeError::ApiError { Errors: e });
            }
            None => {}
        };

        let data = resp.data.unwrap();
        let question_title_no_spaces = data.question.title.replace(" ", "");
        let example_test_cases = data.question.exampleTestcaseList.join("\n");
        let question = LeetcodeQuestion {
            question_id: data.question.questionFrontendId,
            code_snippets: data.question.codeSnippets,
            question_title: data.question.title.clone(),
            question_title_no_spaces,
            example_test_cases,
        };
        Ok(question)
    }

    pub fn new() -> Self {
        return Leetcode {
            endpoint: String::from("https://leetcode.com/graphql"),
        };
    }
}
