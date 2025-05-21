# Quackmire

Quackmire is an interactive DuckDB client written in Rust using the [ratatui](https://github.com/tui-rs-revival/ratatui) library. It provides a terminal-based interface for executing SQL queries and exploring data files.

![](./assets/Quackmire_Demo.gif)

## Features

- File explorer for easy navigation of scripts and data files
- Multi-line query input area
- Results display in a customizable table format
- Error handling with pop-up notifications

## Installation

To install Quackmire, you need to have Rust and Cargo installed on your system. If you don't have them installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/jojayaro/quackmire.git
cd quackmire
cargo build --release
```

The compiled binary will be available in `target/release/quackmire`.

## Usage

To start Quackmire, run:

```bash
./target/release/quackmire
```

### Key Bindings

- `Super + Arrow keys`: Navigate the file explorer
- `F2`: Execute the current query
- `Ctrl + O`: Open and load a file into the query area
- `Ctrl + S`: Insert the current file path into the query area
- `Esc`: Exit the application or close error popup

## Contributing

We welcome contributions to Quackmire! Here's how you can help:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ratatui](https://github.com/tui-rs-revival/ratatui) for the TUI library
- [DuckDB](https://duckdb.org/) for the database engine

## Contact

If you have any questions or feedback, please open an issue on this repository.

---

Happy querying with Quackmire! 🦆
