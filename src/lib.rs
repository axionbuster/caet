//! A judge for cause-effect systems.
//!
//! A cause-effect system is a system that takes in a cause (input) and produces an effect (output).
//! The judge is a test driver for the system.
//!
//! According to this crate's worldview, an interactive system is composed of three parts:
//! 1. A **universe**: set of all sensory experiences (observations) that the system can perceive,
//! the set of all reactions that the system can produce, and any constraints on the evolution of
//! the system.
//! 2. An **object** (entity, subject, thing, etc.): the only designated "thing" in the universe,
//! and that which is the system under test.
//! 3. **Time**: Encompasses the notion of causality, and perhaps more. Only a broad and
//! ambiguous notion of causality is used here.
//!
//! As time progresses, the object perceives the universe and reacts to it, which
//! then destructively changes the universe. The object will then perceive the new universe,
//! and react to it, and so on.
//!
//! ## What's important to know about the *universe*?
//!
//! In this worldview, there's some peculiar refinements to the notion of a *universe*.
//! Let me list some of them here:
//! 1. Exactly one being exists in the universe: the object. Hence, the object is
//! not aware of its relationships with any other objects, because they're in
//! parallel universes.
//! 2. The universe is defined by the object's perceptions of it. Only the immediate
//! surroundings, and only those as understood by the object, are relevant.
//! However, the judge can manipulate the universe to test the object's reactions.
//!
//! ## Recursive division of a universe
//!
//! If a programmer decides the language of the universe is too complicated, or
//! if there is incomplete information (synonymous with the requirement for interaction)
//! that must be factored in the problem statement, then the **object** (not the universe)
//! can be divided into multiple sub-universes, and each sub-universe will contain
//! exactly one object. This is done until either:
//! - The problem can be outsourced to a dependency, or
//! - Both: the language is simple enough AND there is complete information (no interaction)
//! to perfectly predict or produce the object's reactions.
//! In the latter case, the object is no longer a cause-effect system and more
//! like a computational job like a function call.
//!
//! The image of dividing the "guts" of an object into multiple types of universes resembles
//! the way the human body is organized.
//!
//! See how the brain is in its own space that's separated from the blood flow
//! (through a blood-brain barrier), and the lungs exist in their own peculiar environment
//! that allows the presence of air, and the stomach is in its own environment that
//! is acidic, and so on. And yet all of them are also different from the outside
//! world.
//!
//! The brain will simply melt away in the stomach, and the lungs will simply
//! collapse in the cranial cavity. They are smaller objects that are specialized to
//! their own environments and that understand only changes in their own environments.
//!
//! The human body somehow maintains distinct environments that are specialized for
//! and mutually incompatible with every organ. When the human senses something,
//! and decides to engage in an action, the human body changes these respective
//! environments and lets the organs naturally react to the changes. The human
//! body then compiles those reactions into a single reaction that is then
//! sent to the outside world.
//!
//! ## Is this some peculiar philosophy?
//!
//! No, it's called **object-orientation** (OO).
//!
//! It's about using divide and conquer for interactive systems, and not
//! just algorithms.
//!
//! This crate, which tests any general cause-effect system, is originally
//! designed to test and prototype OO systems, even before
//! anything has been written in a programming language.
//!
//! OO design is important, but getting it right from the start is hard.
//! The scale of systems in which OO design is important also tends to be
//! the larger ones, where the cost of getting it wrong is also high.
//!
//! **That's why this crate exists: It allows you to validate your breakdown.**
//! It allows you to validate each subsystem in your OO design
//! so you can make an accurate judgment about whether your breakdown
//! is actually helping the problem or rather making it worse
//! without connecting it to a real codebase or a real system, and all the while
//! avoiding the rigidity of unit tests.
//!
//! This simulation idea is also useful for **simply getting started in
//! an unfamiliar domain**. It's a way to get your feet wet without
//! committing to a particular design.
//!
//! ## When to use this crate to test your OO system or prototype
//!
//! Remember the divide and conquer idea?
//!
//! The object is subdivided into simpler universes that span shorter time periods.
//! This division is done until the universe is both simple to describe
//! and requires no interaction (due to being so short-lived).
//!
//! In your OO design, it's important to prototype different subuniverses
//! and subobjects, and to test them in isolation.
//!
//! It's often quickest to test the idea on paper first, but your cost-benefit
//! analysis may be inaccurate. Diving straight into code is, of course,
//! the most expensive option, while being the most accurate.
//!
//! This crate is the middle ground between the two extremes. It allows you
//! to test out your OO analysis **without writing too much code.**
//!
//! It's just a simulation, anyway. Don't bother with async, threads, or
//! any other fancy stuff. Using this crate requires that you focus on
//! expressing the most important ideas into code. Namely,
//! - How you choose to represent sensory experiences (observations) and reactions
//! as data types, and
//! - The way your object reacts to the simulated universe.
//!
//! ## How to use this crate to test your OO system or prototype
//!
//! This crate has exactly three public types:
//! - trait [`Judge`]: A god-like entity that controls the universe and judges the object's reactions.
//! - struct [`Outcome`]: The final result of a test.
//! - struct [`Judgment`]: A judgment about an object's reaction evaluated at a particular time.
//! Also contains the next input to the object.
//!
//! Needless to say, but, the structs have a fixed implementation.
//!
//! Your job is to implement the trait [`Judge`] and its three associated types.
//!
//! And, two public functions (essentially one, though):
//! - [`judge`]: Run a test.
//! - [`judge_panic`]: Run a test, but panic and report if the object's reaction is unacceptable,
//! or if the judge hits some sort of an error of its own.
//! So, essentially, a wrapper around [`judge`].
//!
//! The [`judge`] function call is your entry point into this crate.
//! More on that later.
//!
//! ### [`Judge`]
//!
//! Start by implementing the trait [`Judge`].
//!
//! There are three associated types you must implement:
//! - **`Change`**: A combined observation and reaction type.
//! - **`Fault`**: In case a reaction is unacceptable, this type is used to report the reason.
//! - `Error`: A judge can also hit an error, in which case this type is used to report the reason.
//!
//! The first two are the most important. Let's see why:
//! - **`Change`**: You must abstract away the object's sensory experiences and reactions
//! by stripping out the details that are irrelevant to the problem at hand.
//! - **`Fault`**: You must explain what it means for a reaction to be unacceptable.
//!
//! The third type (`Error`) can be whatever. It's a simulation error, an implementation detail,
//! and not relevant to the OO design.
//!
//! Lastly, you must implement the trait method [`Judge::next`].
//!
//! This method is called by the [`judge`] function to get the next input to the object
//! based on an ordered list of reactions that the object has produced so far.
//!
//! Check out the module `test_stack` in the source code for an example.
//!
//! In said example, a `StackJudge` type is implemented. It has a constructor
//! that accepts the variations on the test case. Each instance of `StackJudge`
//! will simulate a different universe, which are, however, all described by the same laws.
//!
//! ### Your object
//!
//! The definition of an object is any thing that reacts to changes in its environment.
//!
//! Because the "thing" concept is so general, it would be unreasonable to require
//! you to define it as some sort of a concrete data type. Instead, in this crate,
//! your object will be represented by a closure, called its **reaction function**.
//!
//! In the signature of [`judge`], you must provide a closure that implements
//! `FnMut(J::Change) -> Vec<J::Change>` where `J` is your implementation of [`Judge`].
//! This closure is your object's reaction function, and, therefore, its representation.
//!
//! Let's break it down. It's a lasting closure with its own private state
//! (hence the `FnMut` trait bound) that reacts to a change in the universe
//! and produces its own changes in the universe---maybe silence, maybe a single change,
//! or perhaps multiple changes (ordered from index 0 at the earliest to the latest).
//!
//! So the "object" is kind of abstract, hidden behind the `FnMut` bound,
//! and the concept of either "method call" ("observation") and "return value" ("reaction")
//! are also abstract, hidden behind your provided `Change` type together.
//! So, the `Change` type is kind of a conflation of the two concepts.
//!
//! In the example given as `test_stack`, the universe gives the object (a stack data structure)
//! a command to either push or pop a value. The object is then supposed to
//! produce a "value" reaction, which provides what was at the top of the stack
//! for each pop.
//!
//! If the object produces a "push"
//! or "pop" reaction back at the world, which is clearly absurd and confused,
//! the judge will report a fault. The object, likewise, will panic if the
//! universe gives a "value" (stack's value) reaction, which is only meant
//! to be produced by the object itself, because that's absurd. So that's
//! the way the example deals with the conflated "observation" and "method call"
//! concepts.
//!
//! (There's a minor difference between them, that is, a judge will merely
//! report a fault while the object will panic. This is because the judge
//! is meant to be a god-like entity that can't be *wrong*---though it can
//! certainly be *adversarial*---while the object
//! is a mere mortal that can be wrong. But this is a minor detail.)
//!
//! So, just implement a dynamic dispatch kind of thing that decodes
//! the `Change` type (think of it as a message or signal) and produces
//! some answer (also the same `Change`) type.
//!
//! ### [`judge`]
//!
//! The function [`judge`] takes in exactly two arguments:
//! - **`judge`**: Your implementation of [`Judge`].
//! - **`object`**: Your object. A function closure as described above.
//!
//! It returns an [`Outcome`] type if the judge successfully completes, or [`Judge::Error`],
//! an Judge-associated type, if the judge fails to simulate the universe.
//!
//! Once an [`Outcome`] is produced, you can inspect it to see
//! whether the object has passed or failed the test, and
//! the number of observations (`calls` field) made by the object.
//!
//! Don't forget the check to see if the object passed too early
//! test because, while the object's reactions were acceptable,
//! they went beyond what the judge was expecting, which caused
//! it to prematurely halt the test. That's why the [`Outcome`]
//! type has a iteration count field. Check that field to see
//! if it's way too low.
//!
//! ## Example
//!
//! The source code provides a more detailed example in the module `test_stack`.
//!
//! Here, let's do a much simpler, but also less complete, example.
//!
//! We model an agent that plays the guessing game.
//!
//! The range of possible numbers is -1000 to 1000, and the agent
//! must guess the correct number within 10 guesses.
//!
//! ```
//! use caet::{judge, Judge, Judgment, Outcome};
//! use std::convert::Infallible;
//!
//! /// Universe
//! #[derive(Debug, PartialEq)]
//! enum GuessIs {
//!     Start, // The initial call to `next` is always with an empty vector
//!     TooLow, // Judge: The agent's guess was too low
//!     TooHigh, // Judge: The agent's guess was too high
//!     Value(i32), // Agent: Here's my guess
//! }
//! use GuessIs::*;
//!
//! /// Judge
//! struct MyJudge {
//!     count: u32,
//!     target: i32,
//!     begun: bool,
//! }
//! impl Judge for MyJudge {
//!     type Change = GuessIs; // All changes in the universe
//!     type Fault = String; // What it means for a reaction to be unacceptable
//!     type Error = Infallible; // What it means for the judge to fail
//!     /// Get object's reactions, judge them,
//!     /// and, if acceptable, return the next input or stop.
//!     fn next(&mut self, reactions: Vec<GuessIs>) -> Result<Judgment<GuessIs, String>, Infallible> {
//!         // The initial call to `next` is always with an empty vector, given by the `judge`
//!         // procedure itself, so prepare for that.
//!         if !self.begun {
//!             self.begun = true;
//!             return Ok(Judgment::Continue(Start));
//!         }
//!
//!         // Now, all subsequent calls to `next` has a vector that was created by the object.
//!
//!         // Ignore earlier reactions, if many
//!         println!("reactions: {:?}", reactions);
//!         if let Some(reaction) = reactions.last() {
//!             self.count += 1;
//!             if self.count > 10 {
//!                 // Unacceptable: Too many guesses
//!                 return Ok(Judgment::Halt(format!("It was {}.", self.target)));
//!             }
//!             match reaction {
//!                 // Acceptable: Legitimate actions by the agent
//!                 Value(n) if *n == self.target => return Ok(Judgment::Done),
//!                 Value(n) if *n < self.target => return Ok(Judgment::Continue(TooLow)),
//!                 Value(n) if *n > self.target => return Ok(Judgment::Continue(TooHigh)),
//!                 // Unacceptable: Faulty implementation of the agent
//!                 _ => return Ok(Judgment::Halt(
//!                     format!("Invalid reaction type for agent: {:?}",
//!                     reaction
//!                 ))),
//!             }
//!         }
//!         Ok(Judgment::Halt(format!("You can't just pass a turn.")))
//!     }
//! }
//!
//! // My object (bisect)
//! let mut lower_bound = -1000;
//! let mut upper_bound = 1000;
//! let mut guess = 0;
//! let mut object = |observation| {
//!     println!("I sensed: {observation:?}");
//!     match observation {
//!         Start => {
//!             guess = (lower_bound + upper_bound) / 2;
//!             vec![Value(guess)]
//!         }
//!         TooLow => {
//!             lower_bound = guess;
//!             guess = (lower_bound + upper_bound) / 2;
//!             vec![Value(guess)]
//!         }
//!         TooHigh => {
//!             upper_bound = guess;
//!             guess = (lower_bound + upper_bound) / 2;
//!             vec![Value(guess)]
//!         }
//!         Value(_) => panic!("You're not supposed to guess!"),
//!     }
//! };
//!
//! // Test
//! let outcome: Outcome<MyJudge> = judge(
//!     MyJudge { count: 0, target: 42, begun: false },
//!     &mut object
//! ).unwrap();
//! assert_eq!(outcome.judgment, Judgment::Done);
//! println!("It took {} guesses.", outcome.calls);
//! ```

