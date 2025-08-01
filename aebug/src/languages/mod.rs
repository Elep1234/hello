pub mod python;

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

pub trait Language {
    fn name(&self) -> &str;
    fn extension(&self) -> &str;
    fn analyze(&self, file_path: &Path) -> Result<AnalysisResult, String>;
    fn run(&self, file_path: &Path) -> Result<ExecutionResult, String>;
    fn fix(
        &self,
        analysis: &AnalysisResult,
        execution: &ExecutionResult,
        ai_model: &str,
    ) -> Result<String, String>;
}
