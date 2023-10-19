use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Retrieve the embedded content of the sample.tex file
    let tex_content = include_str!("sample.tex");
    let mut file = File::create("sample.tex").expect("Could not create file");
    file.write_all(tex_content.as_bytes()).expect("Could not write to file");

    let output = Command::new("pdflatex")
        .arg("sample.tex")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to generate PDF");
    } else {
        println!("Successfully generated PDF\n{}",
            String::from_utf8_lossy(&output.stdout));
    }
}
