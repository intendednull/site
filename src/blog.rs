use pandoc;
use std::path::Path;


pub fn update_blog() {
    let blogdir = Path::new("./src/static/blog");
    let outdir = Path::new("./src/templates/blog");

    for entry in blogdir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            convert(&entry.path(), &outdir);
        }
    }
}


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
