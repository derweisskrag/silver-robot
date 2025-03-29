# Count - React Reducer Pattern

## Objective

Implement Count Component and apply the logic to Kotlin, Rust, Python, C#.

> We focus on Count component, rather "Store" to track state within the app.

## Languages Overview

- Rust
- Python
- JavaScript
- C#
- C++
- Kotlin
- Java

## Python

First of all, we define the state, actions, and the dispatch function:

```python
state = {
    "count": 0
}
```

As prescribed in the task, we want to modify the state by using two functions (actions):

- Increment(value: int, payload: int)
- Decrement(value: payload: int)

As such, we can implement this functions:

```python
def increment(value, payload) -> None:
    value += payload
    
def decrement(value, payload) -> None:
    value -= payload
```

After, we can define the actions in the enum class:

> Please, notice that we normally use Enum

```python
class Actions:
    def __init__(self):
        self.inc = "INCREMENT"
        self.dec = "DECREMENT"
```

Then we can match our actions in the `dispatch` function:

```python

state = { "count": 0 }

def use_reducer(input_state, action, payload):
    match action:
        case "INCREMENT":
            return { "count": input_state["count"] + payload }
        case "DECREMENT":
            return { "count": input_state["count"] - payload }
        case _:
            return "Wrong action"


def dispatch(action, payload):
    global state  # React-like state update
    state = use_reducer(state, action, payload)
```

### Solution

```python
state = { "count": 0 }

def increment(value, payload) -> None:
    value += payload

def decrement(value, payload) -> None:
    value -= payload

def use_reducer(input_state, action, payload):
    match action:
        case "INCREMENT":
            return { "count": input_state["count"] + payload }
        case "DECREMENT":
            return { "count": input_state["count"] - payload }
        case _:
            return "Wrong action"


def dispatch(action, payload):
    global state  # React-like state update
    state = use_reducer(state, action, payload)

def main():
    # increase the state value
    dispatch("INCREMENT", 5)

    # decrease the state value
    dispatch("DECREMENT", 3)
```

## Kotlin

## My old React code

```tsx
import React, { useReducer } from 'react';

type State = {
  count: number;
};

type Action =
  | { type: 'increment' }
  | { type: 'decrement' }
  | { type: 'reset' };

const initialState: State = { count: 0 };

function reducer(state: State, action: Action) {
  switch (action.type) {
    case 'increment':
      return { count: state.count + 1 };
    case 'decrement':
      return { count: state.count - 1 };
    case 'reset':
      return { count: 0 };
    default:
      throw new Error();
  }
}

function Counter() {
  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <div>
      Count: {state.count}
      <button onClick={() => dispatch({ type: 'increment' })}>+</button>
      <button onClick={() => dispatch({ type: 'decrement' })}>-</button>
      <button onClick={() => dispatch({ type: 'reset' })}>Reset</button>
    </div>
  );
}

export default Counter;
```
