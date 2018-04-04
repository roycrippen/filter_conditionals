# filter_conditionals

## install
```
git clone ....
cargo install
cd directory/with/targets.txt
filter_conditionals -h
```

### linux steps
1. help
```
filter_conditionals -h
```

2. create a `targets.txt` file like this example:
```
NRTSIM
RTCLSIM
```

3. build `affirmative_targets.txt` and `non_affirmative_targets.txt`
```
filter_conditionals path/to/source -b
```

4. modify affirmative_targets.txt and non_affirmative_targets.txt manually as needed

5. explore the results of removing targets WITHOUT changes to source files
```
filter_conditionals path/to/source -e
```
6. remove targets; backup made of each file (my_file.cc -> my_file.cc.original)
```
filter_conditionals path/to/source -r
```

7. undo remove targets
```
filter_conditionals path/to/source -u
```
