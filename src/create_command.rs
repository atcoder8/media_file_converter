use std::collections::BTreeMap;

/// Create a command to add metadata.
pub fn add_metadata(
    original_file_pathname: &str,
    converted_file_pathname: &str,
    metadata: &BTreeMap<String, String>,
    overwrite_flag: bool,
    specify_copy_parameter_flag: bool,
) -> Vec<String> {
    // Specify command name and original file pathname
    let mut cmd: Vec<String> = vec!["ffmpeg", "-i", original_file_pathname]
        .into_iter()
        .map(|x| String::from(x))
        .collect();

    // Specify overwrite flag
    if overwrite_flag {
        cmd.push(String::from("-y"));
    }

    // Specify copy parameter
    if specify_copy_parameter_flag {
        cmd.push(String::from("-codec"));
        cmd.push(String::from("copy"));
    }

    // Specify metadata
    for (tag, data) in metadata {
        cmd.push(String::from("-metadata:g"));
        cmd.push(format!("{}={}", tag, data));
    }

    // Specify converted file pathname
    cmd.push(String::from(converted_file_pathname));

    cmd
}

/// Create a command to add album art and metadata.
pub fn add_album_art_and_metadata(
    original_file_pathname: &str,
    converted_file_pathname: &str,
    album_art_file_pathname: &str,
    metadata: &BTreeMap<String, String>,
    overwrite_flag: bool,
    specify_copy_parameter_flag: bool,
) -> Vec<String> {
    // Specify command name, original file pathname, and album pathname
    let mut cmd: Vec<String> = vec![
        "ffmpeg",
        "-i",
        original_file_pathname,
        "-i",
        album_art_file_pathname,
        "-map",
        "0:a",
        "-map",
        "1:v",
        "-disposition:1",
        "attached_pic",
    ]
    .into_iter()
    .map(|x| String::from(x))
    .collect();

    // Specify overwrite flag
    if overwrite_flag {
        cmd.push(String::from("-y"));
    }

    // Specify copy parameter
    if specify_copy_parameter_flag {
        cmd.push(String::from("-codec"));
        cmd.push(String::from("copy"));
    }

    // Specify metadata
    for (tag, data) in metadata {
        cmd.push(String::from("-metadata:g"));
        cmd.push(format!("{}={}", tag, data));
    }

    // Specify converted file pathname
    cmd.push(String::from(converted_file_pathname));

    cmd
}
