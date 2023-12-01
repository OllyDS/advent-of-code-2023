/// takes a path for the location of the text file and converts to a String.
pub fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
