use pandoc;
use lazy_static::lazy_static;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use serde::{Serialize};

use super::util::File;


#[derive(Serialize, Clone)]
struct BlogPost {
    title: String,
    timestamp: u64,
    file: File
}


impl BlogPost {
    fn new(fp: PathBuf) -> Self {
        let file = File::new(fp);
        Self {
            title: file.title(),
            timestamp: file.timestamp().unwrap(),
            file: file
        }
    }
}


lazy_static! {
    static ref POSTS: HashMap<String, BlogPost> = {
        let blogdir = Path::new("blog");
        let mut posts = HashMap::new();

        for fp in blogdir.read_dir().unwrap() {
            if let Ok(fp) = fp {
                let post = BlogPost::new(fp.path());
                posts.insert(post.title.clone(), post);
            }
        }
        posts
    };
}


/// Convert all blog `org` files to `html`.
/// Places the html files in a directory that is visible by `tera`.
pub fn update_blog() {
    let outdir = Path::new("src/templates/blog");

    for entry in POSTS.values() {
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


fn make_get_blogpost() -> tera::GlobalFn {
    Box::new(move |args| -> tera::Result<tera::Value> {
        match args.get("title") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(v) =>  Ok(tera::to_value(POSTS.get(&v).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("Could not get blog post.".into()),
        }
    })
}


pub fn configure_tera(_tera: &mut tera::Tera) {
    _tera.register_function("get_blogpost", make_get_blogpost());
}
