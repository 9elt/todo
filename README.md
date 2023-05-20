# todo

A CLI tool to parse [todo notes](#todo-notes-syntax) in any utf-8 file

## todo notes syntax

```
@todo @high {some message}
```

reference a line
```
@todo {some message [line]}
```

reference a range of lines
```
@todo @low {some message [from:to]}
```

Spaces, new lines ,`/` and `#`, are ignored.  
The following syntax is valid and equivalent to the first example:

```rust
// @todo   @high
// {
//   some message
// }
```

## CLI usage

`-r` recursion max depth *(defaults to 5)*

`-e` target file extensions

`-d` target a directory *(from current)*

`-i` show high priority only

see [main.rs](./src/main.rs )
```
$ todo -e rs -r 4

 ./src/main.rs  
│
└─ᐅ 5  you can reference lines:
│  
│   77|          let mut result = Parser::new(&file).parse();
│  
│  will print line 77
│
└─ᐅ 7  or ranges:
│  
│  100|  fn relative_path(cd: &PathBuf, path: &Path) -> String {
│  101|      path.to_string_lossy()
│  102|          .replace(cd.to_string_lossy().as_ref(), ".")
│  103|  }
│  
│  to print lines 100 to 103
│
└─ᐅ 1  implement some cache
│
└─ᐅ 3  refactor main.rs mess

```

## installation

```
cargo install --git https://github.com/9elt/todo
```

see: 
[**installing binaries with cargo install**](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html), [**install rust and cargo**](https://doc.rust-lang.org/cargo/getting-started/installation.html)
