# rust_readability
This is a package to assess the complexity of texts using a variety of well-established readability formulas written in Rust. The package includes implementations of the Lix, Rix, Flesch, Flesch-Kincaid, Linsear Write, Coleman-Liau, and Automated Readability Index methods.

## Usage
The package includes functions for the Lix, Rix, Flesch, Flesch-Kincaid, Linsear Write, Coleman-Liau, and Automated Readability Index methods. When presented with either a file or a string, each function prints and returns a corresponding readability index. Call each function like so:

```
use rust_readability::lix;

lix("path/to/file.txt");
// or, for a string: lix_string("your string");
```

The full list of function names is as follows:
```
lix("path/to/file.txt");
rix("path/to/file.txt");
flesch("path/to/file.txt");
flesch_kincaid("path/to/file.txt");
linsear_write("path/to/file.txt");
coleman_liau("path/to/file.txt");
ari("path/to/file.txt");
```
And for strings:

```
lix_string("your string");
rix_string("your string");
flesch("your string");
flesch_kincaid("your string");
linsear_write_string("your string");
coleman_liau("your string");
ari("your string");
```
## Installation
There are two ways to install rust_readability on your system: through Cargo or via source code.
### Cargo
rust_readability can be installed using cargo, a tool for installing Rust libraries. To do it, run the next command:
```
cargo add rust_readability
```
### Source Code
To install from the source code you will need to clone the repository first:
```
git clone https://github.com/ian-nai/rust_readability.git
cd rust_readability
```


