# Theseus

`theseus` is a library for reporting errors in text files, which is something 
useful that you might want to do when writing a compiler. It wraps the excelent
Rust library `ariadne` using a Pythonic interface.

## Installing 

Just `pip install theseus` or use your prefered method for organizing virtual 
enviroments and dependencies.

## Using the Library in Python

After building the project, you can use the library in your Python code as follows:

```python
from theseus import Report
from pathlib import Path

# Consider a file "program.lox" with the content:
# print("Hello World!");
path = Path("program.lox")

# We can create a report warning the user of a misuse of the print command.
report = Report(
   path,
   start=5,
   end=21,
   message="In Lox, the print command does not require parenthesis",
   kind="warning",
)
report.label(
   start=5,
   end=6,
   message="Parenthesis start here",
   color=(color := report.color()),
)
report.label(
   start=20,
   end=21,
   message="And end here",
   color=color,
)

report.print()
```

Ariadne should output something like

```
Warning: In Lox, the print command does not require parenthesis
   ╭─[ /home/chips/git/compiler/theseus/tests/example.lox:1:6 ]
   │
 1 │ print("Hello World!");
   │      │              │  
   │      ╰──────────────┼── Parenthesis start here
   │                     │  
   │                     ╰── And end here
───╯
```

## Constributing

### Prerequisites

- Rust (latest stable version)
- Python (3.10 or higher)
- `maturin` for building the Python package

### Project Structure

```
theseus
├── src
│   ├── lib.rs        # Rust library code with PyO3 bindings
│   └── main.rs       # Entry point for the Rust application
├── Cargo.toml        # Cargo configuration file
├── pyproject.toml    # Python project configuration
└── README.md         # Project documentation
```

### Building the Project

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-pyo3-ariadne
   ```

2. Build the Rust library:
   ```
   cargo build --release
   ```

3. Build the Python package using `maturin`:
   ```
   maturin build
   ```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.


## Why Theseus? Why not simply name this lib Ariadne?

Because another project named Ariadne already exists in PyPI. So I picked up 
[Ariadne's lover](https://en.wikipedia.org/wiki/Ariadne#Minos_and_Theseus), 
which perhaps together can be used to slay a Minotaur `;-]`