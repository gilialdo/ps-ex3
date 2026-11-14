use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut entrada = String::new();

    print!("Quantos caracteres deve ter a senha? ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;

    let comprimento = entrada
        .trim()
        .parse::<usize>()
        .map_err(|erro| io::Error::new(io::ErrorKind::InvalidInput, erro))?;
    println!("Comprimento solicitado: {comprimento}");

    Ok(())
}
