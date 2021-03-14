# eulercli Templates

Templates for programming solutions to [Project Euler] problems.

## Introduction

This repository contains a set of [Project Euler] solution templates written in [julia] and [go]. 

These templates are designed to be used with [eulercli] to generate ready-to-use "starting points" for solving Project Euler problems. This is a time-saver, especially if you plan on solving multiple problems.

## Usage

To use these templates, you'll need to install the [eulercli] tool.

Next, create a directory for your Project Euler solutions.  We'll refer to this as your "project directory" going forward.

```sh
mkdir projecteuler
cd projecteuler
```

### "Installing" the eulercli templates

There are several ways to "install" these templates into your project directory.

#### Installing with `eulercli pull`

You can copy these templates into your `projecteuler` directory by running `eulercli pull` in that directory.

```sh
eulercli pull
```

At this point, your `projecteuler` directory should look like this

```sh
projecteuler/
    eulercli-templates/
        julia/
            [...]
        go/
            [...]
```

`eulercli pull` downloads template files directly from [this github repository](https://github.com/koomen/eulercli-templates), and you can always update your templates by rerunning `eulercli pull`.  You can also modify these templates locally or add your own--see the [eulercli README][eulercli] for instructions.

#### Installing with `git clone`

From within your `projecteuler` directory:

```sh
git clone https://github.com/koomen/eulercli-templates
```

### Rendering templates for a specific Project Euler problem

Once you've installed these templates in your project directory, you can use `eulercli generate` to render them for a specific Project Euler problem.

For example, if you'd like to use the [julia programming language][julia] to solve [Problem #1](https://projecteuler.net/problem=1), run the following from within your project directory:

```sh
eulercli generate 1 --language julia
```

If you're doing this for the first time, you should see a new `julia` directory in your `projecteuler` directory:

```sh
projecteuler/
    eulercli-templates/
    julia/
        initenv.jl
        README.md
        euler0001/
            solution.jl
```

When you run `eulercli generate` again, eulercli will skip existing files as long as they are unchanged by the operation. So, for example, if you run

```sh
eulercli generate 2 --language julia
```

...you should see the following in your `projecteuler` directory:

```sh
projecteuler/
    eulercli-templates/
    julia/
        initenv.jl
        README.md
        euler0001/
            solution.jl
        euler0002/
            solution.jl
```

### Running and Benchmarking your solution program

Once you've used `eulercli generate` to render your solution files, you can start work on solving your Project Euler problem.

The process of building, running, and benchmarking your solution programs is different for each language.  You can find language-specific instructions in the language README.md files:

- [julia README.md](julia/README.md)
- [go README.md](go/README.md)

Don't forget you can use `eulercli check` to [check your whether your solution is correct](https://github.com/koomen/eulercli/#check-your-answers-using-pipes).

### Adding your own solution program templates

When you run `eulercli generate --language <langauge>`, the tool will look for templates in `./eulercli-templates/<language>`.

If you'd like to create templates for a new language or modify the templates for an existing language, you can do so by saving them in the `./eulercli-templates/<language>` directory.

Template solution files and filenames can include [text/template package](https://golang.org/pkg/text/template/) directives with the following fields:

- `{{.ProblemNum}}` - the problem number (e.g. "42")
- `{{.PaddedProblemNum}}` - the problem number, padded with 0s (e.g. "0042")
- `{{.ProblemText}}` - the problem text (e.g. "The nth term of the sequence of triangle...")
- `{{.Answer}}` - The correct answer to the problem (e.g. "123")
- `{{.AnswerMD5}}` - The correct answer to the problem, hashed using [MD5](https://en.wikipedia.org/wiki/MD5) (e.g. "ba1f2511fc30423bdbb183fe33f3dd0f")

For example, calling

```sh
$ eulercli generate 42 --language julia
```

will render the following template file

```
./eulercli-templates/julia/src/euler{{.PaddedProblemNum}}/solution.jl
```

to the target output file

```
./julia/src/euler0042/solution.jl
```

If you find ways to improve existing template files or create useful new template files for an as-yet-unsupported language, consider [contributing to this project](#contributing)

## Contributing

Code contributions to eulercli-templates are encouraged and appreciated! If you'd like to contribute, clone this repository, commit your proposed changes, and create a pull request.

<!-- Links -->

[eulercli]: https://github.com/koomen/eulercli
[go]: https://golang.org
[julia]: https://julialang.org
[Project Euler]: https://projecteuler.net








