use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Scripts(BTreeMap<String, String>);

#[derive(Deserialize, Debug)]
struct PackageJson {
    scripts: Scripts,
}

impl fmt::Display for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = &self
            .0
            .iter()
            .map(|(script, command)| {
                let s = format!("\x1b[32m{}\x1b[m:", script);
                format!("{0: <24}{1}\n", s, command)
            })
            .collect::<String>();
        write!(f, "{}", output)
    }
}

fn read_package_json() -> io::Result<Scripts> {
    let path = Path::new("./package.json");
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let package: PackageJson = serde_json::from_str(&s)?;
    Ok(package.scripts)
}

fn main() {
    let package_script = read_package_json();
    match package_script {
        Err(e) => println!("Error: cause {:?}", e.kind()),
        Ok(s) => println!("{}", s)
    }
}
