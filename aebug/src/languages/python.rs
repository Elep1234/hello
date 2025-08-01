use crate::ai;
use super::{AnalysisResult, ExecutionResult, Language};
use rustpython_parser::{self, Mode};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

pub struct PythonLanguage;

impl Language for PythonLanguage {
    fn name(&self) -> &str {
        "Python"
    }

    fn extension(&self) -> &str {
        ".py"
    }

    fn analyze(&self, file_path: &Path) -> Result<AnalysisResult, String> {
        let code = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let issues = match rustpython_parser::parse(&code, Mode::Module, file_path.to_str().unwrap_or("<unknown>")) {
            Ok(_) => vec![],
            Err(e) => vec![format!("Syntax error: {}", e)],
        };

        Ok(AnalysisResult {
            file_path: file_path.to_path_buf(),
            issues,
        })
    }

    fn run(&self, file_path: &Path) -> Result<ExecutionResult, String> {
        let mut cmd = Command::new("python3");
        cmd.arg(file_path);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn process: {}", e))?;

        let timeout = Duration::from_secs(5);

        match child.wait_timeout(timeout).map_err(|e| format!("Failed to wait for process: {}", e))? {
            Some(status) => {
                let output = child.wait_with_output().map_err(|e| format!("Failed to get output: {}", e))?;
                Ok(ExecutionResult {
                    exit_code: status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    timed_out: false,
                })
            }
            None => {
                child.kill().map_err(|e| format!("Failed to kill process: {}", e))?;
                child.wait().map_err(|e| format!("Failed to wait on killed process: {}", e))?;
                Ok(ExecutionResult {
                    exit_code: None,
                    stdout: "".to_string(),
                    stderr: "Process timed out".to_string(),
                    timed_out: true,
                })
            }
        }
    }

    fn fix(
        &self,
        analysis: &AnalysisResult,
        execution: &ExecutionResult,
        _ai_model: &str,
    ) -> Result<String, String> {
        ai::get_fix_suggestion(analysis, execution)
    }
}
