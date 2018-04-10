# veritas
A Package Manager for Verilog Projects

Veritas is designed to decrease confusion in your verilog workflow, and to simplify dependency management in hardware deisgn.

## Installing

### Requirements
You must have Rust installed along with Cargo to install Veritas

### Install
To install, simply go into the `veritas` directory and run `cargo install`

## Getting Started

### Create a project
Simply run `veritas init` and follow the oncsreen prompts to create a veritas verilog project. This will create a project tree like this:
```
.
├── my_verilog.v
└── package.json

```
Where `my_verilog.v` is your existing verilog code.

### Installing dependencies
Dependencies in Veritas are simply git repositories for other veritas projects. Currently, packages hosted on GitHub are the only ones that are supported, but custom git server support is coming soon™.

Dependencies are installed using the `veritas install` command

Dependencies are managed through your `modules.toml` file, which has a structure like this:
```toml
[package]
name = 'example'
version = '0.1.0'
description = 'An example project'
author = 'your name here <your email here>'
license = 'your choice of license'

[repository]
url = 'source code repo url'
version_control = 'version control type'

[dependencies]
"my_first_dependency" = "required version"
```
Dependencies are managed under the `dependencies` tag, where each dependency is a key value pair, thus, for a project with multiple dependencies, you will have:
```toml
[dependencies]
"my_first_dependency" = "required version"
"my_second_dependency" = "required version"
```

If your project has no dependencies, a `dependencies` you can leave the area empty but do not remove the section. 

Dependencies are then created in a `veritas_modules` directory, and can be used in your verilog project through the `include` keyword as such:
```
`include "veritas_modules/some_module/some_module.v"
```

#### Getting dependencies 
While right now veritas assumes a git repo hosted on github, in the future there will be a module registry containing the metadata and blobs of modules. When that transition happens the namespacing of modules will depend on the layout of the registry not github username and repo name 

#### Other notes
It should be noted that when you install dependencies, existing dependencies are not updated when new installs are done.

### Updating Dependencies (TODO)
To update all dependencies (if they are not locked to a commit) simply run `veritas update`


## Current Work
 - Create Project ✅
 - Update Dependencies
 - Module Registry and Explorer 
 - Adding of dependencies from the CLI 
 - Cycle detection
 - Dependecy resolution
 - Build step (flat packing verilog code for compilation)
 - Module upload 

## Example
An example project with dependencies can be found here: https://github.com/veritas-verilog/four_bit_adder
