use pandoc;
use std::path::Path;


/// Convert all blog `org` files to `html`.
/// Places the html files in a directory that is visible by `tera`.
pub fn update_blog() {
    let blogdir = Path::new("blog");
    let outdir = Path::new("src/templates/blog");

    for entry in blogdir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            convert(&entry.path(), &outdir);
        }
    }
}


/// Use pandoc to convert `org` file to `html`.
fn convert(fp: &Path, outdir: &Path) {
    let mut pdoc = pandoc::new();
    let fpout = outdir
        .join(fp.file_name().unwrap())
        .with_extension("html");

    pdoc.add_input(fp.to_str().unwrap());
    pdoc.set_output(
        pandoc::OutputKind::File(fpout.to_str().unwrap().to_string())
    );
    pdoc.execute().unwrap();
}
