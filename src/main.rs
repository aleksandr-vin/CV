use chrono::NaiveDate;
use std::fs;
use std::io::Write;
use tectonic;
use image::{open, ImageFormat};

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

fn get_cv_date() -> Option<NaiveDate> {
  let year = env!("CARGO_PKG_VERSION_MAJOR").parse()
    .expect("Failed to parse year from major version number");
  let month = env!("CARGO_PKG_VERSION_MINOR").parse()
    .expect("Failed to parse month from minor version number");
  NaiveDate::from_ymd_opt(year, month, 1)
}

fn get_cv_repo() -> String {
  let repo = env!("CARGO_PKG_REPOSITORY");
  assert_ne!(repo, "", "Repository must not be empty");
  repo.to_string()
}

fn get_cv_version() -> String {
  env!("CARGO_PKG_VERSION").to_string()
}

fn resize_image(factor: f32) {
  let img_filename = "me.jpg";
  let img = open(&img_filename).expect(&format!("Could not open {}", &img_filename));
  // Resize to 30% of the original dimensions.
  let new_dimensions = (img.width() as f32 * factor, img.height() as f32 * factor);
  let resized = img.resize_exact(new_dimensions.0 as u32, new_dimensions.1 as u32, image::imageops::Nearest);
  resized.save_with_format(&img_filename, ImageFormat::Jpeg).expect(&format!("Could not save {}", &img_filename))
}

fn main() {
  let dt = get_cv_date().expect("Faied to get CV date from crate version");
  let author = get_author_name().expect("Failed to get author name");
  let cv_repo = get_cv_repo();
  let cv_version = get_cv_version();

  with_files_included!(
    "work---2004-apr--2005-mar---atlas.tex",
    "work---2005-mar--2005-dec---dsec.tex",
    "work---2006-apr--2007-oct---alarity.tex",
    "work---2007-oct--2016-jul---ps.tex",
    "work---2016-aug--2019-apr---veon.tex",
    "work---2019-may--2020-jul---stackstate.tex",
    "work---2020-aug--2021-aug---ing.tex",
    "work---2021-aug--2023-dec---klm.tex",
    "personal-projects-content.tex",
    "personal-projects.tex",
    "opensource.tex",
    "startup-content.tex",
    "startup.tex",
    "startup-and-personal-projects.tex",
    "skills.tex",
    "certs.tex",
    "edu.tex",
    "work.tex",
    "cv-for-e.tex",
    "cv-for-s.tex",
    "me.jpg"; {
      resize_image(0.40);

      for suffix in [
        'e',
        // 's',
        ] {
        let filename = format!("CV {} {} {}.pdf", author, dt.format("%B %Y"), suffix);

        let pdf_data: Vec<u8> = tectonic::latex_to_pdf(format!(r#"
          \newcommand{{\cvdate}}{{{}}}
          \newcommand{{\cvrepo}}{{{}}}
          \newcommand{{\cvversion}}{{{}}}
          \input{{cv-for-{}}}
        "#, dt.format("%B, %Y"), cv_repo, cv_version, suffix)).expect("Processing failed");
        let mut file = fs::File::create(&filename)
            .expect(&format!("Could not create {}", &filename));
        file.write_all(&pdf_data).expect(&format!("Could not write to {}", &filename));
        println!("{} bytes saved to \"{}\"", pdf_data.len(), filename);
      }
  });
}
