# Verustd: Verifying Rust Standard Library

## Installing Verus 
[Link](https://github.com/verus-lang/verus/blob/main/INSTALL.md)

## Running Verification

To run verification, invoke Verus with the crate-type library on the `src/lib.rs` file:

```
$ verus --crate-type=lib src/lib.rs
```
## Cross-crate verification
Currently, Verus works by invoking `rustc` and [Cargo support](https://github.com/verus-lang/verus/pull/1475) is on the way. 

To support cross-crate verification, we need to run commands like these:

For the lib crate
```
verus --crate-type=lib --export vl.vir src/lib.rs
```

For the main crate
```
verus src/main.rs --extern=verified_lib -L target/debug/deps --import verified_lib=../verified_lib/vl.vir
```
## Technical report
[Paper link](./doc/final.pdf)

## Google Slides
[Slides link](https://docs.google.com/presentation/d/1jK3tDHYMTfy9VjzTunskSFhiiWZyaIMGwUaq4xVYIlE/edit?usp=sharing)

## Proof artifacts
- [RawVec](./src/alloc/src/raw_vec.rs)
- [BinaryHeap](./src/alloc/src/collections/binary_heap/mod.rs)
- [Gas](./examples/test_recursive.rs)
- [PointsTo](./examples/test_ptr.rs)

## Limitations of Verus
- Mutable reference(`&mut T`) as return value or struct field not supported 
- `&mut data[i]` not supported
- Deref type conversion not supported
- Comparison operators for non SMT-arith types not supported
- Insufficient external specification for `std` in `vstd`
- Supports atomic primitives with only sequential consistency ordering
- Floating point numbers unsupported

## Difficulties for verification of `std`
- Language items
- `std`-only features 
- High level invariants

## How Verus can help `std`
- Ghost code: specifies all safety invariants explicitly and statically checks them 
    - Number of safety conditions in STD comments
        ```bash
        rg -i --type rust '^\s*//.*safety:' -o | wc -l
        2475

        ```
    - Panic possibility in STD comments 
        ```bash
        rg -i '^\s*//[/!]+\s*#\s*Panics' | wc -l
        397
        ```

- Eliminates debug and runtime asserts in STD
    - Number of runtime assertions:
        ```bash
        rg -i --type rust '^(?!\s*//).*assert!' --pcre2 | wc -l
        9089
        ```
    - Number of debug assertions 
        ```bash 
        rg -i --type rust '^(?!\s*//).*debug_assert!' --pcre2 | wc -l
        678
        ```
- Removes redundant safety abstractions 
- Verus by default checks for possible arithmetic overflow/underflow. 

## Hints for verifying data structures
- Select an abstract model(`impl View`) of the target data structure. For example, `BinaryHeap<T>` can be represented as `Seq<T>` 
- Define a well-formedness specification(usually recursively) in terms of the abstract model. For example, in a `BinaryHeap`, both left and right children are less than or equal to their parent. For all `pub fn x(&mut self, ...)` API, the precondition and postcondition both include well-formedness.
    - There are usually more than one way to define well-formedness: bottom-up, top-down, etc. Need to find one that is easy for Verification.
- Reuse spec and proof code blocks as functions like regular programming.
- Write external specifications for functions, types and traits that are out of the scope of verification.
- Use assumptions to temporarily finish proofs.

## Verus features 
The `examples` directory contains small code snippets we write for testing Verus features.

### Verification Developer Experience
- Develop and debug proofs like programming
- Refactor specs and proofs like refactoring code 
- Incrementally prove stronger properties  
- Linear ghost state, ghost tokens  

## Trusted Computing Base for verifying STD 
- Rust Compiler 
- Verus itself including vstd
- The specifications 
- All assumptions in the proofs: `admit()`, `assume()`, `#[verifier::external_body]`, `assume_specification` etc
- An implicit assumption: the internal data structures of the std are only mutated by external code through the public API. It is difficult to enforce this in memory unsafe languages like C/C++ but easy in Rust: if all external Rust code is safe Rust.    


## Reference
1. [Verus Doc](https://verus-lang.github.io/verus/guide/)
1. [Vstd Doc](https://verus-lang.github.io/verus/verusdoc/vstd/)
1. [Verification Challenges of Rust std](https://model-checking.github.io/verify-rust-std)
1. [Kani overview](https://model-checking.github.io/kani-verifier-blog/2023/08/03/turbocharging-rust-code-verification.html)
1. [The Rust Security Advisory Database](https://rustsec.org/advisories/)
1. [Too many lists](https://rust-unofficial.github.io/too-many-lists/fifth-miri.html)
1. [Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html)
1. [Systems Verification](https://tchajed.github.io/sys-verif-fa24/)
