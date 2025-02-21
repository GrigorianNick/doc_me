# doc_me
Sometimes editing one file means you need to remember to go update another file, but nothing in your CI/CD pipeline tells you to. For example, if you update your UI then documentation screenshots need to be redone as well

`doc_me` attempts to remedy this by building a mapping from files and their dependents, then cross-referencing it against `git diff` to make sure you updated everything

# How to use
## Requirements
1. `git`
## Adding mappings
`doc_me -m root_file dep_file1 dep_file2`
## Checking to see if you forgot any dependencies
`doc_me -b branch_name` (`branch_name` defaults to `develop`)