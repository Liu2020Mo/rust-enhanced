// SYNTAX TEST "Packages/Rust Enhanced/RustEnhanced.sublime-syntax"

struct BasicStruct(i32);
// ^^^^^^^^^^^^^^^^^^^^ meta.struct
// <- storage.type.struct
//^^^^ storage.type.struct
//     ^^^^^^^^^^^ entity.name.struct
//                ^ punctuation.definition.group.begin
//                 ^^^ storage.type
//                    ^ punctuation.definition.group.end

struct PrintableStruct(Box<i32>);
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ meta.struct
// <- storage.type.struct
//^^^^ storage.type.struct
//     ^^^^^^^^^^^^^^^ entity.name.struct
//                    ^ punctuation.definition.group.begin
//                     ^^^^^^^^ meta.generic
//                        ^ punctuation.definition.generic.begin
//                         ^^^ storage.type
//                            ^ punctuation.definition.generic.end
//                             ^ punctuation.definition.group.end

struct Nil;
// ^^^^^^^ meta.struct
//        ^ - meta.struct

struct Pair(i32, i32);
// ^^^^^^^^^^^^^^^^^^ meta.struct
//          ^^^ storage.type
//               ^^^ storage.type
//                   ^ - meta.struct

struct Point
// ^^^^^^^^^ meta.struct
{
// <- meta.struct meta.block punctuation.definition.block.begin
    x: i32,
//  ^ variable.other.member
//   ^ punctuation.separator
//     ^^^ storage.type
    y: i32
//  ^ variable.other.member
//   ^ punctuation.separator
//     ^^^ storage.type
}
// <-  meta.block punctuation.definition.block.end

impl Point
//^^^^^^^^ meta.impl
{
// <- meta.impl meta.block punctuation.definition.block.begin
    fn new(x: i32, y: i32) -> Point
    // <- storage.type.function
    // ^^^ entity.name.function
    {
    // <- meta.function meta.block
        Point {x: x, y: y}
    }

    fn double(&mut self) {
    // ^^^^^^ entity.name.function
        self.x *= 2;
        self.y *= 2;
    }
}

// TODO: `meta.group` should cover the closing parenthesis.
struct Val (f64,);
//^^^^^^^^^^^^^^^ meta.struct
//     ^^^ entity.name.struct
//         ^^^^^ meta.group
//         ^ punctuation.definition.group.begin
//          ^^^ storage.type
//              ^ punctuation.definition.group.end
//               ^ punctuation.terminator

// TODO: Fix extern.
struct F {
    f: extern "C" fn(x: u8, ... /* comment */),
    g: extern "C" fn(x: u8, /* comment */ ...),
    h: extern "C" fn(x: u8, ...),
    i: extern "C" fn(
        x: u8,
        // comment 4
        y: String, // comment 3
        z: Foo,
        // comment
        ... // comment 2
    ),
}

let mut j = BasicStruct(10);
//  ^^^ storage.modifier
//                      ^^ constant.numeric.integer.decimal

let p = Point {x: 10.0, y: 20.0};
//      ^^^^^ storage.type.source
//            ^^^^^^^^^^^^^^^^^^ meta.block
//            ^ punctuation.definition.block.begin
//              ^ punctuation.separator
//                ^^^^ constant.numeric.float
//                             ^ punctuation.definition.block.end
let n = NothingInMe {};
//      ^^^^^^^^^^^ storage.type.source
//                  ^^ meta.block
let tp = TuplePoint { 0: 10.0, 1: 20.0 };
//                    ^constant.numeric.integer.decimal
//                             ^ constant.numeric.integer.decimal
let p = Point { x, y };
//      ^^^^^ storage.type.source
//            ^^^^^^^^ meta.block
