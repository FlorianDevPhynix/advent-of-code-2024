use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

/// does not collect the input
pub(crate) fn many_till<I, O, P, E, F, G>(mut f: F, mut g: G) -> impl FnMut(I) -> IResult<I, P, E>
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
