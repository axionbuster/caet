//! A judge for cause-effect systems.
//!
//! Think: LeetCode, but with interactive problems.

/// A judgment of a cause-effect system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Judgment<M, S> {
    /// Acceptable; use this input.
    Green(M),
    /// Unacceptable (with why not); terminate the program.
    Red(S),
    /// Probably acceptable, but the judge has no more inputs
    /// in this case.
    Yellow,
}

/// A judge for a cause-effect system.
///
/// See also: [`judge`].
pub trait Judge {
    /// Any internal error type.
    type Error: core::fmt::Display;
    /// Explainer for failures.
    type Failure: core::fmt::Display;
    /// A "message" data type.
    type Message;
    /// Given the ordered (earliest to latest in 0..n) effects,
    /// return the judgment and the next message.
    fn next(
        &mut self,
        effects: Vec<Self::Message>,
    ) -> Result<Judgment<Self::Message, Self::Failure>, Self::Error>;
}

/// A test driver for a cause-effect system.
///
/// Test drive the task with the judge, and return the number
/// of times the judge has called the task.
pub fn judge<J>(
    mut judge: J,
    mut task: impl FnMut(J::Message) -> Vec<J::Message>,
) -> Result<(Judgment<J::Message, J::Failure>, usize), J::Error>
where
    J: Judge,
{
    use std::mem;
    let mut out = vec![];
    let mut ite = 0;
    loop {
        let jud = judge.next(mem::take(&mut out));
        match jud {
            Ok(Judgment::Green(msg)) => {
                out = task(msg);
                ite += 1;
            }
            Ok(x) => return Ok((x, ite)),
            Err(e) => return Err(e),
        }
    }
}

pub fn judge_panic<J>(j: J, task: impl FnMut(J::Message) -> Vec<J::Message>) -> usize
where
    J: Judge,
{
    match judge(j, task) {
        Ok((Judgment::Yellow, count)) => count,
        Ok((Judgment::Red(why), count)) => {
            panic!("subject fault (iter count: {count}): {why}")
        }
        Ok((Judgment::Green(_), count)) => {
            panic!("judge fault (iter count: {count}): judge stopped at green [meaning continue]")
        }
        Err(e) => panic!("judge fail (internal error): {e}"),
    }
}

#[cfg(test)]
mod test_stack {
    use std::collections::VecDeque;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StackMessage {
        Push(i32),
        Pop,
        Value(Option<i32>),
    }
    use StackMessage::*;

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    struct StackJudge {
        scenario: VecDeque<StackMessage>,
        working: Vec<i32>,
        expect: VecDeque<i32>,
    }
    impl Judge for StackJudge {
        type Error = String;
        type Failure = String;
        type Message = StackMessage;

        fn next(
            &mut self,
            effects: Vec<Self::Message>,
        ) -> Result<Judgment<Self::Message, Self::Failure>, Self::Error> {
            if self.expect.len() < effects.len() {
                return Ok(Judgment::Red("too many effects".to_string()));
            }
            for (eff, exp) in effects.iter().zip(self.expect.drain(0..effects.len())) {
                match eff {
                    StackMessage::Value(Some(value)) if *value != exp => {
                        return Ok(Judgment::Red(format!(
                            "expected {:?}, got {:?}",
                            exp, value
                        )))
                    }
                    StackMessage::Value(None) => {
                        return Ok(Judgment::Red("falsely reported empty stack".to_string()))
                    }
                    StackMessage::Value(_) => (),
                    _ => return Ok(Judgment::Red("undefined response from stack".to_string())),
                }
            }
            if let Some(act) = self.scenario.pop_front() {
                match act {
                    StackMessage::Push(push) => self.working.push(push),
                    StackMessage::Pop => {
                        if let Some(pop) = self.working.pop() {
                            self.expect.push_back(pop);
                        } else {
                            return Err("bad sim: more pops than pushes".to_string());
                        }
                    }
                    StackMessage::Value(_) => return Err("bad sim: Value in scen".to_string()),
                }
                return Ok(Judgment::Green(act));
            } else {
                if self.expect.is_empty() {
                    Ok(Judgment::Yellow)
                } else {
                    Ok(Judgment::Red("too few effects".to_string()))
                }
            }
        }
    }
    impl StackJudge {
        #[allow(dead_code)]
        fn new() -> Self {
            Self::default()
        }
        fn new_scenario(scenario: Vec<StackMessage>) -> Self {
            Self {
                scenario: scenario.into(),
                ..Default::default()
            }
        }
    }

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

