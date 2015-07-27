# stator
Clearly manage state transitions using closures in Rust.

### Create your state definition struct

```
struct States;
impl States
{
    fn a() -> &'static str { "a" }
    fn b() -> &'static str { "b" }
    fn c() -> &'static str { "c" }
}
```

### Create a new Stator object, specifying initial state

```
let mut state = Stator::new(States::a());
```

### Add some state handlers

1. Specify `Stator::any_state()` as the first parameter and a custom state as the second to execute on state enter
2. Specify a custom state as the first parameter and `Stator::any_state()` as the second to execute on state exit
3. Specify a custom state as both parameters to execute on transition between the two states
4. Specify `Stator::any_state()` as both parameters to execute on every state change

```
state.add_handler(States::a(), States::b(), |from: &String|
{
    println!("a -> b");
});

state.add_handler(Stator::any_state(), States::b(), |from: &String|
{
    println!("? -> b");
});

state.add_handler(States::a(), Stator::any_state(), |from: &String|
{
    println!("a -> ?");
});

state.add_handler(Stator::any_state(), States::a(), |from: &String|
{
    println!("? -> a");
});
```

### Enter a new state at any time

*Protip: Don't pass `Stator::any_state()` to the `enter()` method.*

```
state.enter(States::b());
```

### Enjoy the irony

*Stator: The stator is the stationary part of a rotary system, found in electric generators, electric motors, sirens, or biological rotors.*