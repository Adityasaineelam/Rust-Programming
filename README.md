$ rustc --explain rust_basics

> PRIMITIVE DATA TYPES
Rust has several primitive types:
Integers (i8, i16, i32, i64, i128, isize and unsigned variants)
Floating points (f32, f64)
Booleans (true / false)
Characters (char, supports Unicode)
String slices (&str - immutable view of string data)

> COMPOUND DATA TYPES
Tuples group values of different types together (e.g. (i32, f64, char))
Arrays are fixed-size collections [1, 2, 3]
Vectors (Vec<T>) are growable lists

> SHADOWING
Shadowing lets you reuse variable names:
Each 'let' creates a new binding that replaces the old one.
It allows changing type or mutability safely.

> IF / ELSE
Rust's if is an expression:
It can be used to assign values based on conditions.
No parentheses are needed around the condition.

> LOOPS
Rust provides:
loop { } — infinite loop until manually broken
while condition { } — runs while condition is true
for item in collection { } — iterates over items or ranges

> STRUCTS
Structs are custom data types that group related fields.
Used to represent objects with properties.
Methods are implemented using 'impl' blocks.

> ENUMS
Enums define types that can be one of several variants.
They are powerful with 'match' for pattern matching.
Commonly used for state handling and error types.

> ERROR HANDLING
Handled using the Result<T, E> type.
Ok(T) indicates success, Err(E) indicates failure.
Use match, unwrap_or, or the '?' operator to handle results.

> VECTORS
Vectors are resizable arrays.
Created with vec![...] macro.
Support push, pop, and iteration.

> OWNERSHIP
Rust’s core memory safety feature.
Each value has one owner at a time.
When ownership moves, the previous variable becomes invalid.
This prevents double free and dangling pointers.

> BORROWING
Allows access to data without taking ownership.
Immutable borrow: &T (read-only)
Mutable borrow: &mut T (can modify)
Only one mutable borrow can exist at a time.

> FUNCTIONS
Declared using 'fn' keyword.
They define reusable blocks of logic.
Ownership and borrowing rules apply to parameters and returns.

> SUMMARY
Rust ensures:
- Memory safety without a garbage collector.
- No null, no dangling pointers.
- Safe concurrency through ownership and borrowing.
- Compile-time checks that prevent data races.


