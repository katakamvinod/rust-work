# Craits are packages in Rust
 crates.io is the source
# Channels (stable,beta,nightly(HEAD of master and Unstable))
* rustup toolchain install (required)
## Stack , Heap,Pointers,smart pointers
## Stack :
 memory for function (variable internal to function) 
 for every function cal a new stack frame is allocated on top of the current one
 the size of every variable on the stack has to be know at compile time
 after function exists stack is relased

# Heap
 manually allocate and relese
 no size restrictions 
 accessable by any one
 allocations are expensive and may cause defragmentation
* Smart Pointers
** make sure it frees the memory allocations
** Memory Layout
* Building a command line Application
** Functions
** Basic Data Types
*** Boolean,Charecters,Integers,Floats
*** u8,i8 (u -unsigned,signed(can be + or -ve))(number represent size 8-128 bits)
*** uzise and isize(32 bits or 64 bits)
*** two float (f32 and f64)
*** boolean(1 byte)
*** char (single unicode value always 4 char ASCII will waste)
** Standard Library
** Memory Ownership
*** Funtions***
*** Snake case all letters are lowercase and _seperates
***  main i Mutability ***
*** varaible let and name of variable
*** immutable and mutable ( is immutable or everything is immutable) , explisityly use mut
*** standard Library ***  
*** use keyword
* Ownership & Borrowing*** 
 RAII
 Rules (1) each value in rust is owned by a varaible ,(2) when owener goes out of scope , value will be deallocated,(3) ther can be ony on owner at atime
 reference (&type,&mut type)(mutable and immutable)
 Just borrowing 
 references are immutable
 once we use mutable borrow cannot use immutable  any more
 As many immutable references or only one mutable reference
 result can be in two states (SUCCESS or Failed)
 unwrapped (if error will terminate else Yield)
* HTTP Server
Layer 7 over TCP
 Message based
* STRUCTS
class
methods are funtions in structs ,always take self as first
associated funtion , similar to static ::
String vs str(literal) -> slice immutable referece to string
* Enums
finite set of values
* Option Enum
Null ("No" as not null values)
typesafe way of defining Null
None or Some(T)
T can be of any type generic
# Modules
inline module by mod
every thing is private , explicity state "pub"
# Result
error two categories (Recoverable and non Recoverable)
OK ,ERR
# infinite loop 
annotation
loops
infinite loops
loop{
brek  or continue to jump
}
# Tuples
grouping multiple type to one ,fixed size 
# Match Expression
compare a value against a seris of values 
# Traits
similar to interfaces
tryfrom and try into

# lifetimes
after deallocating the main , what will happen ? use after free,dangling references
so if we are referring to some variabnle what will happen if we refer to that 
to statically check we use lifetime specifiers
gaurentee memory saftey , only allows us to instruct to compiler
# Dynamic Dispatch VS Static Dispatch
desired implementation resolved at rumtime
mapping Vtable / static dispatch dyn -> impl
Static dispatch is true duplication for every implementation