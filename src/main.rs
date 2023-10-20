use chrono::NaiveDate;
use std::fs;
use std::io::Write;
use tectonic;

macro_rules! with_files_included {
  ($($filename:expr),*; $code:block) => {
      {
        $(
            let content = include_bytes!($filename).to_vec();
            fs::File::create($filename)
                .expect(&format!("Could not create {}", $filename))
                .write_all(&content)
                .expect(&format!("Could not write to {}", $filename));
        )*

        $code

        $(
          fs::remove_file($filename).expect(&format!("Failed to delete {}", $filename));
        )*
      }
  };
}

fn get_author_name() -> Option<String> {
  let authors = env!("CARGO_PKG_AUTHORS");
  // Split the string by the colon (':') character, which separates multiple authors, and take the first
  let first_author = authors.split(':').next()?;
  // Extract the name by splitting at the '<' character and taking the part before it
  let name = first_author.split('<').next()?;
  Some(name.trim().to_string())
}

fn get_filename() -> String {
  let version = env!("CARGO_PKG_VERSION");
  let mut parts = version.split('.');
  let year = parts
    .next()
    .expect("Failed to detect major version number")
    .parse()
    .expect("Failed to parse year from major version number");
  let month = parts
    .next()
    .expect("Failed to detect minor version number")
    .parse()
    .expect("Failed to parse month from minor version number");
  let dt = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
  let date_suffix = dt.format("%B %Y");
  let author = get_author_name().expect("Failed to get author name");
  format!("CV {} {}.pdf", author, date_suffix)
}

fn main() {
  let filename = get_filename();

  with_files_included!(
    "me.jpg",
    "work---2004-apr--2005-mar---atlas.tex",
    "work---2005-mar--2005-dec---dsec.tex",
    "work---2006-apr--2007-oct---alarity.tex",
    "work---2007-oct--2016-july---ps.tex",
    "work---2016-aug--2019-apr---veon.tex",
    "work---2019-may--2020-jul---stackstate.tex",
    "work---2019-nov--2020-jul---stackstate.tex",
    "work---2020-aug--2021-aug---ing.tex",
    "work---2021-aug--now---klm.tex",
    "opensource.tex",
    "startup.tex",
    "skills.tex",
    "certs.tex",
    "edu.tex",
    "work.tex"; {
      let tex_content = include_str!("cv.tex");
      let pdf_data: Vec<u8> = tectonic::latex_to_pdf(tex_content).expect("Processing failed");
      let mut file = fs::File::create(&filename)
          .expect(&format!("Could not create {}", &filename));
      file.write_all(&pdf_data).expect(&format!("Could not write to {}", &filename));
      println!("{} bytes saved to \"{}\"", pdf_data.len(), filename);
  });
}
