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
$ todo -e ts tsx -r 4

.ts-project/src/app.ts 
│
└── IMPORTANT  
│  ✪ line 67:
│     this is an high priority todo
│
└── TODOs  
│  ✪ line 20:
│     this is a normal priority todo 
│
└── OTHER  
   ✪ line 75:
      this is a low priority todo 

```
