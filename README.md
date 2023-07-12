# `caet`: Cause and effect tester

## What is it?

`caet` simulates a universe, operated by a judge, in which you can
test an object's behavior in response to a stream of events, while
that object, in turn, influences the universe.

## Why would I want to do that?

`caet` is primarily intended to prototype and validate object-oriented
analysis and design models because of the deficiencies in earlier
approaches:

- Any sort of theoretical argumentation: quick and easy (compared to
actually writing code), but may be inaccurate or incomplete. Surprises.
- Writing code: slow, expensive, lock-in.
- Writing unit tests: rigid, brittle, lock-in.
- Writing integration tests: not generally applicable;
scaffolding effort.
- Many elaborate approaches: at some point, you get the impression
that all they care about is selling you their book.

And, when is simulation a better approach?

- When you want to test a design model, not an implementation.
- You have a proposal to break down a complex system to manageable parts.
Alright, but are you sure that it's *good* design? How do you know?
- Well, simulate it and see what happens.

How would simulation be better?

- Not too expensive: you do have to write code, but you don't have
to deal with actual I/O or performance. Just define the data types
that represent events and reactions, and add some mock code for
the object under test (and the judge, as well; more on that later).
Once you're done, you can review it, discuss it, throw it away, or keep it.
- Not locked-in: you don't to connect to existing code at all.
If you know your domain, you can write the simulation code
from scratch, and then throw it away or keep it as documentation.
So, even though this crate uses Rust, and you must use Rust
to write the simulation code, you don't have to use Rust for
your actual implementation. I figured you'd appreciate this if you're
already working on a big project.
- Not rigid: most testing suites simply recall a sequence of
events and check the results. `caet` allows you to define
a custom `Judge` machine that just traces the events your 
system-under-test emits and, in response,
manipulates future states of its environment so as to poke the
object under test in interesting ways---or just recalling the
events and checking the results, if that's what you want.
You have to write the `Judge` yourself, but it's not hard.
- Not brittle: it's not about recalling a sequence of events,
it's checking it using an actual computer program. So, it's
just about as brittle as you make it. If you spend the effort,
you'll be rewarded with a robust test suite. If you don't,
you'll get a fantastic demonstration in just a few dozen minutes
that you can discuss later.
- Not a book: I'm not selling any philosophy here. I'm just
giving you a tool that you can use to test your ideas.
Ok, there is some philosophy, but it's mostly a paraphrase
of object-orientation, nothing proprietary.

## Explain to me the philosophy behind it.

`caet` is based on the following principles:
- **Protocol not algorithm**: We're solving an interactive problem, not a computational problem.
In other words, we're building bots, not cracking LeetCode.
- **Breaking down interactions**: We look at any interactive problem in three ways:
    - **Object**: this is the system that's being modeled.
    All objects are specialized for each of their universes.
    An object can also be called a thing, entity, agent, whatever.
    There's many names for it, but it's all the same thing.
    - **Universe**: this term is a bit ambiguous, but it's essentially
    a *view* of the outside world stated in a language so simple that the
    object can understand. You can, if you want, incorporate a notion
    of some absolute laws this universe obeys during its evolution,
    but `caet` assumes nothing by itself. It's also called the environment,
    surroundings, context, whatever. Again, many names, same thing.
    - **Time**: again, ambiguous concept and there's a lot you
    can add to this concept for your own gain, but for our purposes, it's just a
    definition of the order in which things happen. Maybe you
    might want to incorporate physical timing, simultaneity, etc.
    But `caet` assumes nothing except "X happened before Y."
- **Singleton principle**: A universe contains exactly one object. So if you wanted to model
communication, you'd have to do it roundaboutedly: first convert
their collective actions to changes in the universe, and then
let each object react to those changes. But if you really want to
model concurrency and communication, `caet` is not the tool for you,
though it can be used to prototype such a system, if you hack it.
- **Object-orientation**: An object may be composed internally of distinct,
simpler, shorter-lived universes, akin to how the human body is composed
of multiple distinct cavities, each isolated from the outside,
where different organs are specialized to rest in.
And, the collective evolution of these universes determines the larger compound
object's behavior. This breakdown can be done recursively until the programmer
decides that a prepared dependency or module can handle it,
or the universe no longer requires interaction, so it can be
replaced with an *infallible* function call.
- **Instantaneity of observation**: The object can "sense" or "observe" things from the universe, and this is cheap.
In fact, the object can sense anything in the universe reliably and instantaneously,
but only passively; it can't actively choose what to sense and when
because that amounts to violating causality.
- **Reaction**: The object can "act" on the universe, which is neither instantaneous nor reliable.
An object's action caused by an event is called a reaction.
- **Causality**: The future can't affect the past. *Causality*, as `caet` understands it, is exactly the same as any of the following (and mutually so):
  - **Encapsulation**: The universe can hide information from the object,
  and the object can hide information from the universe.
  - **Inherent interactivity**: Some processes may, inherently, be interactive;
  they could also be perhaps, but not necessarily, adversarial.
  - **State**: Both the universe and the object have *state*, which is not
  only about keeping information, but also, about time sensitivity.