    fn demo_impl_good(stack: &mut Vec<i32>) -> impl FnMut(StackMessage) -> Vec<StackMessage> + '_ {
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
            // println!("demo_impl_good: {:?} -> {:?}", msg, ret);
            ret
        }
    }

    #[test]
    fn test_scenario_1() {
        let mut stack = vec![];
        judge_panic(scenario_1(), demo_impl_good(&mut stack));
    }

    /// This implementation will discard everything.
    fn demo_impl_discard() -> impl FnMut(StackMessage) -> Vec<StackMessage> {
        move |_| vec![]
    }

    #[test]
    fn test_scenario_1_discard() {
        let j = judge(scenario_1(), demo_impl_discard());
        assert!(j.is_ok());
        let (j, _) = j.unwrap();
        assert_eq!(j, Judgment::Red("too few effects".to_string()));
    }

    /// This implementation will say '0' for everything, and will never run out.
    fn demo_impl_zero() -> impl FnMut(StackMessage) -> Vec<StackMessage> {
        move |_| vec![Value(Some(0))]
    }

    #[test]
    fn test_scenario_1_zero() {
        let j = judge(scenario_1(), demo_impl_zero());
        assert!(j.is_ok());
        let (j, _) = j.unwrap();
        assert_eq!(j, Judgment::Red("too many effects".to_string()));
    }

    /// This implementation will remember the stack size, but will report '0' for everything.
    fn demo_impl_zero_smrat(
        count: &mut usize,
    ) -> impl FnMut(StackMessage) -> Vec<StackMessage> + '_ {
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
    fn test_scenario_1_zero_smrat() {
        let mut count = 0;
        let j = judge(scenario_1(), demo_impl_zero_smrat(&mut count));
        assert!(j.is_ok());
        let (j, _) = j.unwrap();
        assert!(matches!(j, Judgment::Red(_)), "not \"Red\": {:?}", j);
        let j = match j {
            Judgment::Red(s) => s,
            _ => unreachable!(),
        };
        assert!(
            j.starts_with("expected"),
            "not \"expected X, got Y\": {:?}",
            j
        );
    }

    /// This implementation will always report an empty stack
    fn demo_impl_empty() -> impl FnMut(StackMessage) -> Vec<StackMessage> {
        move |msg: StackMessage| match msg {
            Push(_) => vec![],
            Pop => vec![Value(None)],
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_empty() {
        let j = judge(scenario_1(), demo_impl_empty());
        assert!(j.is_ok());
        let (j, _) = j.unwrap();
        assert_eq!(j, Judgment::Red("falsely reported empty stack".to_string()));
    }

    /// This implementation will return something irrelevant when popping
    fn demo_impl_irrelevant() -> impl FnMut(StackMessage) -> Vec<StackMessage> {
        move |msg: StackMessage| match msg {
            Push(_) => vec![],
            Pop => vec![Push(42)],
            Value(_) => panic!("Value in demo_impl"),
        }
    }

    #[test]
    fn test_scenario_1_irrelevant() {
        let j = judge(scenario_1(), demo_impl_irrelevant());
        assert!(j.is_ok());
        let (j, _) = j.unwrap();
        assert_eq!(
            j,
            Judgment::Red("undefined response from stack".to_string())
        );
    }

    /// This scenario contains an implementation bug
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
}
