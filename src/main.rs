pub mod genetic;
pub mod individual;
pub mod instruction;

use tinyrand::StdRand;
use clap::{arg, command, Command};
use crate::individual::evaluate_stack;
use crate::instruction::Instruction;

fn cli() -> Command {
    command!().args([
        arg!(--rangeup <INTEGER> "Maximum size of initial program"),
        arg!(--rangedown <INTEGER> "Minimum size of initial program"),
        arg!(--pop <INTEGER> "Size of population"),
        arg!(--gen <INTEGER> "Number of generations"),
        arg!(--reproduction <FLOAT> "Reproduction rate"),
        arg!(--crossover <FLOAT> "Crossover rate"),
        arg!(--addition-mutation <FLOAT> "Addition mutation rate"),
        arg!(--removal-mutation <FLOAT> "Removal mutation rate"),
    ])
}

fn main() {
    let mut dataset: Vec<Vec<i32>> = vec![];
    for i in 0..100 {
        let i = i as i32;
        dataset.push([i, i*i+i*i].to_vec());
    }
    let matches = cli().get_matches();
    let props = genetic::GeneticProperties {
        range_up: *matches.get_one::<usize>("rangeup").unwrap_or(&7),
        range_down: *matches.get_one::<usize>("rangeup").unwrap_or(&2),
        population_size: *matches.get_one::<usize>("pop").unwrap_or(&1000),
        removal_mutation_rate: *matches.get_one::<f32>("removal-mutation").unwrap_or(&0.01),
        addition_mutation_rate: *matches.get_one::<f32>("addition-mutation").unwrap_or(&0.01),
        reproduction_rate: *matches.get_one::<f32>("reproduction").unwrap_or(&0.05),
        cross_over_rate: *matches.get_one::<f32>("crossover").unwrap_or(&0.9),
    };
    {
        let mut stack = vec![Instruction::Integer(2), Instruction::Integer(3), Instruction::Sum, Instruction::Integer(2), Instruction::Neg, Instruction::Multiply];
        println!("Testing evaluate stack function...");
        assert!(evaluate_stack(&stack, vec![]) == -10);
        stack.push(Instruction::Duplicate);
        stack.push(Instruction::Multiply);
        assert!(evaluate_stack(&stack, vec![]) == 100);
        stack.push(Instruction::Integer(-1));
        stack.push(Instruction::Sum);
        stack.push(Instruction::Swap);
        assert!(evaluate_stack(&stack, vec![]) == -1);
        let stack = vec![Instruction::Sum];
        assert!(evaluate_stack(&stack, vec![2, -2]) == 0);
        println!("Testing done it's fine :)");
    }
    let mut rng = StdRand::default();
    let mut g = genetic::Genetic::new(props, &mut rng);
    g.run(200, &dataset, &mut rng);
    g.sort_population_by_fitness(&dataset);
    println!("{:?}", g.population[0].stack);
    g.sort_population_by_complexity();
    println!("{:?}", g.population[0].stack);
}
