use std::fs::File;
use std::io::{Read, Write};

// file guard
struct FileGuard {
    file: File,
}

impl FileGuard {
    fn new(file: File) -> Self {
        FileGuard { file }
    }

    fn write_data(&mut self, data: &str) -> std::io::Result<()> {
        self.file.write_all(data.as_bytes())
    }

    fn read_data(&mut self) -> std::io::Result<String> {
        let mut data = String::new();
        self.file.read_to_string(&mut data)?;
        Ok(data)
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("FileGuard: flashing file resources");
    }
}

fn main() {
    let file_path = "raii.txt";

    {
        let file = File::create(file_path).expect("Failed to create file");
        let mut file_guard = FileGuard::new(file);

        // Write data to the file using the FileGuard.
        file_guard
            .write_data("RAII pattern, also known as the Resource Acquisition Is Initialization (RAII), a new pattern for me, but it is very useful.")
            .expect("Failed to write data");
    }

    let file = File::open(file_path).expect("Failed to open file");
    let mut file_guard = FileGuard::new(file);

    let data = file_guard.read_data().expect("Failed to read data");
    println!("File contents: {}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_guard() {
        let file_path = "test.txt";

        {
            let file = File::create(file_path).expect("Failed to create file");
            let mut file_guard = FileGuard::new(file);

            // Write data to the file using the FileGuard.
            file_guard
                .write_data("Test data")
                .expect("Failed to write data");
        }

        let file = File::open(file_path).expect("Failed to open file");
        let mut file_guard = FileGuard::new(file);

        let data = file_guard.read_data().expect("Failed to read data");
        assert_eq!(data, "Test data");
    }
}
