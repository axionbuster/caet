//! A judge for cause-effect systems.
//!
//! This crate allows you to simulate an external environment that interacts with your object
//! OO system prototype. This is useful for validating your OO design.
//!
//! ## How does it help me?
//!
//! Writing a simulation implies writing code, but not *real* code that interacts
//! with the actual system (and that would burden you with the details of the system's
//! implementation), but *fake* code that interacts with a *fake* system that you
//! have designed to be as close to the real system as necessary.
//!
//! So, after analyzing your problem, and applying certain design patterns to
//! plan what code you will write, you can use this crate to test your idea by writing
//! lightweight, throwaway, fake code that interacts with your fake system.
//!
//! Unlike real code, it's only supposed to take you one or two hours to flesh out
//! an entire system. And, unlike real code, you can keep it as long as you'd like
//! without worrying about keeping up with the changes in the real system.
//!
//! ### Use cases
//!
//! - Discussion and reflection: You can use this crate to discuss or document your design.
//! - Prototyping: You can use this crate to prototype your design.
//! - Validation: You can use this crate to validate your design.
//! - Mocking: You can use this crate to mock your design. However,
//! you'll also be mocking your implementation, which may or may not be what you want.
//!
//! ### Benefits Profile
//!
//! This crate satisfies this niche:
//!
//! **Moderate accuracy, moderate cost**: You do need to write code, mostly throwaway
//! code at that, so this is more expensive than validating your idea on paper,
//! but it's much cheaper than writing real code.
//!
//! It's often believed that the only way to accurately gauge the quality of a design
//! is to write real code. As they say: you can't know until you try.
//! **No!** This crate is the counterexample to that belief.
//! Save yourself the time and effort of getting stuck with the wrong design,
//! or the pressure to turn your flimsy prototype (with tons of anomalies) as the real thing.
//!
//! What this crate offers you: write a little code about a little part of your system,
//! and implement a universe that is limited, fair, hostile to your object.
//! See if your object keeps up with the universe. Also, try to get a feel for
//! what it's like to actually turn your architecture into code.
//!
//! Was it too verbose? Was it too complicated? Did it introduce a fundamental
//! anomaly in your system? Did you miss something important?
//! Did you lose flexibility? Did you gain flexibility?
//! What did you feel after getting a taste of your design? Did you like it?
//!
//! You can answer these questions without writing real code, plus, discuss and document
//! your assumptions about the universe and the object's responsibilities.
//!
//! ## How to use this crate to test your OO system or prototype
//!
//! First, there's a full example right here on this page. You might have to scroll
//! to the bottom of the page to see it.
//!
//! And, there's also a more elaborate example in the `lib.rs` file of this crate.
//! It's called `mod test_stack`. It's much bigger than the example on this page,
//! but also comprehensive. It's a simulation of a stack data structure.
//! Look at the example and see if `caet` is a good fit for you.
//!
//! Now, let's get to the details.
//!
//! This crate has exactly three public types:
//! - trait [`Judge`]: A god-like entity that controls the universe and judges the object's reactions.
//! - struct [`Outcome`]: The final result of a test.
//! - struct [`Judgment`]: A judgment about an object's reaction evaluated at a particular time.
//! Also contains the next input to the object.
//!
//! Your job is to implement the trait [`Judge`].
//!
//! And, essentially, one public function:
//! - [`judge`]: The entry point into this crate. This is a test runner.
//! It takes the ownership of your judge implementation and your object,
//! and runs the simulation. Once it's finished, it returns an [`Outcome`].
//!
//! So, it's actually pretty simple. You implement the trait [`Judge`],
//! you implement your object, then you test it by calling [`judge`].
//! That's it.
//!
//! ### First, a synopsis.
//!
//! The `caet` crate is a flexible testing system.
//!
//! Any judge and any object can work together as long as they agree on one thing:
//! the "change" data type.
//!
//! The "change" data type is the type of the object's sensory experiences and reactions.
//! In other words, it really defines (the alphabet of) the language of the universe.
//!
//! That means, anyone that speaks the same language (i.e. uses the same "change" data type)
//! can work together.
//!
//! ### [`Judge`]
//!
//! Start by implementing the trait [`Judge`].
//!
//! There are three associated types you must implement:
//! - **`Change`**: A combined observation and reaction type.
//! - `Fault`: In case a reaction is unacceptable, this type is used to report the reason.
//! - `Error`: A judge can also hit an error, in which case this type is used to report the reason.
//!
//! Though you should define all three, only the first one (`Change`) is relevant to the OO design.
//!
//! The second and third types are essentially for error handling. It's not super important.
//! You can make them `String` if you want.
//!
//! Then, you must implement the trait method [`Judge::next`].
//!
//! Before we discuss [`Judge::next`], you should think of the simulation as some sort of game or battle.
//!
//! The "god" (i.e. the judge) controls the universe and the object.
//! The judge's goal is to defeat the object by making it hit a fault
//! (a rule-breaking or otherwise inappropriate reaction).
//! Meanwhile, the object's goal is to survive the universe by not hitting a fault,
//! and continue until it outsmarts the judge (i.e., ends the simulation).
//! (Of course, you control both, so you can make them as boring or as interesting as you want.)
//!
//! So, the [`Judge::next`] method should be thought of as the judge's next move.
//!
//! Namely, it will read all the reactions of the object that hasn't been judged yet,
//! and then it will decide what to do next.
//!
//! The judge controls the lifetime of the universe. It can end the simulation at any time.
//! The judge is also responsible for judging the object's reactions.
//! Both of these responsibilities are combined in the return value of [`Judge::next`]:
//! - If the judge decides the reaction is acceptable and still has a challenge for the object,
//! it will return `Ok(Judgment::Continue(next_input))` with whatever the `next_input` is.
//! - If the judge decides the reaction is acceptable but has no more challenges for the object,
//! it will return `Ok(Judgment::Done)` to end the simulation.
//! - Lastly, if the judge decides the reaction is unacceptable, it will return
//! `Err(Judgment::Halt(fault))` with whatever the `fault` is. This also ends the simulation.
//!
//! In [`Judge::next`], you'll be implementing the judge's decision-making process.
//!
//! Actually, this single method is the only thing you need to implement, and
//! it conceptually defines the judge's identity and the rules of the universe.
//!
//! ### Your object
//!
//! The definition of an object is any *thing* that reacts to changes in its environment.
//!
//! Because the "thing" concept is so general, it would be unreasonable to require
//! you to define it as some sort of a concrete data type. Instead, in this crate,
//! your object will be represented by a closure, called its **reaction function**.
//!
//! In the signature of [`judge`], you must provide a closure that implements
//! `FnMut(J::Change) -> Vec<J::Change>` where `J` is your implementation of [`Judge`].
//! This closure is your object's reaction function, and, therefore, its representation.
//!
//! Let's break this down.
//!
//! - `FnMut`: The object has private state that it can mutate, which it persists
//! while it is called multiple times.
//! - `(J::Change)`: It makes a passive observation of the universe, though it is
//! actually given by the judge.
//! - `-> Vec<J::Change>`: It reacts to the observation by producing a vector of changes.
//! It's a vector because it can produce multiple changes at once, or none at all.
//! Pay close attention to the doctrine of non-immediacy of reactions: This doctrine
//! says that, unlike observations, reactions are not immediate. In other words,
//! it's possible for the object to bunch up its reactions and produce them all at once
//! as a way to defeat the judge. The judge should generally agree that this is a valid strategy.
//! Otherwise, the judge is at fault. (But of course, this is up to how you implement your judge.
//! I only strongly recommend that you follow this doctrine.)
//!
//! So in summary, the "object" is abstracted away, hidden behind the `FnMut` closure.
//! The `caet` crate will never ever touch it directly. Instead, you will be providing
//! a closure that stands in for your object.
//!
//! In more conventional OOP terms, the closure is a proxy, translating each "command"
//! (or event) to your real object, and then translating each "response" (or reaction)
//! back to the judge.
//!
//! But, your proxy can actually get a bit smarter than the underlying object,
//! by bunching up reactions and releasing them all at once perhaps in unpredictable ways.
//! May be it will even re-order the reactions. The precise rules for what's valid
//! and invalid, and how synchronization is done,
//! should be agreed upon between the judge and the object.
//!
//! However, the doctrine of the non-immediacy of reactions is a good starting point.
//!
//! ### [`judge`] (lowercase)
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
//! But, here, let's do a much simpler one that concisely demonstrates the
//! basic idea of the crate.
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
//!
//!         // In reality, according to the doctrine of non-immediacy of reactions,
//!         // the object can bunch up its reactions and produce them all at once,
//!         // which is, by the doctrine, a valid strategy. If I followed this
//!         // doctrine, then this judge is actually incorrect, because it's
//!         // expecting the object to react immediately. But, I'm not following
//!         // the doctrine for the sake of simplicity.
//!
//!         // (Of course, the doctrine does allow for the possibility of adding
//!         // synchronization primitives to the universe, after which
//!         // the object *must* react "soon," or the judge will keep waiting
//!         // until it's waited "too long" and fails the object. But that's
//!         // not relevant here.)
//!
//!         Ok(Judgment::Halt(format!("You can't just pass a turn.")))
//!     }
//! }
//!
//! // My object (bisect)
//! let mut lower_bound = -1000;
//! let mut upper_bound = 1000;
//! let mut guess = 0;
//! // (Proxy for my object)
//! let mut proxy = |observation| {
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
//!     &mut proxy,
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