/// A judgment of a cause-effect system.
///
/// - Did the subject produce an acceptable reaction?
/// - And, if so, should the judge continue or halt the program?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Judgment<M, S> {
    /// Acceptable; continue with this input.
    Continue(M),
    /// Unacceptable (with a reason); terminate the program.
    Halt(S),
    /// Acceptable; finished testing.
    Done,
}

/// A judge for a cause-effect system.
///
/// - maintain a virtual "universe" (surroundings) in a language understood by the subject,
/// - define said universe (see `Observation` assoc. type), and
/// - judge the subject's reactions to the universe (see [`Judgment`]; `Fault` assoc. type), while
/// - controlling what the subject senses (observes) about that universe.
///
/// (It can sound peculiar that the *judge is in control of the subject's senses*,
/// but this is done to allow simplifying the language of
/// the universe by essentially boiling it down to what the subject can understand.
/// Think of the famous "brain in a vat" thought experiment.)
///
/// See also: [`judge`].
pub trait Judge {
    /// Explain why the subject is at fault.
    type Fault;
    /// Express an observation about the universe (surroundings) made by the subject
    /// or a reaction produced by the subject.
    type Change;
    /// Any internal error type.
    type Error;
    /// Given the ordered (earliest to latest in `0..n`) reactions produced by the subject
    /// thus far since the last call...
    /// - Decide whether the response is acceptable.
    /// - Decide whether to continue or halt the program.
    /// - If continuing, produce the next observation to send to the subject.
    /// Once the judge decides to halt the program, it should return [`Judgment::Done`]
    /// on success; otherwise, it should return [`Judgment::Halt`] on failure.
    ///
    /// (A faulty judge may also return [`Judgment::Continue`]. This is considered a judge fault.)
    ///
    /// ## Starting with silence
    ///
    /// When the [`judge`] routine starts, it calls this method with an empty vector.
    ///
    /// See: [`judge`], [`Judgment`], [`Outcome`].
    fn next(
        &mut self,
        reactions: Vec<Self::Change>,
    ) -> Result<Judgment<Self::Change, Self::Fault>, Self::Error>;
}

