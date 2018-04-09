#[macro_use]
extern crate serde_derive;

extern crate git2;
extern crate serde;
extern crate serde_json;
extern crate clap;

use std::path::Path;
use std::fs::File;
use std::fs::create_dir_all;
use std::error::Error;
use git2::build::{ RepoBuilder };
use clap::{Arg, App};

/*
 * A dependency only needs a name, which is in the format "username/git-repo". The latest master
 * commit is fetched
 * If a commit is provided, the dependency will be cloned from that commit
 * If a branch is provided, the dependency will be cloned from the latest commmit in that branch.
 * If both commit and branch are provided, then the commit from that branch will be cloned.
 */
#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    name: String,
    commit: Option<String>,
    branch: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    description: String,

    dependencies: Option<Vec<Dependency>>,

    author: Option<String>,
    license: Option<String>
}

fn get_repo(repo: &str, clone_path: &str) {
    println!("Getting dependency {}...", repo);

    let mut builder = RepoBuilder::new();

    let _repo = builder.clone(&format!("https://github.com/{}.git", repo), Path::new(clone_path)).expect("Error cloning repo");
}

fn read_package_json<P: AsRef<Path>>(path: P) -> Result<Package, Box<Error>> {
    let file = File::open(path)?;
    let pkg = serde_json::from_reader(file)?;

    Ok(pkg)
}

fn get_dependencies(current_path: &str, package: &Dependency) {
    // Check if veri_modules directory exists, otherwise, make it
    let name = package.name.split("/").collect::<Vec<&str>>()[1];
    let path_name = format!("{}/veri_modules/{}", current_path, name);
    create_dir_all(&path_name);
    // Clone our first dependency in the veri_modules directory, with the name of the module
    get_repo(&package.name, &path_name);
    // get_dependencies of the package we cloned
    let p = read_package_json(format!("{}/package.json", &path_name)).unwrap();
    let deps = p.dependencies.unwrap_or(Vec::new());
    // repeat
    for d in &deps {
        get_dependencies(&path_name, d);
    }
}

/*
 * Create a project in the current directory
 */
fn init_project() {

}

fn main() {
    let matches = App::new("Veritas")
        .version("1.0")
        .author("Tanishq Dubey <tanishq.dubey@gmail.com>")
        .about("Veritas is a package manager for verilog projects")
        .arg(Arg::with_name("COMMAND")
             .help("The operation Veritas should perform")
             .possible_values(&["init", "install", "update"])
             .required(true))
        .get_matches();

    match matches.value_of("COMMAND").unwrap() {
        "init" => {
            println!("Not Implemented yet...")
        },
        "install" => {
            let p = read_package_json(&"./package.json").unwrap();
            let deps = p.dependencies.unwrap_or(Vec::new());
            // repeat
            for d in &deps {
                get_dependencies(".", d);
            }
        },
        "update" => {
            println!("Not Implemented yet...")
        },
        _ => unreachable!()
    }

}
