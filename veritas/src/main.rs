#[macro_use]
extern crate serde_derive;

extern crate git2;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate colored;

use std::path::Path;
use std::fs::{File, create_dir_all};
use std::error::Error;
use std::io::{Write, BufWriter, stdin, stdout};
use git2::build::{ RepoBuilder };
use clap::{Arg, App};
use colored::*;

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
    println!("{} {}","Getting dependency:".green().bold(), repo);

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
    create_dir_all(&path_name).unwrap();
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
    // Ask the user for information
    println!("{}", "Creating a new verilog project...".yellow().bold());

    print!("{}", "Please enter the project name: ".yellow().bold());
    stdout().flush().unwrap();
    let mut nme = String::new();
    stdin().read_line(&mut nme).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    nme = nme.trim().to_string();
    while nme.chars().count() == 0 {
        print!("{}", "Name is invalid, please try again: ".red().bold());
        stdout().flush().unwrap();
        stdin().read_line(&mut nme).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
        nme = nme.trim().to_string();
    }

    print!("{}", "Please enter a short project description: ".yellow().bold());
    stdout().flush().unwrap();
    let mut desc = String::new();
    stdin().read_line(&mut desc).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    desc = desc.trim().to_string();
    while desc.chars().count() == 0 {
        print!("{}", "Description is invalid, please try again: ".red().bold());
        stdout().flush().unwrap();
        stdin().read_line(&mut desc).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
        desc = desc.trim().to_string();
    }

    print!("{}", "Please enter a project version (Default: 0.1.0): ".yellow().bold());
    stdout().flush().unwrap();
    let mut ver = String::new();
    stdin().read_line(&mut ver).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    ver = ver.trim().to_string();
    if ver.chars().count() == 0 {
        ver = "0.1.0".to_string();
    }

    print!("{}", "Please enter your name (Default: \"\"): ".yellow().bold());
    stdout().flush().unwrap();
    let mut rname = String::new();
    stdin().read_line(&mut rname).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    rname = rname.trim().to_string();
    if rname.chars().count() == 0 {
        rname = "".to_string();
    }

    print!("{}", "Please enter a license (Default: MIT): ".yellow().bold());
    stdout().flush().unwrap();
    let mut lic = String::new();
    stdin().read_line(&mut lic).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    lic = lic.trim().to_string();
    if lic.chars().count() == 0 {
        lic = "MIT".to_string();
    }

    // Create the json
    let package = Package {name: nme, version: ver, description: desc, dependencies: Some(Vec::new()), author: Some(rname), license: Some(lic)};
    let j = serde_json::to_string_pretty(&package).unwrap();

    let f = File::create("./package.json").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(j.as_bytes()).expect("Unable to write data");

    println!("{}", "Project created! Begin by adding dependencies to your package.json.".green().bold())
}

fn main() {
    // Create help, version, and validation
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
            // Create a project
            init_project();
        },
        "install" => {
            // Install dependencies
            // TODO: Make a lock file
            let p = read_package_json(&"./package.json").unwrap();
            let deps = p.dependencies.unwrap_or(Vec::new());
            // repeat
            for d in &deps {
                get_dependencies(".", d);
            }
        },
        "update" => {
            // Update dependencies
            println!("Not Implemented yet...");
        },
        _ => unreachable!()
    }

}
