use toml::Value;
use std::collections::BTreeMap;
use veritas::format::Package;
use veritas::format::Repository;
use veritas::format::Dependency;


#[derive(Debug, Serialize)]
pub struct Project {
    pub package: Package,
    pub repository: Repository,
    pub dependencies: Option<Vec<Dependency>>
}


pub fn parse_modules_toml(manifest: &Value) -> Project {
    let info_tree = BTreeMap::from(manifest.as_table().unwrap().clone());
    println!("{:?}", info_tree.keys());
    Project {
        package: Package::parse_package_info(&manifest["package"]),
        repository: Repository::parse_repo_info(&manifest["repository"]),
        dependencies: Dependency::parse_dep_info(&manifest["dependencies"])
    }
}