use std::fs;
use std::collections::{HashMap,VecDeque};
use num::integer::lcm;

struct FlipFlopState {
    state: bool
}

struct ConjunctionState<'a> {
    input_states: HashMap<&'a str, bool>
}

enum ModuleType<'a> {
    FlipFlop(FlipFlopState),
    Broadcaster,
    Conjunction(ConjunctionState<'a>),
}

struct Module<'a> {
    module_type: ModuleType<'a>,
    name: &'a str,
    outputs: Vec<&'a str>
}

fn parse_module<'a>(line: &'a str) -> Module<'a> {
    let (module, outputs) = line.split_once(" -> ").unwrap();

    let outputs: Vec<_> = outputs.split(", ").collect();

    if module.starts_with('%') {
        return Module {
            module_type: ModuleType::FlipFlop(FlipFlopState { state: false }),
            name: &module[1..],
            outputs: outputs
        }
    } else if module.starts_with('&') {
        return Module {
            module_type: ModuleType::Conjunction(ConjunctionState { input_states: HashMap::new() }),
            name: &module[1..],
            outputs: outputs
        }
    } else if (module == "broadcaster") {
        return Module {
            module_type: ModuleType::Broadcaster,
            name: module,
            outputs: outputs
        }
    }
    panic!();
}

fn part1() {
    let contents = fs::read_to_string("./src/input20.txt").unwrap();
    let mut modules: HashMap<&str, Module> = contents.lines()
        .map(parse_module)
        .map(|module| (module.name, module)).collect();

    let mut module_to_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    for module in modules.values() {
        for output in module.outputs.iter() {
            if let Some(inputs) = module_to_inputs.get_mut(output) {
                inputs.push(module.name);
            } else {
                module_to_inputs.insert(output, vec![module.name]);
            }
        }
    }

    let mut low_count = 0;
    let mut high_count = 0;

    // Part 2 exceeds 100000000

    for i in 0..1000 {
        let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::new();
        pulses.push_back(("button", "broadcaster", false));

        while pulses.len() > 0 {
            let (src_name, dst_name, value) = pulses.pop_front().unwrap();

            if value {
                low_count += 1;
            } else {
                high_count += 1;
            }

            // if dst_name == "vz" && !value {
            //     println!("vz gets pulse: {}, {}, src:{}", i + 1, value, src_name);
            //     // return;
            // }
            if let Some(module) = modules.get_mut(dst_name) {
                let to_send: Option<bool> = match &mut module.module_type {
                    ModuleType::Broadcaster => Some(value),
                    ModuleType::FlipFlop(state) => {
                        if !value {
                            state.state = !state.state;
                            Some(state.state)
                        } else {
                            None
                        }
                    }
                    ModuleType::Conjunction(state) => {
                        state.input_states.insert(src_name, value);
                        let all_initialized = module_to_inputs.get(module.name).unwrap().len() == state.input_states.len();
                        Some(!(all_initialized && state.input_states.values().all(|v| *v)))
                    }
                };
                
                if let Some(value_to_send) = to_send {
                    for output in module.outputs.iter() {
                        pulses.push_back((module.name, output, value_to_send))
                    }
                }
            }
        }
    }

    println!("{}", low_count * high_count);
}

fn part2() {
    // Periods for vz, bq, qh, and lt
    println!("{}", vec![4093_i64, 3889, 3821, 3739].into_iter().reduce(lcm).unwrap());
}

fn main() {
    part2();
}