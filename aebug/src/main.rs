use clap::Parser;
use std::path::{Path, PathBuf};

mod ai;
mod languages;
use languages::{python::PythonLanguage, Language};

/// A fast, portable, and secure AI-powered debugger.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The file to debug.
    #[arg(required = true)]
    file_path: PathBuf,

    /// The AI model to use for suggestions.
    #[arg(long, default_value = "basic")]
    ai_model: String,
}

fn get_language_handler(file_path: &Path) -> Option<Box<dyn Language>> {
    match file_path.extension()?.to_str()? {
        "py" => Some(Box::new(PythonLanguage)),
        _ => None,
    }
}

fn main() {
    let cli = Cli::parse();
    let file_path = &cli.file_path;

    if let Some(language_handler) = get_language_handler(file_path) {
        println!("\nDebugging `{}` with `{}` language handler...", file_path.display(), language_handler.name());

        let analysis_result = language_handler.analyze(file_path);
        let execution_result = language_handler.run(file_path);

        match (&analysis_result, &execution_result) {
            (Ok(analysis), Ok(execution)) => {
                println!("\n--- Analysis Result ---");
                if analysis.issues.is_empty() {
                    println!("No issues found.");
                } else {
                    for issue in &analysis.issues {
                        println!("- {}", issue);
                    }
                }

                println!("\n--- Execution Result ---");
                println!("Exit Code: {:?}", execution.exit_code);
                if !execution.stdout.is_empty() {
                    println!("Stdout:\n{}", execution.stdout);
                }
                if !execution.stderr.is_empty() {
                    println!("Stderr:\n{}", execution.stderr);
                }
                if execution.timed_out {
                    println!("Execution timed out!");
                }

                println!("\n--- AI Suggestion ---");
                match language_handler.fix(analysis, execution, &cli.ai_model) {
                    Ok(suggestion) => println!("{}", suggestion),
                    Err(e) => eprintln!("Error getting suggestion: {}", e),
                }
            }
            (Err(e), _) => eprintln!("Analysis error: {}", e),
            (_, Err(e)) => eprintln!("Execution error: {}", e),
        }
    } else {
        eprintln!("Unsupported file type: {:?}", file_path.extension());
    }
}
