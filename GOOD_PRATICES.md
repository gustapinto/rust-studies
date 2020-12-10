# Rust Coding Best Pratices

The current Rust best pratices as defined per **[rustfmt][1]** configs.

- **Maximum width of each line**: 100 columns
- **Ideal width of each line**: 80 columns
- **Number of spaces per tab**: 4 spaces
- **Maximum width of the args of a function call¹**: 60 columns
- **Maximum width in the body of a struct lit¹**: 16 columns
- **Windows or UNIX line endings**: UNIX line ending
- **Brace style for functions**: Same line as function declaration

¹ Before falling back to vertical formatting style.


[1]: https://github.com/nrc/rustfmt/blob/3ce2b840cf73253cedf86aaf3b6b7a71e27d6b79/src/config.rs#L259-L304