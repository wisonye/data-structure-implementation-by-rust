# `Rust` data structure implemenation

## What is `Abstract Data Type (ADT)`

An `Abstract Data Type (ADT)` is an abstraction of a data structure which provides the only interface to which data structure must adhere to.

The interface doesn't give any specific details about how something should be implemented or in what programming language.

For example:

| **Abstraction** (ADT) | **Implementation** (DS)
|-------------------|--------------------
| List | Dynamic list </br> LinkedList
| Queue | LinkedList based Queue </br> Array based Queue </br> Stack based Queue
| Map | TreeMap</br> HashMap


## Complexity Analysis

- How much `time` does this algorithm need to finish?
- How much `spcace` does this algorithm need for the computation?

That's what `Big-O` notation for:

`n` - The size of the input.
Complexity ordered in form smallest to largest:

| Complexity | Notation
|-------------: | --------------
| Constant Time| **O(1)**
| Logarithmic Time| **O(log(n))**
| Linear Time| **O(n)**
| Linearithmic Time| **O(nlog(n))**
| Quadric Time| **O(n²)**
| Cubic Time| **O(n³)**
| Exponential Time| **O(bⁿ) b > 1**
| Factorial Time| **O(n!)**

</br>

## which data structure I should use?

### `LinkedList` 

- use cases:

    - Used in many `List`, `Queue` and `Stack` implementation.
    - Great for creating circular lists.
    - Used in separated chaining, which is present certain `hasttable` implementations to deal with hashing collisions.
    - Often used in implementation of adjacency list for graphs.

- Props and cons:

    | |Props | Cons
    |-----: | ---- | -------
    | **SingleLinkedList** | _Use less memory_</br>_Simpler implementation_ | _Cannot easily access previous element_
    | **DoubleLinkedList** |_Can easily access backwards_ | _Takes 2X memory_


- Complexity

    | |SingleLinkedList | DoubleLinkedList
    |-----: | ---- | -------
    | **Search** | **O(n)** | **O(n)**
    | **Insert at head** |**O(1)** | **O(1)**
    | **Insert at tail** |**O(1)** | **O(1)**
    | **Remove at head** |**O(1)** | **O(1)**
    | **Remove at tail** |**O(n)** | **O(1)**
    | **Remove in middle** |**O(n)** | **O(n)**

### `Stack` 

- use cases:

    - Undo mechanism
    - Compiler syntax checking and parsing for the matching symbols
    - Model a pile of books and plates
    - Used behind the scenes to support recursion by keeping track of previous function call
    - Do a `Depth First Search (DFS)` on a graph

- Complexity

    | Complexity | Notation
    |----------: | --------
    | Pushing | **O(1)**
    | Poping | **O(1)**
    | Peeking | **O(1)**
    | Searching | **O(n)**
    | Size | **O(1)**


- Sample:

    - [source_code_token_pairing_validator.rs](https://github.com/wisonye/data-structure-implementation-by-rust/tree/master/src/source_code_token_pairing_validator.rs)

        Run that test with the following command:

        ```bash
        # No debug info output
        cargo watch -c --exec 'test token -- --nocapture'

        # Print all debug info
        cargo watch -c --exec 'test --features "enable_debug_code_token_pairing" token -- --nocapture'
        ```
