use std::{
    io::{self, Write},
    process,
};

use clap::clap_derive::ArgEnum;

use crate::path_info_for_conversion::PathInfoForConversion;

/// Overall settings for overwriting
#[derive(Debug, Clone, PartialEq, Eq, ArgEnum)]
pub enum OverallOverwriteSelect {
    /// Always overwrite.
    Yes,

    /// Always skip without overwriting.
    No,

    /// Determined for each file.
    Undecided,
}

/// Overwrite Settings
pub enum OverwriteSelect {
    /// Overwrite
    Yes,

    /// Skip without overwriting.
    No,

    /// Always overwrite.
    AllYes,

    /// Always skip without overwriting.
    AllNo,

    /// Exit this program.
    Exit,
}

/// Determines whether or not to perform the conversion.
pub fn decide_whether_to_execute(
    path_info_for_conversion: &PathInfoForConversion,
    overall_overwrite_select: &mut OverallOverwriteSelect,
) -> bool {
    // Returns true if the file does not exist in the destination path.
    if !path_info_for_conversion.get_converted_file_path().exists() {
        return true;
    }

    match overall_overwrite_select {
        // Returns true if set to always overwrite.
        OverallOverwriteSelect::Yes => true,

        // Returns false if set to always not overwrite.
        OverallOverwriteSelect::No => false,

        OverallOverwriteSelect::Undecided => {
            // Returns false if set to always not overwrite.

            // Ask whether to override.
            let overwrite_select = question_whether_to_overwrite();

            let execute_conversion_flag = match overwrite_select {
                OverwriteSelect::Yes => true,

                OverwriteSelect::No => false,

                OverwriteSelect::AllYes => {
                    *overall_overwrite_select = OverallOverwriteSelect::Yes;
                    true
                }

                OverwriteSelect::AllNo => {
                    *overall_overwrite_select = OverallOverwriteSelect::No;
                    false
                }

                OverwriteSelect::Exit => {
                    process::exit(0);
                }
            };

            // Output the basename of the original file.
            print!(
                "\n{}: ",
                path_info_for_conversion.get_original_file_basename()
            );
            io::stdout().flush().unwrap();

            execute_conversion_flag
        }
    }
}

/// Ask whether or not to perform the conversion.
pub fn question_whether_to_overwrite() -> OverwriteSelect {
    println!(
        "
The file already exists at the file output destination.
Please select one of the following.

\"yes\": Overwrite the file.
\"no\": Skip the file without overwriting it.
\"all-yes\": Overwrite all remaining files.
\"all-no\": Skip without overwriting for all remaining files.
\"exit\": Exit this program."
    );

    loop {
        print!("\nPlease input: ");
        io::stdout().flush().unwrap();

        let mut selection = String::new();
        std::io::stdin().read_line(&mut selection).unwrap();

        let selection = match selection.trim() {
            "yes" => Some(OverwriteSelect::Yes),
            "no" => Some(OverwriteSelect::No),
            "all-yes" => Some(OverwriteSelect::AllYes),
            "all-no" => Some(OverwriteSelect::AllNo),
            "exit" => Some(OverwriteSelect::Exit),
            _ => None,
        };

        if let Some(selection) = selection {
            return selection;
        }

        println!("Please choose one of the following options: {{\"yes\", \"no\", \"all-yes\", \"all-no\", \"exit\"}}.");
    }
}
