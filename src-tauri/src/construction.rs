use crate::{Command, Point, State};
use serde_json::{json, Value};

pub fn parse_commands(contents: String) -> Value {
    let file_content = contents.to_lowercase();
    let file_content = file_content.replace(' ', "");
    let csv_file = file_content.lines();

    let mut execution_state: State = State::Start;
    let mut spline_state: State = State::Start;
    let mut commands: Vec<Command> = vec![];
    for line in csv_file {
        let mut line = line.split(',').collect::<Vec<&str>>();
        let command = line.first().unwrap().to_string();
        let state = line.last(); // TODO: Use this for state checking an outside component
        line.remove(0);
        line.pop();

        match command.as_str() {
            "start" => {
                let args = line
                    .iter()
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect::<Vec<f32>>();
                commands.push(Command::Start(Point::new_from_vec(args)));
                execution_state = State::Check;
                spline_state = State::Start;
            }
            "wait" => {
                if execution_state != State::Check {
                    continue;
                }
                let args = line;
                commands.push(Command::Wait(args[0].to_string()));
                execution_state = State::Check;
            }
            "goto" => {
                if execution_state != State::Check {
                    continue;
                }
                let args = line
                    .iter()
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect::<Vec<f32>>();
                commands.push(Command::Goto(Point::new_from_vec(args)));
                execution_state = State::Check;
            }
            "spline" => match execution_state {
                State::Check => {
                    commands.push(Command::Spline(vec![]));
                    execution_state = State::Spline;
                    spline_state = State::Check;
                }
                State::Spline => {
                    spline_state = State::Start;
                    execution_state = State::Check;
                }
                _ => {
                    continue;
                }
            },
            "bezier" => {
                if execution_state != State::Spline {
                    continue;
                }
                match spline_state {
                    State::Check => {
                        let args = line
                            .iter()
                            .map(|x| x.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>();

                        let point = Point::new_from_vec(args);
                        match commands.last_mut().unwrap() {
                            Command::Spline(spline) => {
                                spline.push(vec![point]);
                            }
                            _ => {
                                continue;
                            }
                        }
                        spline_state = State::Bezier;
                    }
                    State::Bezier => {
                        let args = line
                            .iter()
                            .map(|x| x.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>();
                        let point = Point::new_from_vec(args);
                        match commands.last_mut().unwrap() {
                            Command::Spline(spline) => {
                                spline.last_mut().unwrap().push(point);
                            }
                            _ => {
                                continue;
                            }
                        }
                        spline_state = State::Check;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            "control" => {
                if execution_state != State::Spline {
                    continue;
                }
                match spline_state {
                    State::Bezier => {
                        let args = line
                            .iter()
                            .map(|x| x.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>();

                        let point = Point::new_from_vec(args);
                        match commands.last_mut().unwrap() {
                            Command::Spline(spline) => {
                                spline.last_mut().unwrap().push(point);
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }

    json!(commands)
}
