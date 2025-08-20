use askit::prompt_mod::prompt;

fn main() -> Result<(), askit::Error> {
    // Default tipado: não precisa parse
    let port: u16 = prompt("Port [typed default=5432]: ")
        .to()
        .default_val(5432)
        .get()?;

    println!("Port = {port}");
    Ok(())
}