The last bit can be a bit surprising. But, yes, all four concepts are
the same thing, one single concept.

## How do I use it?

First, this is more like a prototyping tool than an actual testing tool.
Or, this is a tool to *test* (validate) your ideas before you actually
write code. So, keep that in mind.

Also, this is a Rust crate, so you need to know Rust to use it.

Now, you need to perform some conceptual work before using this tool.

1. You need to define the language of universe. (Let's call it `MyEvent` for now.)
That sounds daunting, but remember, `caet` actually has a pretty
narrow and particular definition of a universe. So, specifically, you need to define:
  - A data type that represents an event. This is the *language* of the universe.
  (Or, pedantically, the "alphabet" part of the language.)
  Note that `caet` conflates the concepts of events, messages, and commands,
  and doesn't distinguish between them. What this means in practice is that
  the return values of the object's reactions are described using the same
  data type as the method calls.
2. Now, you need to implement the `Judge` trait for yourself.
This trait takes in the set of events that the object has last emitted,
judges these events for acceptance, maintains and evolves the internal universe,
and then---if it decides to keep going---
generates a new event (a *challenge*) for the object. Or, halts the simulation
and reports its findings.
3. There's no dedicated "Object" trait, but in the `judge` function,
which is responsible for running the simulation, you can pass in
a closure of this type: `FnMut(MyEvent) -> Vec<MyEvent>`.
The object will be reacting to a single event at a time,
generating an ordered list of events in response.
What happens when any of these events interleave with the outside
world is up to the judge.
Perhaps surprisingly, this is all you need to do to define an object.
4. Run the test using the `judge` function.
This function takes in your Judge implementation,
and the closure that represents the object.
It will run the simulation until the judge decides to halt it,
after which an `Outcome` structure is returned.
You can inspect the `Outcome` to see whether the simulation was
successful or not and how many iterations it took, and if not, what went wrong.

On why I decided to use a closure for the object:

I designed it that way to hide any
state that the object might have because, maybe, the object you're testing
is actually a real-world system, so maybe it isn't a computer program
at all. So, it's not reasonable to expect the object to be a structure
that you can just instantiate and pass around. It might be a physical
system that you can't just copy and paste, after all.

So, let's summarize this:
1. Define all events in a data type. (Let's call it `MyEvent`.)
Commands, method calls, signalling, return values, whatever.
Cram them all into this data type.
2. Implement the `Judge` trait for yourself.
The judge is like a god:
It runs an adversarial universe that tries to break your object
by strategically generating events that the object will have to react to.
(Or, it can be an indifferent universe that just recalls a pre-recorded
sequence of events. You decide.)
3. Define the object as a closure of type `FnMut(MyEvent) -> Vec<MyEvent>`.
The return value is the set of events---if any---emitted by the object
listed in the order they were emitted.
The goal of the object is to emit the events, in the correct order,
that the judge accepts.
4. Run the simulation using the `judge` function by passing
ownership of the judge and the object closure.
This function will let them fight and return an `Outcome` structure
that you can inspect.

One thing you should note is that the `Judge` trait is simply unaware of object types, and objects also don't know about the Judge. The only thing that connects the two is the `judge` function.

That means, any judge can test any object if they agree to use the same event type,
`MyEvent` in this case. Hopefully, this is a powerful feature because it means
you can test your object against multiple judges, and you can also test
multiple objects against the same judge.

## Where to find examples

Perhaps looking at examples will make this clearer.

As for examples, there's the main page of the documentation, and there's also
the unit tests for the crate itself.

The documentation on the main page is simple but doesn't cover all the features.
The unit tests are more comprehensive, but they're also more complicated.
I recommend you start with the documentation and then move on to the unit tests.
