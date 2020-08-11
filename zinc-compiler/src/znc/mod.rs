//!
//! The Zinc compiler binary.
//!

mod arguments;
mod error;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;

use zinc_compiler::Entry;
use zinc_compiler::Source;
use zinc_compiler::State;

use self::arguments::Arguments;
use self::error::Error;
use self::error::OutputError;

/// The unit tests subdirectory path. Is relative to the bytecode build directory.
static TEST_BINARIES_DIRECTORY: &str = "test/";
/// The witness template JSON file suffix. Is appended before the `.json` extension.
static WITNESS_TEMPLATE_SUFFIX: &str = "_witness";
/// The public data template JSON file suffix. Is appended before the `.json` extension.
static PUBLIC_DATA_TEMPLATE_SUFFIX: &str = "_public_data";

///
/// The application entry point.
///
fn main() {
    process::exit(match main_inner() {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> Result<(), Error> {
    let args = Arguments::new();

    zinc_utils::logger::initialize(zinc_const::app_name::ZINC_COMPILER, args.verbosity);

    let bytecode = Source::try_from_entry(&args.source_path)?.compile("stableswap".to_owned())?;
    let compiled_entries =
        State::unwrap_rc(bytecode).into_methods(args.optimize_dead_function_elimination);

    for (entry_name, entry) in compiled_entries.into_iter() {
        let mut bytecode_path = args.build_path.clone();

        match entry {
            Entry::Default {
                bytecode,
                witness_template,
                public_data_template,
            } => {
                if args.test_only {
                    continue;
                }

                fs::create_dir_all(&bytecode_path).map_err(|error| {
                    Error::DirectoryCreating(bytecode_path.as_os_str().to_owned(), error)
                })?;
                bytecode_path.push(format!("{}.znb", entry_name));

                let mut witness_template_path = args.data_path.clone();
                fs::create_dir_all(&witness_template_path).map_err(|error| {
                    Error::DirectoryCreating(witness_template_path.as_os_str().to_owned(), error)
                })?;
                witness_template_path
                    .push(format!("{}{}.json", entry_name, WITNESS_TEMPLATE_SUFFIX));

                let mut public_data_template_path = args.data_path.clone();
                fs::create_dir_all(&public_data_template_path).map_err(|error| {
                    Error::DirectoryCreating(
                        public_data_template_path.as_os_str().to_owned(),
                        error,
                    )
                })?;
                public_data_template_path.push(format!(
                    "{}{}.json",
                    entry_name, PUBLIC_DATA_TEMPLATE_SUFFIX
                ));

                if !witness_template_path.exists() {
                    File::create(&witness_template_path)
                        .map_err(OutputError::Creating)
                        .map_err(|error| {
                            Error::WitnessTemplateOutput(
                                witness_template_path.as_os_str().to_owned(),
                                error,
                            )
                        })?
                        .write_all(witness_template.as_slice())
                        .map_err(OutputError::Writing)
                        .map_err(|error| {
                            Error::WitnessTemplateOutput(
                                witness_template_path.as_os_str().to_owned(),
                                error,
                            )
                        })?;
                    log::info!("Witness template written to {:?}", witness_template_path);
                } else {
                    log::info!(
                        "Witness template {:?} already exists. Skipping",
                        witness_template_path
                    );
                }

                File::create(&public_data_template_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::PublicDataTemplateOutput(
                            public_data_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?
                    .write_all(public_data_template.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::PublicDataTemplateOutput(
                            public_data_template_path.as_os_str().to_owned(),
                            error,
                        )
                    })?;
                log::info!(
                    "Public data template written to {:?}",
                    public_data_template_path
                );

                File::create(&bytecode_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::BytecodeOutput(bytecode_path.as_os_str().to_owned(), error)
                    })?
                    .write_all(bytecode.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::BytecodeOutput(bytecode_path.as_os_str().to_owned(), error)
                    })?;
                log::info!("Compiled to {:?}", bytecode_path);
            }
            Entry::Test { bytecode } => {
                bytecode_path.push(TEST_BINARIES_DIRECTORY);
                fs::create_dir_all(&bytecode_path).map_err(|error| {
                    Error::DirectoryCreating(bytecode_path.as_os_str().to_owned(), error)
                })?;
                bytecode_path.push(format!("{}.znb", entry_name));

                File::create(&bytecode_path)
                    .map_err(OutputError::Creating)
                    .map_err(|error| {
                        Error::BytecodeOutput(bytecode_path.as_os_str().to_owned(), error)
                    })?
                    .write_all(bytecode.as_slice())
                    .map_err(OutputError::Writing)
                    .map_err(|error| {
                        Error::BytecodeOutput(bytecode_path.as_os_str().to_owned(), error)
                    })?;
                log::info!("Compiled to {:?}", bytecode_path);
            }
        }
    }

    Ok(())
}
