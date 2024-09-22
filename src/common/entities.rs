use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SnippetArgs {
    #[arg(short,long)]
    pub title: String,
    #[arg(short, long, default_value_t = 1 )]
    pub count: i32,
    #[arg(short,long)]
    pub description: String,

    #[arg(short,long)]
    pub lang: String,

    #[arg(short('o'), long("code"))]
    pub code: String,
}
