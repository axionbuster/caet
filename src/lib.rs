//! A judge for cause-effect systems.
//!
//! Think: LeetCode, but with interactive problems.

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
    /// Any internal error type.
    type Error: core::fmt::Display;
    /// Explain why the subject is at fault.
    type Fault: core::fmt::Display;
    /// Express an observation about the universe (surroundings) made by the subject.
    type Observation;
    /// Given the ordered (earliest to latest in 0..n) reactions produced by the subject
    /// thus far since the last call...
    /// - Decide whether the response is acceptable.
    /// - Decide whether to continue or halt the program.
    /// - If continuing, produce the next observation to send to the subject.
    fn next(
        &mut self,
        reactions: Vec<Self::Observation>,
    ) -> Result<Judgment<Self::Observation, Self::Fault>, Self::Error>;
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
    pub judgment: Judgment<J::Observation, J::Fault>,
    /// Number of times the judge has called the task.
    pub calls: usize,
}

/// I got too lazy to convert the old code that didn't have the [`Outcome`] type
/// and simply used a tuple instead (which got a Clippy warning for type complexity).
trait PrivateOutcomeDecompose<J: Judge> {
    /// Decompose the outcome into its components.
    fn decompose(self) -> (Judgment<J::Observation, J::Fault>, usize);
}
impl<J: Judge> PrivateOutcomeDecompose<J> for Outcome<J> {
    fn decompose(self) -> (Judgment<J::Observation, J::Fault>, usize) {
        (self.judgment, self.calls)
    }
}

/// A test driver for a cause-effect system.
///
/// Test drive the task with the judge, and return the number
/// of times the judge has called the task.
pub fn judge<J>(
    mut judge: J,
    mut task: impl FnMut(J::Observation) -> Vec<J::Observation>,
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
                out = task(msg);
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
pub fn judge_panic<J>(j: J, task: impl FnMut(J::Observation) -> Vec<J::Observation>) -> usize
where
    J: Judge,
{
    match judge(j, task).map(|o| o.decompose()) {
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
    enum StackObservation {
        Push(i32),
        Pop,
        Value(Option<i32>),
    }
    use StackObservation::*;

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
        scenario: VecDeque<StackObservation>,
        /// Reference implementation of the stack.
        refimpl: Vec<i32>,
        /// The reactions that must be produced by the subject
        /// arranged in order.
        expect: VecDeque<i32>,
    }
    impl Judge for StackJudge {
        type Error = String;
        type Fault = String;
        type Observation = StackObservation;

        fn next(
            &mut self,
            reactions: Vec<Self::Observation>,
        ) -> Result<Judgment<Self::Observation, Self::Fault>, Self::Error> {
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
                    StackObservation::Value(Some(value)) if *value != exp => {
                        // Defeat subjects that pop the wrong value.
                        return Ok(Judgment::Halt(format!(
                            "expected {:?}, got {:?}",
                            exp, value
                        )));
                    }
                    StackObservation::Value(None) => {
                        // A subject stays silent. It may be buffering the value, or it may
                        // have just forgotten it. Buffering is allowed, so let's put it back.
                        put_back = Some(exp);
                        break;
                    }
                    // OK: Some(value) where value == exp
                    StackObservation::Value(_) => (),
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
                    StackObservation::Push(push) => self.refimpl.push(push),
                    StackObservation::Pop => {
                        if let Some(pop) = self.refimpl.pop() {
                            self.expect.push_back(pop);
                        } else {
                            return Err("bad sim: more pops than pushes".to_string());
                        }
                    }
                    StackObservation::Value(_) => return Err("bad sim: Value in scen".to_string()),
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
        fn new_scenario(scenario: Vec<StackObservation>) -> Self {
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

    fn demo_impl_good(
        stack: &mut Vec<i32>,
    ) -> impl FnMut(StackObservation) -> Vec<StackObservation> + '_ {
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
    fn demo_impl_discard() -> impl FnMut(StackObservation) -> Vec<StackObservation> {
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
    fn demo_impl_dumb() -> impl FnMut(StackObservation) -> Vec<StackObservation> {
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
    fn demo_impl_zero_smart(
        count: &mut usize,
    ) -> impl FnMut(StackObservation) -> Vec<StackObservation> + '_ {
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
    fn demo_impl_empty() -> impl FnMut(StackObservation) -> Vec<StackObservation> {
        move |msg: StackObservation| match msg {
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
    fn demo_impl_irrelevant() -> impl FnMut(StackObservation) -> Vec<StackObservation> {
        move |msg: StackObservation| match msg {
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
    fn demo_impl_lazy() -> impl FnMut(StackObservation) -> Vec<StackObservation> {
        let mut stack = vec![];
        let mut count = 0;
        move |msg: StackObservation| {
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
