# veritas
A Package Manager for Verilog Projects

Veritas is designed to decrease confusion in your verilog workflow, and to simplify dependency management in hardware deisgn.

## Getting Started

### Create a project (TODO)
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

Dependencies are managed through your `package.json` file, which has a structure like this:
```
{
  "name": "my_project",
  "version": "1.0.0",
  "description": "A example project json",
  "dependencies": [
      {
          "name": "some_git_user/some_git_repo"
      }
  ],
  "author": "You go here",
  "license": "Some license"
}

```
Dependencies are managed under the `dependencies` tag, where each dependency is a JSON object, thus, for a project with multiple dependencies, you will have:
```
"dependencies": [
      {
          "name": "some_git_user/some_git_repo"
      },
      {
          "name": "some_git_user/some_other_git_repo"
      },
      ...
  ],
```

If your project has no dependencies, a `dependencies` array must still be provided, even if it is empty. 

Dependencies are then created in a `veri_modules` directory, and can be used in your verilog project through the `include` keyword as such:
```
`include "veri_modules/some_module/some_module.v"
```

#### Getting dependencies from specifc branches or commits (TODO)
Sometimes it is useful to get a specific branch of a dependency or clone a dependency at a specific commit. This can be done by specifiying the option in the `package.json` as follows:
```
"dependencies": [
      {
          "name": "some_git_user/some_git_repo",
          "commit": "97dc64341640deff370e5ffa13382f42bf303b51"
      },
      {
          "name": "some_git_user/some_other_git_repo"
          "branch": "dev"
      },
      {
          "name": "some_git_user/some_other_git_repo"
          "commit": "97dc64341640deff370asdfw33382f42bf303b51"
          "branch": "dev-testing"
      },
      ...
  ],
```
You can specify, branch, commit (by hash), or both, and veritas will handle the rest. When specifying a commit, it should be noted that veritas will lock that dependency to that commit, and not update it.

#### Other notes
It should be noted that when you install dependencies, existing dependencies are not updated when new installs are done.

### Updating Dependencies (TODO)
To update all dependencies (if they are not locked to a commit) simply run `veritas update`


## Current Work
 - Create Project
 - Update Dependencies
 - Specify branch or commit to clone

## Example
An example project with dependencies can be found here: https://github.com/veritas-verilog/four_bit_adder
