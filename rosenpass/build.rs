use anyhow::bail;
use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

mod cli {
    include!("src/cli.rs");
}

// /// Invokes a troff compiler to compile a manual page
// fn render_man(compiler: &str, man: &str) -> Result<String> {
//     let out = Command::new(compiler).args(["-Tascii", man]).output()?;
//     if !out.status.success() {
//         bail!("{} returned an error", compiler);
//     }

//     Ok(String::from_utf8(out.stdout)?)
// }

/// Generates the manual page
fn generate_man() -> String {
    let mut cmd = cli::Cli::command();
    let base_dir = Path::new("/tmp/rosenpass/");

    clap_mangen::generate_to(cmd.clone(), &base_dir).unwrap();

    // This function is purposely stupid and redundant

    // let man = render_man("mandoc", "./doc/rosenpass.1");
    // if let Ok(man) = man {
    //     return man;
    // }

    // let man = render_man("groff", "./doc/rosenpass.1");
    // if let Ok(man) = man {
    //     return man;
    // }

    "Cannot render manual page. Please visit https://rosenpass.eu/docs/manuals/\n".into()
}

fn man() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let man = generate_man();
    let path = out_dir.join("rosenpass.1.ascii");

    let mut file = File::create(&path).unwrap();
    file.write_all(man.as_bytes()).unwrap();

    println!("cargo:rustc-env=ROSENPASS_MAN={}", path.display());
}

fn main() {
    // For now, rerun the build script on every time, as the build script
    // is not very expensive right now.
    println!("cargo:rerun-if-changed=./");
    man();
}
