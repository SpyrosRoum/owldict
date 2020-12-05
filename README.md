# owldict
- - -
owldict is a command line dictionary that uses [owlbot](https://owlbot.info) to find definitions and examples of words.

# Build
You can install owldict from source if you have Rust installed on your system:
```shell
$ git clone https://github.com/SpyrosRoum/owldict.git
$ cd owldict
$ cargo build --release
```

# Set up
You need to get a token from owlbot for owldict to work correctly.
Get your owlbot token from https://owlbot.info.  
Then you will need to set an the `OWLDICT_TOKEN` environment variable.  
On linux it's as simple as running `export OWLDICT_TOKEN={Your token}`