/// The final judgment of a cause-effect system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Outcome<J: Judge> {
    /// The final judgment.
    ///
    /// See also: [`judge`].
    ///
    /// - The [`Done`](Judgment::Done) judgment is the only one that
    /// should be returned in a functioning system.
    /// - [`Continue`](Judgment::Continue) and [`Halt`](Judgment::Halt) judgments
    /// indicate errors in the judge and the subject, respectively.
    pub judgment: Judgment<J::Change, J::Fault>,
    /// Number of times the judge has called the task.
    pub calls: usize,
}

/// I got too lazy to convert the old code that didn't have the [`Outcome`] type
/// and simply used a tuple instead (which got a Clippy warning for type complexity).
trait PrivateOutcomeDecompose<J: Judge> {
    /// Decompose the outcome into its components.
    fn decompose(self) -> (Judgment<J::Change, J::Fault>, usize);
}
impl<J: Judge> PrivateOutcomeDecompose<J> for Outcome<J> {
    fn decompose(self) -> (Judgment<J::Change, J::Fault>, usize) {
        (self.judgment, self.calls)
    }
}

/// A test driver for a cause-effect system.
///
/// Test drive the task with the judge, and return the number
/// of times the judge has called the task.
///
/// See the `test_stack` module in the source code for an example.
///
/// See also: [`judge_panic`].
pub fn judge<J>(
    mut judge: J,
    mut object: impl FnMut(J::Change) -> Vec<J::Change>,
) -> Result<Outcome<J>, J::Error>
where
    J: Judge,
{
    use std::mem;
    let mut out = vec![];
    let mut ite = 0;
    loop {
        let jud = judge.next(mem::take(&mut out));
        match jud {
            Ok(Judgment::Continue(msg)) => {
                out = object(msg);
                ite += 1;
            }
            Ok(j) => {
                return Ok(Outcome {
                    judgment: j,
                    calls: ite,
                })
            }
            Err(e) => return Err(e),
        }
    }
}

