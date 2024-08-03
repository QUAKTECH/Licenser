use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_templates() -> HashMap<&'static str, PathBuf> {
    let mut templates = HashMap::new();
    let base_path = PathBuf::from("Templates");

    templates.insert("ecl-2.0", base_path.join("ecl-2.0.tmpl"));
    templates.insert("eupl-1.2", base_path.join("eupl-1.2.tmpl"));
    templates.insert("lgpl-3.0", base_path.join("lgpl-3.0.tmpl"));
    templates.insert("wtfpl", base_path.join("wtfpl.tmpl"));
    templates.insert("mulanpsl-2.0", base_path.join("mulanpsl-2.0.tmpl"));
    templates.insert("mpl-2.0", base_path.join("mpl-2.0.tmpl"));
    templates.insert("ms-rl", base_path.join("ms-rl.tmpl"));
    templates.insert("lgpl-2.1", base_path.join("lgpl-2.1.tmpl"));
    templates.insert("cc-by-4.0", base_path.join("cc-by-4.0.tmpl"));
    templates.insert("epl-2.0", base_path.join("epl-2.0.tmpl"));
    templates.insert("bsd-0-clause", base_path.join("bsd-0-clause.tmpl"));
    templates.insert("mit-0", base_path.join("mit-0.tmpl"));
    templates.insert("free-art-1.3", base_path.join("free-art-1.3.tmpl"));
    templates.insert("osl-3.0", base_path.join("osl-3.0.tmpl"));
    templates.insert("bsd-3-clause", base_path.join("bsd-3-clause.tmpl"));
    templates.insert("afl-3.0", base_path.join("afl-3.0.tmpl"));
    templates.insert("agpl-3.0", base_path.join("agpl-3.0.tmpl"));
    templates.insert("cc-by-sa-4.0", base_path.join("cc-by-sa-4.0.tmpl"));
    templates.insert("zlib", base_path.join("zlib.tmpl"));
    templates.insert("apache-2.0", base_path.join("apache-2.0.tmpl"));
    templates.insert("al-2.0", base_path.join("al-2.0.tmpl"));
    templates.insert("mit", base_path.join("mit.tmpl"));
    templates.insert("isc", base_path.join("isc.tmpl"));
    templates.insert("cc0-1.0", base_path.join("cc0-1.0.tmpl"));
    templates.insert("lppl", base_path.join("lppl.tmpl"));
    templates.insert("unlicense", base_path.join("unlicense.tmpl"));
    templates.insert("bsd-4-clause", base_path.join("bsd-4-clause.tmpl"));
    templates.insert("ms-pl", base_path.join("ms-pl.tmpl"));
    templates.insert("bsd-2-clause", base_path.join("bsd-2-clause.tmpl"));
    templates.insert("ofl-1.1", base_path.join("ofl-1.1.tmpl"));
    templates.insert("odbl-1.0", base_path.join("odbl-1.0.tmpl"));
    templates.insert("gpl-2.0", base_path.join("gpl-2.0.tmpl"));
    templates.insert("gpl-3.0", base_path.join("gpl-3.0.tmpl"));
    templates.insert("quaktech-1.0", base_path.join("quaktech-1.0.tmpl"));
    templates.insert("quaktech-2.0", base_path.join("quaktech-2.0.tmpl"));

    templates
}
