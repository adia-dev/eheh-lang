# [eheh-lang - Version 0.1.0 👾](#eheh-lang---version-010-)

Lately a lot of new programming languages have been popping up. Some of them are really cool and offer qualities that others do not provide.
My goal is absolutely not to create a new programming language that will be used by millions of people, but rather to learn how to create a programming language by implementing the features that I like from other languages.

For an in-depth look at my progress, you can find the 🗺️ roadmap [here](#🗺️-roadmap). Additionally, you can explore practical examples of what you can achieve using the language [here](#📦-installation-and-run-🚀).

## 📚 Table of Contents

- [eheh-lang - Version 0.1.0 👾](#eheh-lang---version-010-)
  - [📚 Table of Contents](#-table-of-contents)
  - [📖 About](#-about)
  - [📦 Installation and Run 🚀](#-installation-and-run-🚀)
  - [🧙‍♂️ There are some pretty amazing warnings and errors too!](#-there-are-some-pretty-amazing-warnings-and-errors-too)
  - [🌟 Getting Started -](#-getting-started--)
  - [🗺️ Roadmap](#-roadmap)
  - [🤝 Contributing](#-contributing)
  - [📜 License](#-license)
  - [📞 Contact](#-contact)


## 📖 About

eheh-lang represents my educational endeavor into the realm of programming languages. This dynamically-typed, interpreted, general-purpose programming language serves as a platform for learning and exploration.

I have stumbled upon this book thanks to [ThePrimeagen](https://github.com/ThePrimeagen), it is an amazing book that I would recommend to anyone who wants to learn how to create a programming language: [Writing An Interpreter In Go](https://interpreterbook.com/), it truly is worth buying !!!

I am a big fan of the Rust programming language, so I decided to implement the language in Rust.

There are a lot of features that I would like to implement in the language that comes from others such has:
     
- [Elixir](https://github.com/elixir-lang/elixir) (I really like the pattern matching and the pipe operator)

- [Zig](https://github.com/ziglang/zig) (I also recommend following [Andrew Kelley](https://github.com/andrewrk)'s work, he is an amazing developer and he is doing some really cool stuff with Zig and has some really nice [Talks](https://vimeo.com/649009599))

- [Swift](https://swift.org/) (I like the way you can call methods on objects with the dot syntax)

- [Rust](https://github.com/rust-lang/rust) and [C++](https://en.wikipedia.org/wiki/C%2B%2B?useskin=vector) (Interesting way to manage memory, pointers, namespace with `::`, etc...)
- [Python](https://www.python.org/) (No limit in terms of integer size)


## [📦 Installation and Run 🚀](#📦-installation-and-run-🚀)

To install and run your programming language, follow these steps:

1. Make sure you have Rust installed. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
The project has been created and maintained with `rustc` and `cargo` [1.69.0](https://github.com/rust-lang/rust/releases/tag/1.69.0)
It should work with newer versions, but if you encounter any problems, please open an issue.


2. Clone this repository to your local machine:
```bash
git clone https://github.com/adia-dev/eheh-lang.git
```

3. Navigate to the project directory:
```bash
cd eheh-lang
```

4. Run the REPL (Read-Eval-Print Loop):
```bash
cargo run
```

5. Enjoy! 🎉 Here are some examples of what you can do with the language:
```bash
eheh(1)> let add = fn(x, y) { x + y; };

eheh(2)> add(2, 3)
5

eheh(3)> let factorial = fn(n) { if (n == 0) { 1 } else { n * factorial(n - 1); } };

eheh(4)> factorial(5)
120

eheh(5)> let name: string = "eheh-lang";
"eheh-lang"

------------------------------------- WORK IN PROGRESS -------------------------------------

eheh(6)> name |> String::to_uppercase |> String::chars |> Vec::len |> print
9

eheh(7)> name ** 2
"eheh-langeheh-lang"

eheh(8)> let hash_map = { language: "eheh-lang", version: { major: 0, minor: 1, patch: 0 }, year: 2023 };
{language: "eheh-lang", version: {major: 0, minor: 1, patch: 0}, year: 2023}

eheh(9)> hash_map |> Map::each(|key, value| print("{} => {}\n", key, value))
language => eheh-lang
version => {major: 0, minor: 1, patch: 0}
year => 2023

eheh(10)> help
[...]

eheh(11)> exit
~ Prend moi un Yop ~
```

## 🧙‍♂️ There are some pretty amazing warnings and errors too!

```rust
eheh(1)> let add = fn(x, y { x + y; };
error[E01000]: expected `)`, found `{`
  --> src/main.rs:1:17
    |
1   |   let add = fn(x, y { x + y };
    |                   ^ : expected `)`

eheh(2)> if (true) { 1 } else { 2 }
warning[E01000]: Unnecessary parentheses around true
    |
1   |   if (true) { 1 } else { 2 }
    |       ^^^^

eheh(3)> let add = fn(x, y) { x + y; }; // <- define a function, functions are top-level citizens in eheh-lang

eheh(4)> if true {} else { add(1, 2) }
warning[E01002]: The consequence of the if expression is empty. Consider swapping the alternative and consequence branches of the if expression.
    |
1   |   if true {} else { add(1,2) }
    |           ^       ^^^^^^^^^^^^
```


## 🗺️ Roadmap

- [x] Initial project setup
- [x] Implemented AST (Abstract Syntax Tree) structure
    - [x] Expression nodes
        - [x] Boolean expression
        - [x] Call expression
        - [x] Function literal
        - [x] Identifier
        - [x] If expression
        - [x] Infix expression
        - [x] Integer literal
        - [x] Prefix expression
        - [x] Typed identifier
        - [ ] String literal
        - [ ] Array literal
        - [ ] Hash literal
        - [ ] Index expression
        - [ ] For expression
        - [ ] While expression
        - [ ] Match expression
        - [ ] Range expression
        - [ ] Pipe expression (|> Elixir ♥️)
        - [ ] Defer expression
        - [ ] Await expression ??????????? (let's ignore this for now)
        - [ ] Asy--------------------------^^^^^^^^^^^^^^^^^^^
        - [ ] Cool little dot to call local Enums just like in `Zig` or `Swift`
    - [x] Statement nodes
        - [x] Block statement
        - [ ] Declare statements
            - [x] Let statements
            - [x] Const statements
            - [ ] Var statements
            - [ ] Mutability
            - [ ] Functionning type system
        - [x] Expression statements
        - [x] Return statements
- [x] Lexer implementation
- [x] Parser implementation
- [x] REPL (Read-Eval-Print Loop) implementation
- [x] Evaluation system
    - [x] Boolean object
    - [x] Integer object
    - [x] Null object
    - [ ] Prefix expressions
    - [ ] Infix expressions
    - [ ] If expressions
    - [ ] Return statements
    - [ ] Let statements
    - [ ] Function objects
    - [ ] Function application
    - [ ] Closures
    - [ ] Built-in functions
    - [ ] First-class functions
    - [ ] Higher-order functions
    - [ ] String object
    - [ ] Array object
    - [ ] Hash object
    - [ ] Index expressions
- [x] Type system implementation
    - [ ] Type checking
    - [ ] Compile time type checking ???? (Does this language even compile?)
- [x] Error handling
    - [x] Error logging
    - [x] Warning logging
    - [x] Error handling
    - [x] Warning handling
    - [ ] Help logging
- [x] REPL (Read-Eval-Print Loop) implementation
- [ ] Garbage collector
- [ ] Memory Allocators
    - [ ] Stack allocator
    - [ ] Page allocator
    - [ ] Bump allocator
    - [ ] Slab allocator
    - [ ] Object pool allocator
- [ ] LSP (Language Server Protocol) 
- [ ] Standard Library
- [ ] Dynamic Dispatch (Generics ???????)
- [ ] Powerful Pattern Matching (Elixir Like)
- [ ] Module or Namespace system
- [ ] Macros
- [ ] Documentation
- [ ] Code formatting
- [ ] Structs, Enums and Custom Types
- [ ] Package Manager (AHAHAHHAHAHAHAHAHDAZUIHUDIAHUIDHZAUIHDA, maybe something like zig's package manager, e.g: `build.zig.zon`)
- [x] Complete the main program
- [x] Comprehensive testing
- [x] Documentation
- [x] Additional features (Add more items as you progress!)


## 🤝 Contributing

- **Contributions Welcome**: I don't mind a little pull request that implements async/await as well as wasm portability. I'm sure it's not that hard to implement, right? 😅
- **Feedback Welcome**: I'm sure you have some great ideas for this project. Let me know what you think!
- **Issues Welcome**: If you find a bug, let me know! I'll try to fix it as soon as possible.
- **Questions Welcome**: If you have any questions, feel free to open an issue or contact me on Discord: `adia31`

## 📜 License

This project is licensed under the MIT see the [LICENSE](LICENSE) file for details

## 📞 Contact

Discord: `adia31` (preferred)

Linkedin: [Abdoulaye DIA](https://www.linkedin.com/in/abdoulaye-dia-69a1b0208/)
