use rand::distr::{Alphanumeric, SampleString};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut entrada = String::new();

    let comprimento = loop {
        print!("Quantos caracteres deve ter a senha? ");
        io::stdout().flush()?;

        entrada.clear();
        if io::stdin().read_line(&mut entrada)? == 0 {
            return Ok(());
        }

        if let Ok(comprimento) = entrada.trim().parse::<usize>()
            && comprimento > 0
        {
            break comprimento;
        }

        eprintln!("Digite um número inteiro maior que zero.");
    };

    let senha = Alphanumeric.sample_string(&mut rand::rng(), comprimento);
    println!("{senha}");

    Ok(())
}
