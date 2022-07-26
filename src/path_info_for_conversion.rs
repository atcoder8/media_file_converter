use std::path::{Path, PathBuf};

/// Stores path information about the conversion.
pub struct PathInfoForConversion {
    original_file_path_buf: PathBuf,
    converted_file_path_buf: PathBuf,
}

impl PathInfoForConversion {
    /// Creates a structure.
    pub fn new(
        original_folder_pathname: &str,
        converted_folder_pathname: &str,
        original_file_basename: &str,
        converted_extension: &str,
    ) -> Self {
        let mut original_file_path_buf = PathBuf::from(original_folder_pathname);
        original_file_path_buf.push(original_file_basename);

        let mut converted_file_path_buf = PathBuf::from(converted_folder_pathname);
        converted_file_path_buf.push(original_file_basename);
        converted_file_path_buf.set_extension(converted_extension);

        Self {
            original_file_path_buf,
            converted_file_path_buf,
        }
    }

    /// Get the pathname of the original file.
    pub fn get_original_file_pathname(&self) -> &str {
        self.original_file_path_buf
            .to_str()
            .expect("\nPath is not valid Unicode.")
    }

    /// Get the basename of the original file.
    pub fn get_original_file_basename(&self) -> &str {
        self.original_file_path_buf
            .file_name()
            .expect("\nCould not get file name.")
            .to_str()
            .expect("\nPath is not valid Unicode.")
    }

    /// Get the pathname of the converted file.
    pub fn get_converted_file_path(&self) -> &Path {
        &self.converted_file_path_buf.as_path()
    }

    /// Get the basename of the converted file.
    pub fn get_converted_file_pathname(&self) -> &str {
        self.converted_file_path_buf
            .to_str()
            .expect("\nPath is not valid Unicode.")
    }
}
