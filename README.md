# Logeagle Dashboard 🦀 

This project implements a Rust web server using Rocket.rs to showcase Apache Parquet files. The server serves a web page displaying log data from `access.parquet` and `error.parquet` files.

## Features

- Rust web server powered by Rocket.rs
- Display log data from Parquet files
- Separate sections for access logs and error logs
- Simple and clean HTML/CSS layout

## Usage

1. Make sure you have Rust installed.
2. Clone this repository.
3. Place your `access.parquet` and `error.parquet` files in the root directory of the project.
4. Navigate to the project directory in the terminal.
5. Run `cargo run`.
6. Open your web browser and visit `http://localhost:8000` to see the dashboard.

## Project Structure

```
logeagle/
│
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── templates/
│   │   └── index.html
│   └── static/
│       └── style.css
├── access.parquet
└── error.parquet
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## About

This project is a part of Logeagle, a platform for log management and analysis. Visit [logeagle.com](https://logeagle.github.io) for more information.
