use nom::{bytes::complete::tag, character::complete::*, multi, sequence::*, IResult, Parser};

pub fn many_till<I, O, P, E, F, G>(mut f: F, mut g: G) -> impl FnMut(I) -> IResult<I, P, E>
where
    I: Clone + nom::InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, P, E>,
    E: nom::error::ParseError<I>,
{
    use nom::{error::ErrorKind, Err};
    move |mut i: I| {
        loop {
            let len = i.input_len();
            match g.parse(i.clone()) {
                Ok((i1, o)) => return Ok((i1, o)),
                Err(Err::Error(_)) => {
                    match f.parse(i.clone()) {
                        Err(Err::Error(err)) => {
                            return Err(Err::Error(E::append(i, ErrorKind::ManyTill, err)))
                        }
                        Err(e) => return Err(e),
                        Ok((i1, o)) => {
                            // infinite loop check: the parser must always consume
                            if i1.input_len() == len {
                                return Err(Err::Error(E::from_error_kind(
                                    i1,
                                    ErrorKind::ManyTill,
                                )));
                            }
                            i = i1;
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}

fn parse_mul_input(input: &str) -> IResult<&str, Mul> {
    let (rest, (a, b)) = separated_pair(u32, char(','), u32)(input)?;
    Ok((rest, Mul { a, b }))
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    preceded(tag("mul"), delimited(char('('), parse_mul_input, char(')')))(input)
}

fn parse_corrupted_mul_ops(input: &str) -> IResult<&str, Vec<Mul>> {
    multi::many1(many_till(anychar, parse_mul))(input)
}

fn main() {
    let (rest, output) = parse_mul("mul(5,10)").unwrap();
    println!("Output: \"{output:?}\"; Rest: \"{rest}\"");

    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let (rest, output) = match parse_corrupted_mul_ops(input) {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err.to_string());
            return;
        }
    };
    println!("Output: \"{output:?}\"; Rest: \"{rest}\"");

    let input = std::fs::read_to_string("./src/part1-input.txt").unwrap();
    let (rest, output) = parse_corrupted_mul_ops(&input).unwrap();
    let result = output
        .iter()
        .map(|instruction| instruction.a * instruction.b)
        .reduce(|acc, value| acc + value)
        .unwrap();
    println!("Instruction Result: {result} \nRest: \"{rest}\"");
}
