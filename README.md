# todo

A CLI to parse todo notes in any utf-8 file

notes syntax:

```
@todo @high {message}
```
```
@todo {message}
```
```
@todo @low {message}
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
└── •1 
│  implement some cache
│
└── •2 
   refactor main.rs mess

```
