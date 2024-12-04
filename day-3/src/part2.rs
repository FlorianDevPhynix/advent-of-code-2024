use nom::{
    branch::*, bytes::complete::tag, character::complete::*, combinator::value, multi, sequence::*,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Debug)]
pub struct ExecutionState {
    value: u32,
    mul_enabled: bool,
}

impl Instruction {
    pub fn execute(self, state: &mut ExecutionState) {
        match self {
            Instruction::Mul(a, b) => {
                if state.mul_enabled {
                    state.value = state.value + a * b;
                }
            }
            Instruction::Do => {
                state.mul_enabled = true;
            }
            Instruction::Dont => {
                state.mul_enabled = false;
            }
        }
    }
}

impl ExecutionState {
    pub fn new(start_enabled: bool) -> Self {
        Self {
            value: 0,
            mul_enabled: start_enabled,
        }
    }
}

fn mul_input(input: &str) -> IResult<&str, Instruction> {
    let (rest, (a, b)) = separated_pair(u32, char(','), u32)(input)?;
    Ok((rest, Instruction::Mul(a, b)))
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    preceded(tag("mul"), delimited(char('('), mul_input, char(')')))(input)
}

fn dos(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
    ))(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((mul, dos))(input)
}

fn parse_corrupted_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    multi::many1(crate::many_till(anychar, instruction))(input)
}

pub fn process(input: &str) -> (&str, ExecutionState) {
    let (rest, instructions) = parse_corrupted_instructions(&input).unwrap();
    let mut state = ExecutionState::new(false);
    for instruction in instructions {
        instruction.execute(&mut state);
    }
    (rest, state)
}

#[test]
fn parse_file() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (rest, state) = process(&input);
    println!("Instruction Result: {state:?} \nRest: \"{rest}\"");
}
