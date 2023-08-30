use lykkehjul::run;

fn main() {
    let mut args = std::env::args();
    let file = args.nth(1).unwrap();
    let content = std::fs::read_to_string(file).unwrap();
    let vec = content.lines()
        .map(|s| (s.to_string(), 1))
        .collect();

    run(vec);
}