/// Like [`judge`], but panic on any error, either due to the judge
/// or the task.
pub fn judge_panic<J>(j: J, object: impl FnMut(J::Change) -> Vec<J::Change>) -> usize
where
    J: Judge,
    J::Error: core::fmt::Display,
    J::Fault: core::fmt::Display,
{
    match judge(j, object).map(|o| o.decompose()) {
        Ok((Judgment::Done, count)) => count,
        Ok((Judgment::Halt(why), count)) => {
            panic!("subject fault (iter count: {count}): {why}")
        }
        Ok((Judgment::Continue(_), count)) => {
            panic!("judge fault (iter count: {count}): judge stopped at continue")
        }
        Err(e) => panic!("judge fail (internal error): {e}"),
    }
}

#[cfg(test)]
mod test_stack {
    //! Test an example universe (understood by a LIFO data structure) and
    //! different test subject implementations.

    use std::collections::VecDeque;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StackChange {
        Push(i32),
        Pop,
        Value(Option<i32>),
    }
    use StackChange::*;

    /// An interactive judge for a stack with buffering.
    ///
    /// Note: This judge allows the subject to hold its reactions
    /// and release it all at once when it's ready.
    ///
    /// However, there's no way the judge can force the subject to
    /// flush its reactions, so a subject that discards
    /// all input will pass this judge.
    ///
    /// This is done to test the judge's ability to handle
    /// a subject that doesn't react to every observation
    /// immediately, without adding harnesses to handle buffering.
    ///
    /// At least, it can properly judge implementations that do
    /// produce some reactions.
    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    struct StackJudge {
        /// List of pushes and pops to simulate.
        scenario: VecDeque<StackChange>,
        /// Reference implementation of the stack.
        ref_impl: Vec<i32>,
        /// The reactions that must be produced by the subject
        /// arranged in order.
        expect: VecDeque<i32>,
    }
    impl Judge for StackJudge {
        type Error = String;
        type Fault = String;
        type Change = StackChange;

