mod snippet;
use clap::Parser;
use snippet::Snippet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    count: u8,
}

fn main() {
    let snippet = Snippet::new("echo".to_string(), "test".to_string(), None);
    let toml = toml::to_string(&snippet).unwrap();
    println!("{}", toml);

    let from_toml: Snippet = toml::from_str(r#"
      command = 'echo'
      description = 'echos a thing'
"#).unwrap();

    assert_eq!(from_toml.command, "echo");
}
