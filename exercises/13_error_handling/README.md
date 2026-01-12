# Error handling

Most errors aren't serious enough to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

# Key words
Error and Error Handling
Take action
debug (default) versus --release (use in cargo cli)
Before deploying to production
Recoverable and unrecoverable errors
Compare to exceptions
Result<T, E>
panic! (implicit, explicit)
RUST_BACKTRACE=1
enum Result<T, E> {...}
Generic type parameters
variant


# Quick Training
## Unrecoverable Error
* To have Rust display the call stack, use environment variable before the program RUST_BACKTRACE=1
* Some std Rust libraries will call panic!. This is called implicit panic!. To call explicitly call panic, use panic! macro.
* Unwinding, clean up, and display call stack take a lot of work. To disable it, in Cargo.toml set
```shell
[profile.release]
panic = 'abort'
```

## Recoverable Error

