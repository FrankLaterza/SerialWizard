use rfd::FileDialog;
use std::fs::File;
use std::io::Write;

fn main() {
    let dir = FileDialog::new().set_directory("/").pick_folder();

    // print the path
    match dir {
        Some(path) => {
            // create file
            
            // add the file name to the path
            let file_path = path.join("hello.txt"); // Use the selected path to create a file path
            // create the file
            let mut file = File::create(&file_path).expect("Error: could not create file");

            file.write_all(b"hello world")
                .expect("Error could not write file");
        }
        None => {}
    }
}
