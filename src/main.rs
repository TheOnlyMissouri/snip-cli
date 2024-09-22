use clap::Parser;
use snip_cli::common::entities::SnippetArgs;

fn main() {
    let args: SnippetArgs = SnippetArgs::parse();

    for _ in 0..args.count {

        println!("lang {}!", args.lang);

    }
}
