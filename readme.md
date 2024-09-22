# snipcli ( IN PROGRESS )

![Snipcli Logo](https://github.com/yourusername/snipcli/blob/main/logo.png?raw=true)

A lightweight command-line tool to manage, store, and retrieve code snippets directly from your terminal. Enhance your productivity by organizing and accessing your reusable code blocks with ease.

## Features

- **Add Snippets**: Save new code snippets with titles, descriptions, and language tags.
- **List Snippets**: Display all saved snippets with filtering options by language or keywords.
- **Retrieve Snippets**: Fetch and display snippets by title or ID, with the option to copy to clipboard.
- **Delete Snippets**: Remove unwanted snippets from your storage.
- **Edit Snippets**: Modify existing snippets directly from the CLI.
- **Export/Import**: Backup or share your snippets by exporting to or importing from JSON or Markdown files.
- **Search Functionality**: Search snippets based on keywords in titles, descriptions, or code content.

## Installation

### Using Cargo

Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

```bash
cargo install snipcli
```

### From Source

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/snipcli.git
cd snipcli
cargo build --release
```

The binary will be located at `./target/release/snipcli`. You can move it to a directory in your `PATH` for easier access:

```bash
cp ./target/release/snipcli /usr/local/bin/
```

## Usage

After installation, you can use `snipcli` with various commands. Below are some examples to get you started:

### Add a Snippet

Save a new snippet with a title, language, description, and code file:

```bash
snipcli add --title "Quick Sort in Rust" --lang rust --description "Implementation of quick sort algorithm" --file quick_sort.rs
```

### List Snippets

List all saved snippets:

```bash
snipcli list
```

Filter snippets by programming language:

```bash
snipcli list --lang javascript
```

### Get a Snippet

Retrieve and display a snippet by its title:

```bash
snipcli get "Quick Sort in Rust"
```

Copy the snippet's code to the clipboard:

```bash
snipcli get "Quick Sort in Rust" --copy
```

### Delete a Snippet

Remove a snippet by its title:

```bash
snipcli delete "Old Snippet Title"
```

### Edit a Snippet

Modify an existing snippet:

```bash
snipcli edit "Quick Sort in Rust"
```

This command will open the snippet in your default editor for editing.

### Export Snippets

Export all snippets to a JSON file:

```bash
snipcli export --format json --output snippets_backup.json
```

Export snippets to a Markdown file:

```bash
snipcli export --format markdown --output snippets.md
```

### Import Snippets

Import snippets from a JSON or Markdown file:

```bash
snipcli import --file snippets_backup.json
```

### Search Snippets

Search for snippets containing specific keywords:

```bash
snipcli search "async function"
```

## Configuration

`snipcli` stores snippets in a JSON file located in the user's data directory. By default, it uses the appropriate directory based on your operating system (e.g., `$HOME/.local/share/snipcli/snippets.json` on Linux).

You can customize the storage path or other settings by modifying the configuration file or using environment variables (if implemented).

## Contributing

Contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting pull requests, your help is appreciated.

1. **Fork the Repository**

   Click the [Fork](https://github.com/yourusername/snipcli/fork) button at the top right of the repository page.

2. **Clone Your Fork**

   ```bash
   git clone https://github.com/yourusername/snipcli.git
   cd snipcli
   ```

3. **Create a New Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**

   Implement your feature or fix.

5. **Commit Your Changes**

   ```bash
   git commit -m "Add feature: your feature description"
   ```

6. **Push to Your Fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Open a Pull Request**

   Go to the original repository and open a pull request from your fork.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- Built with [Rust](https://www.rust-lang.org/) and [Clap](https://github.com/clap-rs/clap).
- Inspired by the need for efficient snippet management in the terminal.

## Support

If you encounter any issues or have questions, feel free to open an [issue](https://github.com/yourusername/snipcli/issues) on GitHub.

---

*Happy Coding! 🚀*
