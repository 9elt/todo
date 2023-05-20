# todo

A CLI tool to parse *todo notes* in any utf-8 file

### todo notes syntax:

```
@todo @high {some message}
```
```
@todo {some message}
```
```
@todo @low {some message}
```
*Spaces, new lines ,`/` and `#`, are ignored*, so the following syntax is valid:
```rust
// @todo
// @high
// {
//   some message
// }
```

### basic usage

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
