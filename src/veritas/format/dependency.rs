use std::process;
use std::collections::BTreeMap;
use toml::Value;
use semver::Version;

use veritas::consoleio::output::*;

//
//  A dependency only needs a name, which is in the format "username/git-repo". The latest master
// commit is fetched
// If a commit is provided, the dependency will be cloned from that commit
// If a branch is provided, the dependency will be cloned from the latest commmit in that branch.
//  If both commit and branch are provided, then the commit from that branch will be cloned.
//
#[derive(Debug, Serialize)]
pub struct Dependency {
    pub name: String,
    pub version: Version,
}

impl Dependency {
    pub fn parse_dep_info(deps: &Value) -> Option<Vec<Dependency>> {
        let info_tree = BTreeMap::from(deps.as_table().unwrap().clone());
        println!("{:?}", info_tree.keys());
        let deps_map = match deps.as_table() { 
            Some(table) => table,
            None => { 
                print_to_console(Status::Err, "Unable to Parse Dependencies");
                process::exit(1);
            }
        };


        if deps_map.len() > 0 {
            return Some(deps_map.iter().map(|(k,v)| {
                let mut ver: String = v.to_string();
                ver = ver[1..(ver.len() - 1)].to_string();
                Dependency {
                    name: k.to_string(),
                    version: match Version::parse(&ver) {
                        Ok(ver) => ver,
                        Err(err) => {
                            print_to_console(Status::Err, &format!("Unable to parse version info\n\
                                                                        Specifically: {}", err));
                            process::exit(1);
                        }
                    }
                }
            }).collect::<Vec<Dependency>>());
        }     
        return None;
    }
}