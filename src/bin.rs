use std::env;

use llama2_rs::{load_tokenizer, load_model, generate, Random, fx};

fn main() {
    // poor man's Rust argparse
    // 'checkpoint' is necessary arg
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("Usage: <checkpoint_file> [temperature] [steps] [prompt]");
        return;
    }
    let checkpoint = args[0].clone();
    let temperature = if args.len() >= 2 {
        args[1].parse().expect("temperature must be float")
    } else {
        0.9
    };

    let mut steps = if args.len() >= 3 {
        args[2].parse().expect("steps must be int")
    } else {
        256
    };
    let prompt = args.get(3).unwrap_or(&String::from("")).to_owned();

    let mut random = Random::new();

    // read in the model.bin file
    let transformer = load_model(&checkpoint);

    // right now we cannot run for more than config.seq_len steps
    if steps <= 0 || steps > transformer.config.seq_len {
        steps = transformer.config.seq_len;
    }

    let tokenizer = load_tokenizer("tokenizer.bin", &transformer.config);

    let (gen_time, _) = generate(
        &transformer,
        &tokenizer,
        &prompt,
        steps,
        &mut random,
        temperature,
        true
    );

    // report achieved tok/s
    println!(
        "\nachieved tok/s: {}",
        ((steps - 1) as fx / gen_time as fx) * 1000.0
    );
}
