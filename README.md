# rust_readability
A package to assess the complexity of texts using a variety of well-established readability formulas written in Rust. The package includes implementations of the Lix, Rix, Flesch, Flesch-Kincaid, Linsear Write, Coleman-Liau, and Automated Readability Index methods.

## Usage
The package includes functions for the Lix, Rix, Flesch, Flesch-Kincaid, Linsear Write, Coleman-Liau, and Automated Readability Index methods. When presented with either a file or a string, each function prints and returns a corresponding readability index. Call each function like so:

```
use rust_readability::lix;

lix("path/to/file.txt");
// or, for a string: lix_string("your string");
```
To remove stopwords before assessing the readability of a text, use the functions stopwords_file (to remove stopwords from a file) and stopwords_string (to remove stopwords from a string), like so:

```
use rust_readability::stopwords_string;
stopwords_string("this is an example string", "[your language]");

use rust_readability::stopwords_file;
stopwords_file("path/to/file.txt", "[your language]");
```

All languages included in NLTK's stopwords lists can be used to remove stopwords:
1. Arabic 
2. Azerbaijani
3. Danish
4. Dutch
5. English
6. Finnish
7. French 
8. German 
9. Greek
10. Hungarian 
11. Indonesian
12. Italian 
13. Kazakh 
14. Nepali 
15. Norwegian 
16. Portuguese 
17. Romanian 
18. Russian 
19. Slovene 
20. Spanish 
21. Swedish 
22. Tajik 
23. Turkish

## Readability Metrics 

### Lix 
[Lix](https://en.wikipedia.org/wiki/Lix_(readability_test)) is a readability measure first established by Carl-Hugo Björnsson. For text, the Lix value is derived by summing the average sentence length and the percentage of words longer than six letters. 
```
lix("path/to/file.txt")
lix_string("your string");
```

### Rix 
Also developed by Björnsson, the Rix value is the number of longer words (>6 letters) divided by the number of sentences. 
```
rix("path/to/file.txt")
rix_string("your string");
```

### Flesch
The [Flesch reading-ease test](https://en.wikipedia.org/wiki/Flesch%E2%80%93Kincaid_readability_tests) evaluates how many words are in each sentence and how many syllables are in each word to provide a score ranging from 0-100, with 0 being the hardest to read. 
```
flesch("path/to/file.txt");
flesch("your string");
```

### Flesch-Kincaid
[Flesh-Kincaid](https://en.wikipedia.org/wiki/Flesch%E2%80%93Kincaid_readability_tests) presents the Flesch reading-ease test as a grade-level. 
```
flesch_kincaid("path/to/file.txt");
flesch_kincaid("your string");
```

### Linsear Write 
[Linsear write](https://en.wikipedia.org/wiki/Linsear_Write) is a reading metric developed specifically for technical/scientific texts. The score evaluates syllable count and sentence length in a 100-word sample to present a grade-level metric. 
```
linsear_write("path/to/file.txt");
linsear_write_string("your string");
```
### Coleman-Liau
The [Coleman-Liau index](https://en.wikipedia.org/wiki/Coleman%E2%80%93Liau_index) relies on the average word length in letters and average sentence length in words to produce a grade-level metric. 
```
coleman_liau("path/to/file.txt");
coleman_liau("your string");
```

### Ari 
The [automated readability index](https://en.wikipedia.org/wiki/Automated_readability_index) (ARI) similarly relies on character counts and sentence length to produce a score which uniquely maps to grade-level representation.
```
ari("path/to/file.txt");
ari("your string");
```
## Installation
There are two ways to install rust_readability on your system: through cargo or via source code.
### Cargo
rust_readability can be installed using cargo, the package manager and crate host for rust. To install using cargo, run the following command:
```
cargo add rust_readability
```
### Source Code
To install from the source code you will can clone the repository:
```
git clone https://github.com/ian-nai/rust_readability.git
```
