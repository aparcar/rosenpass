use clap::CommandFactory;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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

// fn generate_man() -> String {
//     let cmd = cli::Cli::command();
//     let base_dir = Path::new("/tmp/rosenpass/");

//     clap_mangen::generate_to(cmd.clone(), base_dir).unwrap();

//     // This function is purposely stupid and redundant

//     // let man = render_man("mandoc", "./doc/rosenpass.1");
//     // if let Ok(man) = man {
//     //     return man;
//     // }

//     // let man = render_man("groff", "./doc/rosenpass.1");
//     // if let Ok(man) = man {
//     //     return man;
//     // }

//     "Cannot render manual page. Please visit https://rosenpass.eu/docs/manuals/\n".into()
// }

// fn man() {
//     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//     let man = generate_man();
//     let path = out_dir.join("rosenpass.1.ascii");

//     let mut file = File::create(&path).unwrap();
//     file.write_all(man.as_bytes()).unwrap();

//     println!("cargo:rustc-env=ROSENPASS_MAN={}", path.display());
// }

const ENV_KEY: &str = "ROSENPASS_GEN_DIR";

fn main() -> std::io::Result<()> {
    // if let Some(dir) = env::var_os(ENV_KEY).or(env::var_os("OUT_DIR")) {
    // let dir = env::var(ENV_KEY).unwrap();
    let dir: &str = "/Users/user/src/rosenppass/rosenpass/foobar/";
    let mut cmd = cli::Cli::command();
    let base_dir = PathBuf::from(dir);

    let man_dir = Path::join(&base_dir, "man");
    create_dir_all(&man_dir)?;
    clap_mangen::generate_to(cmd.clone(), &man_dir)?;
    // }
    // println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    // println!("cargo:rerun-if-env-changed={ENV_KEY}");
    println!(
        "cargo:rustc-env=ROSENPASS_MAN={}/rosenpass.1",
        man_dir.display()
    );
    println!("cargo:rerun-if-changed=./");

    Ok(())
}
