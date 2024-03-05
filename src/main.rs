pub mod render;

macro_rules! time {
    ($name:expr, $e:expr) => {{
        let start = std::time::Instant::now();
        let result = $e;
        let elapsed = start.elapsed();
        println!("{} took {:?}", $name, elapsed);
        result
    }};
}

fn motd(in_file: &str, out_file: &str) -> String {
    format!(
        "leaf v{} - {} to {}",
        env!("CARGO_PKG_VERSION"),
        in_file,
        out_file
    )
}

fn ingest(file_name: &str) -> Result<sbm::Sbm, Box<dyn std::error::Error>> {
    Ok(sbm::Sbm::new(sbm::parser::parse_categories(
        &std::fs::read_to_string(file_name)?,
    )?))
}

fn export(file_name: &str, sbm: &sbm::Sbm) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(file_name, render::render(sbm))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }
    println!("{}", motd(&args[1], &args[2]));
    let sbm = time!("ingest", ingest(&args[1])?);
    time!("export", export(&args[2], &sbm))?;
    Ok(())
}