        fn next(
            &mut self,
            reactions: Vec<Self::Change>,
        ) -> Result<Judgment<Self::Change, Self::Fault>, Self::Error> {
            if self.expect.len() < reactions.len() {
                // Defeat subjects that pop too many times, or pops when it says push.
                return Ok(Judgment::Halt("too many reactions".to_string()));
            }
            // Compare top to bottom and terminate on the first difference.
            let mut put_back = None;
            for (eff, exp) in reactions.iter().zip(self.expect.drain(0..reactions.len())) {
                // eff -> test subject
                // exp -> reference implementation
                match eff {
                    StackChange::Value(Some(value)) if *value != exp => {
                        // Defeat subjects that pop the wrong value.
                        return Ok(Judgment::Halt(format!(
                            "expected {:?}, got {:?}",
                            exp, value
                        )));
                    }
                    StackChange::Value(None) => {
                        // A subject stays silent. It may be buffering the value, or it may
                        // have just forgotten it. Buffering is allowed, so let's put it back.
                        put_back = Some(exp);
                        break;
                    }
                    // OK: Some(value) where value == exp
                    StackChange::Value(_) => (),
                    // Subject is only allowed to produce Value, and not Push or Pop,
                    // which are reserved for the judge.
                    _ => return Ok(Judgment::Halt("undefined response from stack".to_string())),
                }
            }
            if let Some(exp) = put_back {
                // See above comment on buffering.
                self.expect.push_front(exp);
            }
            // Produce the next action.
            if let Some(act) = self.scenario.pop_front() {
                // Simulate the action on the reference implementation.
                match act {
                    StackChange::Push(push) => self.ref_impl.push(push),
                    StackChange::Pop => {
                        if let Some(pop) = self.ref_impl.pop() {
                            self.expect.push_back(pop);
                        } else {
                            return Err("bad sim: more pops than pushes".to_string());
                        }
                    }
                    StackChange::Value(_) => return Err("bad sim: Value in scenario".to_string()),
                }
                // Let it go, here, and wait for the subject to produce the same reaction
                // (*eventually*---the subject is allowed to delay reporting any pops).
                return Ok(Judgment::Continue(act));
            } else {
                // The scenario is exhausted.

                // In this judge impl, the reference implementation may have extra
                // items. So, a subject that always stays silent is actually going to pass.

                // This is intentional: I wanted to test a judge that is capable of
                // bunching up the reactions and then comparing them all at once.
                // Without having a sentinel "flush" message, the judge would have
                // no way of knowing when to stop waiting for more reactions.

                // "Done" means the subject is correct and the judge is satisfied.
                // It could mean either:
                // - The test has been passed, or
                // - The subject produced a response that's correct but not in the scenario,
                // so the scenario is no longer unable to cover that case.
                Ok(Judgment::Done)
            }
        }
    }
    impl StackJudge {
        /// Rust custom indicates that a `new` method should be provided
        /// whenever a `Default` implementation is provided.
        #[allow(dead_code)]
        fn new() -> Self {
            Self::default()
        }
        /// Real constructor right here.
        fn new_scenario(scenario: Vec<StackChange>) -> Self {
            Self {
                scenario: scenario.into(),
                ..Default::default()
            }
        }
    }

