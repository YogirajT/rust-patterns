# rust-patterns


This is a repo to try different rust patterns.


## State


Pros
- State changes/logic remain associated with state trait which can have default implementation to reduce code .
- Easy to extend with new states.


Cons
- Code repetition as we try to perform the same operation while chanding from one state to another (which can be reduced using macros).
- State coupling, when we try to add a state between two adjacent states.
- Invalid state transitions or access are not caught at compile time.


Fixes
- Having different structs and implementations for each state of License process in this example so that we get compile time error trying to set and get invalid states.


## State Encoding into type system

Pros
- The compile time type check prevents unwanted methods from being exposed and throwing errors unlike state pattern.
- More decoupled than State patten as less code needs to be changed if an intermediary need to be introduced.

Cons
- Code repetition as we try to perform the same operation while chanding from one state to another (which can be reduced using macros).
