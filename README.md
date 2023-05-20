# todo

A CLI tool to parse [todo notes](#todo-notes-syntax) in any utf-8 file

## todo notes syntax

```
@todo @high {some message}
```
```
@todo {some message}
```
```
@todo @low {some message}
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

```
$ todo -e rs -r 4

./src/main.rs  
│
└─ᐅ 1  implement some cache
│
└─ᐅ 2  refactor main.rs mess

```

## installation

```
cargo install --git https://github.com/9elt/todo
```

see: 
[**installing binaries with cargo install**](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html), [**install rust and cargo**](https://doc.rust-lang.org/cargo/getting-started/installation.html)
