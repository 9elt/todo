# todo

A CLI tool to parse [**todo notes syntax**](#todo-notes-syntax) in any utf-8 file

## todo notes syntax

To start a **todo note** use **`@todo`**, indicate an **optional** priority with **`@high`** or **`@low`**, finally provide a **message** wrapped by **`{}`**

```
@todo @high { some message }
```

you can **reference a line** in the file using **`[line-number]`** or a **range** using **`[from:to]`**:

```
@todo { some message [12] [12:15] }
```

references can be **RELATIVE** to the current line using **`[+number]`** or **`[+from:+to]`**:

```
@todo @low { some message [+1] [+1:+5] }
```

Spaces, new lines ,`/` and `#`, are ignored. The following syntax is **valid** and equivalent to the first example:

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

```
$ todo -e rs -r 4
```

The output of [**main.rs**](./src/main.rs ) todo notes:
```
 ./src/main.rs  
│
└─ᐅ 3  you can reference lines:
│  
│   74|          let mut result = Parser::new(&file).parse();
│  
│  will print line 74
│
└─ᐅ 7  or ranges:
│  
│   97|  fn relative_path(cd: &PathBuf, path: &Path) -> String {
│   98|      path.to_string_lossy()
│   99|          .replace(cd.to_string_lossy().as_ref(), ".")
│  100|  }
│  
│  to print lines 97 to 100
│
└─ᐅ 9  reference next 2 lines
   
    10|  mod parser;
    11|  mod util;

```

## installation

```
cargo install --git https://github.com/9elt/todo
```

see: 
[**installing binaries with cargo install**](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html), [**install rust and cargo**](https://doc.rust-lang.org/cargo/getting-started/installation.html)
