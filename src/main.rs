use clap::Parser;
use overwrite_select::OverallOverwriteSelect;
use path_info_for_conversion::PathInfoForConversion;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, BufReader, Write},
    process::Command,
};

mod create_command;
mod overwrite_select;
mod path_info_for_conversion;

/// Default filename containing data about the file to be converted.
const CONVERT_DATA_PATHNAME: &'static str = "convert_data.json";

/// Default converted extension
const DEFAULT_CONVERTED_EXTENSION: &'static str = "flac";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A file name containing data about the file to be converted.
    #[clap(default_value_t = String::from(CONVERT_DATA_PATHNAME))]
    convert_data_file_pathname: String,

    /// Specifies whether to overwrite the file if it already exists.
    /// If "yes" is specified, the file is always overwritten.
    /// If "no" is specified, the file is always not overwritten.
    /// If "undecided" is specified, asks whether to overwrite each time.
    #[clap(short = 'o', long = "overwrite", arg_enum, default_value_t = OverallOverwriteSelect::Undecided)]
    overwrite: OverallOverwriteSelect,

    /// Specify the copy parameter.
    #[clap(short = 'c', long = "copy", action)]
    copy: bool,

    /// File extension of the converted file.
    #[clap(short = 'e', long = "extension", default_value_t = String::from(DEFAULT_CONVERTED_EXTENSION))]
    extension: String,
}

#[derive(Deserialize)]
struct AlbumData {
    /// Pathname of the original music file
    original_folder_pathname: String,

    /// Pathname of the converted music file
    converted_folder_pathname: String,

    /// Pathname of the album art
    album_art_file_pathname: Option<String>,

    /// Common Album Metadata
    common_metadata: Option<BTreeMap<String, String>>,

    /// Metadata for each song
    unique_metadata: BTreeMap<String, BTreeMap<String, String>>,
}

fn main() {
    // Information obtained from command line arguments
    let args = Args::parse();

    // Output configurations.
    println!(
        "\
[Configurations]
convert_data_file_pathname = {}
overwrite = {:?}
copy = {}
extension = {}",
        args.convert_data_file_pathname, args.overwrite, args.copy, args.extension,
    );
    io::stdout().flush().unwrap();

    // File containing data related to the conversion
    let convert_data_file =
        File::open(&args.convert_data_file_pathname).expect("Failed to open file.");
    // Buffer of files containing data relevant to the conversion
    let convert_data_buff_reader = BufReader::new(convert_data_file);
    // Data related to conversion
    let convert_data: BTreeMap<String, AlbumData> =
        serde_json::from_reader(convert_data_buff_reader).expect("\nFailed to deserialize.");

    // Overall override setting
    let mut overall_overwrite_select = args.overwrite;

    // Number of times the conversion was executed
    let mut converted_cnt = 0_u32;
    // Number of times a conversion was skipped
    let mut skipped_cnt = 0_u32;

    for (album_name, album_data) in convert_data.iter() {
        // Output album name.
        println!("\n[{}]", album_name);
        io::stdout().flush().unwrap();

        // Create a directory where converted files will be stored.
        fs::create_dir_all(&album_data.converted_folder_pathname)
            .expect("\nFailed to create directory.");

        // Common album metadata
        let common_metadata = match &album_data.common_metadata {
            Some(common_metadata) => common_metadata.clone(),
            None => BTreeMap::new(),
        };

        for (original_file_basename, unique_metadata) in &album_data.unique_metadata {
            // Output the basename of the original file.
            print!("{}: ", original_file_basename);
            io::stdout().flush().unwrap();

            // Path information about the conversion
            let path_info_for_conversion = PathInfoForConversion::new(
                &album_data.original_folder_pathname,
                &album_data.converted_folder_pathname,
                original_file_basename,
                &args.extension,
            );

            // Map with unique metadata added to common album metadata
            let mut metadata = common_metadata.clone();
            unique_metadata.iter().for_each(|(tag, data)| {
                metadata.insert(tag.clone(), data.clone());
            });

            // Flag indicating whether or not to execute the conversion
            let execute_conversion_flag = overwrite_select::decide_whether_to_execute(
                &path_info_for_conversion,
                &mut overall_overwrite_select,
            );

            if execute_conversion_flag {
                // Create a command.
                let cmd = match &album_data.album_art_file_pathname {
                    // If album art is specified
                    Some(album_art_filename) => create_command::add_album_art_and_metadata(
                        path_info_for_conversion.get_original_file_pathname(),
                        path_info_for_conversion.get_converted_file_pathname(),
                        album_art_filename,
                        &metadata,
                        true,
                        args.copy,
                    ),

                    // If album art is not specified
                    None => create_command::add_metadata(
                        path_info_for_conversion.get_original_file_pathname(),
                        path_info_for_conversion.get_converted_file_pathname(),
                        &metadata,
                        true,
                        args.copy,
                    ),
                };

                // Execute command.
                let output = Command::new(&cmd[0])
                    .args(&cmd[1..])
                    .output()
                    .expect("\nFailed to execute process.");

                // If the command fails to execute, the error message is displayed and panic.
                if !output.status.success() {
                    panic!(
                        "
Error: Command execution failed.

--- Error Message ---
{}
",
                        std::str::from_utf8(&output.stderr).expect("The slice is not UTF-8.")
                    );
                }

                // Outputs that the execution of the command is complete.
                println!("Converted");
                io::stdout().flush().unwrap();

                // Add to the number of times the conversion was executed.
                converted_cnt += 1;
            } else {
                // Outputs that execution of the command was skipped.
                println!("Skipped");
                io::stdout().flush().unwrap();

                // Add to the number of times a conversion was skipped.
                skipped_cnt += 1;
            }
        }
    }

    // Displays the number of files converted and the number of files skipped.
    println!("\nConverted: {}, Skipped: {}", converted_cnt, skipped_cnt);

    println!("\nPress the Enter key to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
