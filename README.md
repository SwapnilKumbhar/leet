# `leet`

Generate (custom) starter templates for Leetcode-like problems.

## Usage


```
Leet 
Swapnil Kumbhar <https://github.com/SwapnilKumbhar>
Custom starter templates for Leetcode.

USAGE:
    leet [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>    Provide a custom config file
    -h, --help               Print help information
    -v, --verbose            Toggles verbose logs (INFO level)

SUBCOMMANDS:
    help              Print this message or the help of the given subcommand(s)
    new               Create a new project
    show-languages    Show currently supported languages
```

Create a new Project -- 

```
leet-new 
Create a new project

USAGE:
    leet new [OPTIONS] <LANGUAGE> <NAME>

ARGS:
    <LANGUAGE>    
    <NAME>        

OPTIONS:
    -c, --config <CONFIG>    Provide a custom config file
    -h, --help               Print help information
    -v, --verbose            Toggles verbose logs (INFO level)
```

```
$ leet new cpp myCppProject
```

Show all supported languages in the current config -- 

```
$ leet show-languages
```

## Roadmap

- [ ] Template validity checks.
- [ ] Starter templates for all Leetcode supported languages.
- [ ] Template variables.
- [ ] Enable pulling starter scaffolding directly from Leetcode.

## License

BSD-3-Clause.
