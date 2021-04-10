# `Rust` data structure implemenation

## How to generate the lib documentation

```bash
cargo doc --lib --document-private-items --release --open
```

</br>

## How to run the test

```bash
# If run all internal unit test which allows to test all private functions
cargo test --lib

# If run all internal unit test which allows to test all private functions
# If you want to see the `println` output
cargo test --lib -- --nocapture

# If run all internal unit test which allows to test all private functions
# Also, if you want to run all integrateion unit test in `tests` folder
cargo test

# If run all internal unit test which allows to test all private functions
# Also, if you want to run all integrateion unit test in `tests` folder
# If you want to see the `println` output
cargo test -- --nocapture
```

</br>

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


</br>


## which data structure I should use?

### `LinkedList` 

- Concept:

    A `LinkedList` is a sequential list of nodes that hold data which point to other nodes also containing data.

    **Head** --> Node ---> Node --> **Tail**

    - `Head`: the first node in the list.
    - `Tail`: the last node in the list.
    - `Pointer`: Reference point to another node.
    - `Node`: An object containing data and pointer(s).

    </br>

    - `SingleLinkedList`: Each node only hold the reference to the next node.
    - `DoubleLinkedList`: Each node holds the reference to the next node and the previous node at the same time.

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

- Single linked list:

    - [single_linked_list.rs](https://github.com/wisonye/data-structure-implementation-by-rust/tree/master/src/linked_list/single_linked_list.rs)

        Run that dev test with the following command:

        ```bash
        # No debug info output
        cargo watch --clear --exec 'test list_test'

        # With debug info output
        cargo watch --clear --exec 'test list_test -- --nocapture'
        ```

<hr>

### `Stack`

- Concept:

    A `Stack` is a one-ended linear data structure which models a real-world stack by having two primary operations:

    ```
    Push (on top)  Pop (from top)
       ↘            ↗
         ↘        ↗
       ------------- 
      | *********** |
       ------------- 
    ```


    - `Push`: add the element to the end of the stack.
    - `Pop`: remove the last element from the stack.

    </br>

    It's a `Last-in-first-out (LIFO)` model.

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
        cargo watch --clear --exec 'test stack_test'

        # With debug info output
        cargo watch --clear --exec 'test stack_test -- --nocapture'

        # No debug info output
        cargo watch -c --exec 'test token -- --nocapture'

        # Print all debug info
        cargo watch -c --exec 'test --features "enable_debug_code_token_pairing" token -- --nocapture'
        ```

<hr>

### `Queue`

- Concept:

    A `Queue` is a one-ended linear data structure which models a real-world queue by having two primary operations:

    ```
                                 Queue-Back
                                   ↓
               -   -   -   -   -   - 
    Dequeue ⇐ |*| |*| |*| |*| |*| |*| ⇐ Enqueue
               -   -   -   -   -   - 
               ↑
             Queue-Front
    ```


    - `Enqueue/Adding/Offering`: add the element to the back.
    - `Dequeue/Removing/Polling`: remove the element from the front.

    </br>

    It's a `First-in-first-out (FIFO)` model.

- use cases:

    - Any waiting line modes a queue. For example a lineup in super market counter.
    - Web server request management (first come first serve).
    - Do a `Breadth First Search (BFS)` on a graph

- Complexity

    | Complexity | Notation
    |----------: | --------
    | Enqueue | **O(1)**
    | Dequeue | **O(1)**
    | Peeking | **O(1)**
    | Contains | **O(n)**
    | Removal | **O(n)**
    | Size | **O(1)**
    | Is Empty | **O(1)**

<hr>

### `Priority Queue (PQ)`

- Concept:

    A `Priority Queue` is an **ADT** that works like a normal `Queue` except that `each element has a certain priority`.
    And that priority value determines the removing order from the queue.

    The element has to implement the `PartialEq` trait for supporting the priority comparison.

