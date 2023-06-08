# `leet`

Generate (custom) starter templates for Leetcode problems, pulled directly from Leetcode!

## Usage

For an example template `cpp-make` and the iconic [Two Sum](https://www.leetcode.com/problems/two-sum), run this -- 

```
leet new cpp-make https://leetcode.com/problems/two-sum
```

Here's the usage -

```
Leet 
Swapnil Kumbhar <https://github.com/SwapnilKumbhar>
Custom starter templates for Leetcode.

USAGE:
    leet [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>    Provide a custom config file
    -h, --help               Print help information

SUBCOMMANDS:
    help              Print this message or the help of the given subcommand(s)
    new               Create a new project
    show-templates    Show currently supported languages
```

Create a new Project -- 

```
Create a new project

USAGE:
    leet new [OPTIONS] <TEMPLATE> <LINK>

ARGS:
    <TEMPLATE>    
    <LINK>        

OPTIONS:
    -c, --config <CONFIG>    Provide a custom config file
    -h, --help               Print help information
```

```
$ leet new cpp myCppProject
```

Show all supported languages in the current config -- 

```
$ leet show-languages
```

## Roadmap

The Project tab has more information, but here's the overview --

- [x] Template validity checks.
- [x] Template variables.
- [x] Enable pulling starter scaffolding directly from Leetcode.
- [ ] Starter templates for all Leetcode supported languages.
- [ ] Unit tests.
- [ ] Document everything!

## License

BSD-3-Clause.
