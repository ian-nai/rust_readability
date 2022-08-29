# rust_readability
A package to assess the complexity of texts using a variety of readability formulas, written in Rust. The package includes implementations of the Lix, Rix, Flesch, Flesch-Kincaid, Coleman-Liau, and Automated Readability Index methods.

## How to Use
The package includes functions for the Lix, Rix, Flesch, Flesch-Kincaid, Coleman-Liau, and Automated Readability Index methods. Each function prints and returns its corresponding readability index. Call each method on a file or string like so:

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
coleman_liau("path/to/file.txt");
ari("path/to/file.txt");
```
And for strings:

```
lix_string("your string");
rix_string("your string");
flesch("your string");
flesch_kincaid("your string");
coleman_liau("your string");
ari("your string");
```

