# rust-patterns

This is a repo where I try different rust patterns.


## State

Pros:
- State changes/logic remain associated with state trait which can have default implementation to reduce code 
- Easy to extend with new states.

Cons: 
- Code repetition as we try to perform the same operation while chanding from one state to another (which can be reduced using macros)
- State coupling, when we try to add a state between two adjacent states.
