use crate::languages::{AnalysisResult, ExecutionResult};

pub fn get_fix_suggestion(
    analysis: &AnalysisResult,
    execution: &ExecutionResult,
) -> Result<String, String> {
    if !analysis.issues.is_empty() {
        let mut suggestions = String::from("Found potential issues during static analysis:\n");
        for issue in &analysis.issues {
            suggestions.push_str(&format!("- {}\n", issue));
            if issue.contains("Syntax error") {
                suggestions.push_str("  Suggestion: Check the syntax of your code, paying close attention to the line number mentioned in the error.\n");
            }
        }
        return Ok(suggestions);
    }

    if execution.timed_out {
        return Ok("The process timed out. This could be due to an infinite loop or a very long-running operation. Consider adding print statements to trace the execution flow.".to_string());
    }

    if let Some(exit_code) = execution.exit_code {
        if exit_code != 0 {
            let mut suggestion = format!(
                "The process exited with a non-zero exit code ({}). This usually indicates a runtime error.\n",
                exit_code
            );
            if !execution.stderr.is_empty() {
                suggestion.push_str(&format!(
                    "Stderr:\n{}\nSuggestion: Examine the traceback in the standard error output to locate the source of the error.",
                    execution.stderr
                ));
            }
            return Ok(suggestion);
        }
    }

    if execution.exit_code == Some(0) && analysis.issues.is_empty() {
        return Ok("No issues detected. The code ran successfully.".to_string());
    }

    Ok("No specific suggestion available at this time.".to_string())
}
