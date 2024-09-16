use colored::Colorize;

pub fn pull(name: String) -> anyhow::Result<()> {
    println!();
    println!("Using registry {}..", "github.com".bold());
    println!("Pulling flavour {}...", name.blue());
    println!("Pulling flavour {}...     1%      (1MB/100MB)", name.blue());
    println!(
        "Pulling flavour {}...     20%     (20MB/100MB)",
        name.blue()
    );
    println!(
        "Pulling flavour {}...     74%     (74MB/100MB)",
        name.blue()
    );
    println!(
        "Pulling flavour {}...     95%     (95MB/100MB)",
        name.blue()
    );
    println!(
        "Pulling flavour {}...     100%    (100MB/100MB)",
        name.blue()
    );
    println!();
    println!("{}", "Successfully pulled flavour!".green());
    println!();
    println!("To start, execute");
    println!();
    println!("{}", format!("oam run -f {}", "axum".blue()).bold());
    println!();

    Ok(())
}
