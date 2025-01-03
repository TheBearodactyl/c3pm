use crate::model::{BuildConfig, CStandard, CppStandard, Generator, Language};
use crate::util::AVAILABLE_THREADS;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct C3pmArgs {
    /// Generate a new project :3
    #[command(subcommand)]
    pub new_cmd: NewSubcmd,
}

#[derive(Subcommand, Debug)]
pub enum NewSubcmd {
    /// Generates a CMake project in a new directory
    New {
        /// The name of the project to generate
        name: String,

        /// The generator to use
        #[arg(short, long, required = false, help = "The name of the generator to use (use this flag without an argument for a list of possible values)", hide_possible_values = true)]
        generator: Option<Generator>,

        /// The language of the project (can either be "c", "cpp", "cxx", or "c++")
        #[arg(short, long, value_parser=parse_language, default_value = "cpp")]
        language: Language,

        /// The name of the folder to generate the project in (defaults to the project name)
        #[arg(short, long)]
        folder: Option<String>,
    },
    /// Initializes a new CMake project in the current directory
    Init {
        /// The name of the project to initialize (defaults to the name of the current directory)
        name: Option<String>,

        /// The generator to use
        #[arg(short, long, required = false, help = "The name of the generator to use (use this flag without an argument for a list of possible values)", hide_possible_values = true)]
        generator: Option<Generator>,

        /// The language of the project (can either be "c", "cpp", "cxx", or "c++")
        #[arg(short, long, value_parser=parse_language, default_value = "cpp", required = false)]
        language: Language,
    },
    /// Builds the c3pm project
    Build {
        /// The number of threads to use for building
        #[arg(short = 'j', long = "jobs", default_value_t = *AVAILABLE_THREADS)]
        jobs: usize,

        /// The build config to use (e.g. Debug, RelWithDebInfo, Release)
        #[arg(short = 'c', long = "config", default_value = "RelWithDebInfo")]
        config: BuildConfig,
    },
}

fn parse_language(lang: &str) -> Result<Language, String> {
    let input: Vec<&str> = lang.split(':').collect();
    let lang = input[0];
    let standard = input.get(1);
    let supported_langs = ["c", "cpp", "cxx", "c++"];

    if supported_langs[0] == lang {
        let standard = standard.map(|t| *t).unwrap_or("23");
        let standard = CStandard::try_from(standard)?;
        return Ok(Language::C(standard));
    }

    if supported_langs[1..].contains(&lang) {
        let standard = standard.map(|t| *t).unwrap_or("23");
        let standard = CppStandard::try_from(standard)?;
        return Ok(Language::CPP(standard));
    }

    let formatted_possible_values = supported_langs.join(", ");
    Err(format!(
        "Possible values are {:?}",
        formatted_possible_values
    ))
}