    /// A valid push-and-pop scenario.
    fn scenario_1() -> StackJudge {
        #[rustfmt::skip]
        let sce = vec![
            Push(1), Push(2), Push(3),
            Pop, Pop,
            Push(4),
            Pop
        ];
        StackJudge::new_scenario(sce)
    }

    fn demo_impl_good(stack: &mut Vec<i32>) -> impl FnMut(StackChange) -> Vec<StackChange> + '_ {
        move |msg| {
            let ret = match msg {
                Push(x) => {
                    stack.push(x);
                    vec![]
                }
                Pop => {
                    vec![match stack.pop() {
                        None => Value(None),
                        some => Value(some),
                    }]
                }
                Value(_) => panic!("Value in demo_impl"),
            };
            println!("demo_impl_good: {:?} -> {:?}", msg, ret);
            ret
        }
    }

    #[test]
    fn test_scenario_1() {
        let mut stack = vec![];
        judge_panic(scenario_1(), demo_impl_good(&mut stack));
    }

    /// This implementation will discard everything.
    /// This is a "valid" implementation, according to the judge.
    ///
    /// (This is because the judge allows buffering implementations, but doesn't
    /// have a force the subject to flush its buffer.)
    fn demo_impl_discard() -> impl FnMut(StackChange) -> Vec<StackChange> {
        move |_| vec![]
    }

    #[test]
    fn test_scenario_1_discard() {
        let j = judge(scenario_1(), demo_impl_discard());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Done);
    }

    /// This implementation will produce a value even if the command is 'push.'
    fn demo_impl_dumb() -> impl FnMut(StackChange) -> Vec<StackChange> {
        move |msg| match msg {
            Push(_) => vec![Value(None)],
            Pop => vec![Value(Some(0))],
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_dumb() {
        let j = judge(scenario_1(), demo_impl_dumb());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Halt("too many reactions".to_string()));
    }

    /// This implementation will remember the stack size, but will report '0' for everything.
    fn demo_impl_zero_smart(count: &mut usize) -> impl FnMut(StackChange) -> Vec<StackChange> + '_ {
        move |msg| match msg {
            Push(_) => {
                *count += 1;
                vec![]
            }
            Pop => {
                if *count == 0 {
                    vec![Value(None)]
                } else {
                    *count -= 1;
                    vec![Value(Some(0))]
                }
            }
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_zero_smart() {
        let mut count = 0;
        let j = judge(scenario_1(), demo_impl_zero_smart(&mut count));
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert!(matches!(j, Judgment::Halt(_)), "not \"Halt\": {:?}", j);
        let j = match j {
            Judgment::Halt(s) => s,
            _ => unreachable!(),
        };
        assert!(
            j.starts_with("expected"),
            "not \"expected X, got Y\": {:?}",
            j
        );
    }

    /// This implementation will always report an empty stack.
    ///
    /// Like `demo_impl_discard`, this is a "valid" implementation.
    fn demo_impl_empty() -> impl FnMut(StackChange) -> Vec<StackChange> {
        move |msg: StackChange| match msg {
            Push(_) => vec![],
            Pop => vec![Value(None)],
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_empty() {
        let j = judge(scenario_1(), demo_impl_empty());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Done);
    }

    /// This implementation will return something irrelevant when popping
    fn demo_impl_irrelevant() -> impl FnMut(StackChange) -> Vec<StackChange> {
        move |msg: StackChange| match msg {
            Push(_) => vec![],
            Pop => vec![Push(42)],
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_irrelevant() {
        let j = judge(scenario_1(), demo_impl_irrelevant());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(
            j,
            Judgment::Halt("undefined response from stack".to_string())
        );
    }

    /// This scenario contains an implementation bug
    /// (too many pops).
    fn scenario_2() -> StackJudge {
        #[rustfmt::skip]
        let sce = vec![
            Push(1), Pop, Pop
        ];
        StackJudge::new_scenario(sce)
    }

    #[test]
    fn test_scenario_2() {
        let mut stack = vec![];
        let j = judge(scenario_2(), demo_impl_good(&mut stack));
        assert_eq!(j, Err("bad sim: more pops than pushes".to_string()));
    }

    /// This scenario is when there's items in the stack after the last
    /// operation. This is a valid scenario.
    fn scenario_3() -> StackJudge {
        #[rustfmt::skip]
        let sce = vec![
            Push(1), Push(2), Push(3),
            Pop,
        ];
        StackJudge::new_scenario(sce)
    }

    #[test]
    fn test_scenario_3() {
        let mut stack = vec![];
        let j = judge(scenario_3(), demo_impl_good(&mut stack));
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Done);
    }

    /// This scenario tests a subject that only pops when the stack is empty.
    fn scenario_4() -> StackJudge {
        #[rustfmt::skip]
        let sce = vec![
            Push(1), Push(2), Push(3),
            Pop, Pop, Pop,
            Push(4), Push(5),
            Pop, Pop,
        ];
        StackJudge::new_scenario(sce)
    }

    /// And a corresponding subject implementation.
    fn demo_impl_lazy() -> impl FnMut(StackChange) -> Vec<StackChange> {
        let mut stack = vec![];
        let mut count = 0;
        move |msg: StackChange| {
            println!("msg: {msg:?}");
            match msg {
                Push(x) => {
                    stack.push(Value(Some(x)));
                    count += 1;
                    vec![]
                }
                Pop => {
                    if count > 0 {
                        count -= 1;
                        vec![Value(None)]
                    } else {
                        let mut st = std::mem::take(&mut stack);
                        st.reverse();
                        st
                    }
                }
                Value(_) => panic!("Value in demo_impl"),
            }
        }
    }

    #[test]
    fn test_scenario_4() {
        let j = judge(scenario_4(), demo_impl_lazy());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Done);
    }

    #[test]
    fn test_scenario_1_lazy() {
        let j = judge(scenario_1(), demo_impl_lazy());
        assert!(j.is_ok());
        let (j, _) = j.unwrap().decompose();
        assert_eq!(j, Judgment::Done);
    }
}
