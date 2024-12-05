use std::cmp::Ordering;

#[derive(Debug)]
enum LevelChange {
    Increasing,
    Decreasing,
}

impl From<Ordering> for LevelChange {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Self::Decreasing,
            Ordering::Greater => Self::Increasing,
            Ordering::Equal => unreachable!("Levels onyl ever increase or decrease"),
        }
    }
}

#[derive(Debug)]
enum Safety {
    Safe,
    SingleBadLevel,
    Unsafe,
}

#[derive(Debug)]
struct State {
    change: Option<LevelChange>,
    previous: u32,
    safety: Safety,
}

impl State {
    fn new(previous: u32) -> Self {
        Self {
            change: None,
            previous,
            safety: Safety::Safe,
        }
    }

    fn record_unsafe_level(&mut self) {
        match self.safety {
            Safety::Safe => self.safety = Safety::SingleBadLevel,
            Safety::SingleBadLevel => self.safety = Safety::Unsafe,
            Safety::Unsafe => {
                unreachable!("Reports with unsafe levels should not be checked anymore.")
            }
        }
    }

    fn is_safe(&self) -> bool {
        matches!(self.safety, Safety::Safe | Safety::SingleBadLevel)
    }
}

pub fn process(input: &str) -> u32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        // check each Report(line)
        let mut state: Option<State> = None;
        println!("{line}");
        let levels = line
            .split_whitespace()
            .map(|level| level.parse::<u32>().unwrap());
        for level in line
            .split_whitespace()
            .map(|level| level.parse::<u32>().unwrap())
        {
            println!(
                "level: {level} {state:?} {:?}",
                state.as_ref().map(|state| state.previous.cmp(&level))
            );
            match state {
                None => {
                    state = Some(State::new(level));
                }
                Some(ref mut state) => {
                    let change = state.previous.abs_diff(level);
                    if change < 1 || 3 < change {
                        // level changes too much or not at all, Report not safe
                        state.record_unsafe_level();
                        if !state.is_safe() {
                            break;
                        } else if matches!(state.safety, Safety::SingleBadLevel) {
                            continue;
                        }
                    }

                    match state.change {
                        None => state.change = Some(level.cmp(&state.previous).into()),
                        Some(ref mut change) => match change {
                            LevelChange::Increasing => {
                                if state.previous > level {
                                    // level change direction changed, Report not safe
                                    state.record_unsafe_level();
                                    if !state.is_safe() {
                                        break;
                                    } else if matches!(state.safety, Safety::SingleBadLevel) {
                                        continue;
                                    }
                                }
                            }
                            LevelChange::Decreasing => {
                                if state.previous < level {
                                    // level change direction changed, Report not safe
                                    state.record_unsafe_level();
                                    if !state.is_safe() {
                                        break;
                                    } else if matches!(state.safety, Safety::SingleBadLevel) {
                                        continue;
                                    }
                                }
                            }
                        },
                    }
                    state.previous = level;
                }
            }
        }

        if matches!(state, Some(state) if state.is_safe()) {
            // count as safe report
            safe_reports += 1;
        };
    }
    safe_reports
}

#[test]
fn parse_file() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let amount_safe_reports = process(&input);
    println!("Amount of safe Reports: {amount_safe_reports}");
}