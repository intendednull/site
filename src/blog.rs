use pandoc;
use lazy_static::lazy_static;
use std::path::{PathBuf, Path};

use super::util::File;


struct BlogPost {
    file: File
}

impl BlogPost {
    fn new(fp: PathBuf) -> Self {
        Self { file: File::new(fp) }
    }
}


lazy_static! {
    static ref POSTS: Vec<BlogPost> = {
        let blogdir = Path::new("blog");

        blogdir.read_dir().unwrap()
            .map(|fp| BlogPost::new(fp.unwrap().path()))
            .collect()
    };
}


/// Convert all blog `org` files to `html`.
/// Places the html files in a directory that is visible by `tera`.
pub fn update_blog() {
    let outdir = Path::new("src/templates/blog");

    for entry in POSTS.iter() {
        convert(&entry, &outdir);
    }
}


/// Use pandoc to convert `org` file to `html`.
fn convert(post: &BlogPost, outdir: &Path) {
    let mut pdoc = pandoc::new();
    let fpout = outdir.join(&post.file.name()).with_extension("html");

    pdoc.add_input(post.file.path.to_str().unwrap());
    pdoc.set_output(
        pandoc::OutputKind::File(fpout.to_str().unwrap().to_string())
    );
    pdoc.execute().unwrap();
}
