#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate git2;
extern crate serde;
extern crate clap;
extern crate colored;
extern crate semver;

use std::path::Path;
use std::io::{Write, Read, BufWriter};
use std::fs::File;
use std::fs::create_dir_all;
use std::process;
use git2::build::{ RepoBuilder };
use clap::{Arg, App};


mod veritas;
use veritas::format::{parse_modules_toml, Project, Package, Repository, Dependency};
use veritas::consoleio::output::{Status, print_to_console};
use veritas::consoleio::input::get_console_input;

fn get_repo(repo: &str, clone_path: &str) {
    print_to_console(Status::Special, &format!("Getting dependency {}...", repo));

    let mut builder = RepoBuilder::new();

    //TODO: Support other locations for git repos 
    let _repo = builder.clone(&format!("https://github.com/{}.git", repo), Path::new(clone_path)).expect("Error cloning repo");
}


fn read_modules_toml<P: AsRef<Path>>(path: P) -> Project {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            print_to_console(Status::Err, "modules.toml not found");
            process::exit(1);
        }
    };

    let mut manifest_content = String::new();
    match file.read_to_string(&mut manifest_content) {
        Ok(_) => (),
        Err(_) => {
            print_to_console(Status::Err, "Failed to read modules.toml");
            process::exit(1)
        }
    }

    let proj: Project = match toml::from_str(&manifest_content) {
        Ok(parsed_toml) => parse_modules_toml(&parsed_toml),
        Err(_) => {
            //Make better errors
            print_to_console(Status::Err, "Unable to parse modules.toml");
            process::exit(1)
        }
    };
    return proj;
}

fn get_dependencies(current_path: &str, package: &Dependency) {
    // Check if veritas_modules directory exists, otherwise, make it
    let name = package.name.split("/").collect::<Vec<&str>>()[1];
    let path_name = format!("{}/veritas_modules/{}", current_path, name);
    match create_dir_all(&path_name) {
        Ok(_) => {},
        Err(_) => {
            print_to_console(Status::Err, "Could not create module directory");
            process::exit(1)
        }

    };
    // Clone our first dependency in the veritas_modules directory, with the name of the module
    get_repo(&package.name, &path_name);
    // get_dependencies of the package we cloned
    let p = read_modules_toml(format!("{}/modules.toml", &path_name));
    let deps = match p.dependencies {
        Some(dep_vec) => dep_vec,
        None => Vec::new()
    };
    // repeat
    for d in &deps {
        get_dependencies(&path_name, d);
    }
}


/*
 * Create a project in the current directory
 */
fn init_project() {
    //TODO: Check for file already existing
    // Ask the user for information
    print_to_console(Status::Special, "Creating a new veritas project...");

    let nme = match get_console_input("Please enter the project name: ", "Name", false) {
        Some(name) => name,
        None => {
            print_to_console(Status::Err, "Name is required");
            process::exit(1);
        }
    };

    let desc = match get_console_input("Please enter a short project description: ", "Project Description", false) {
        Some(desc) => desc,
        None => {
            print_to_console(Status::Err, "Project description is required");
            process::exit(1);
        }
    };

    let ver = match get_console_input("Please enter a project version (Default: 0.1.0): ", "Version", true) {
        Some(ver) => ver,
        None => "0.1.0".to_string(),
    };


    let author_name = match get_console_input("Please enter your name (Default: \"\"): ", "Author", true) {
        Some(name) => name,
        None => "".to_string(),
    };
    

    let license = match get_console_input("Please enter a license (Default: \"\"): ", "License", true) {
        //MAY WANT TO CHECK WITH SPDX
        Some(license) => license,
        None => "".to_string(),
    };

    let repo = match get_console_input("Please enter a repository url (Default: \"\"): ", "Repository", true) {
        Some(loc) => loc,
        None => "".to_string(),
    };

    // Create the modules.json
    let project = Project {
        package: Package {
            name: nme,
            version: ver,
            description: desc,
            author: Some(author_name),
            license: Some(license),
        },
        repository: Repository {
            version_control: "git".to_string(),
            url: repo
        },
        dependencies: None,
    };

    let mut t = match toml::to_string_pretty(&project) {
        Ok(t) => t,
        Err(_) => {
            print_to_console(Status::Err, "Unable to serialize project info");
            process::exit(1);
        }   
        
    };
    t = format!("{}\n[dependencies]\n", t);

    let f = match File::create("./modules.toml") {
        Ok(file) => file,
        Err(_) => {
            print_to_console(Status::Err, "Unable to create a modules.toml file");
            process::exit(1);
        }     
    };

    let mut f = BufWriter::new(f);
    match f.write_all(t.as_bytes()) {
        Ok(_) => (),
        Err(_) => {
            print_to_console(Status::Err, "Unable to write data to modules.toml");
            process::exit(1);
        }
    };
    print_to_console(Status::Special, "Project created! Begin by adding dependencies to your modules.toml");
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
            let p = read_modules_toml(&"./modules.toml");
            let deps = match p.dependencies {
                Some(dep_vec) => dep_vec,
                None => Vec::new()
            };
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
