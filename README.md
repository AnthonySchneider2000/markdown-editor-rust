# Markdown to HTML Converter

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

This Markdown to HTML Converter is a simple yet powerful Rust application that allows you to convert Markdown text into HTML format. It's designed with a user-friendly interface and offers two key features: saving the HTML output as a file and copying it to the clipboard.

## Features

- **Markdown to HTML Conversion**: Convert Markdown text to HTML with ease. Simply paste your Markdown content into the input box, press "Submit," and the HTML result will appear in the output box.

- **Save as HTML**: If you want to save the HTML output as a file, check the "Save as HTML" checkbox before clicking "Submit." The HTML will be saved in an "output.html" file in the project directory.

- **Copy to Clipboard**: To quickly copy the HTML output to your clipboard, check the "Copy to clipboard" checkbox before clicking "Submit." You can then paste the HTML wherever you need it.

## Technologies Used

- [Rust](https://www.rust-lang.org/): The core programming language used for building the application.

- [Druid](https://docs.rs/druid/latest/druid/): A Rust-native UI toolkit that provides the graphical user interface for this application.

## Getting Started

1. Clone the repository to your local machine.

2. Install Rust and Cargo if you haven't already. You can download them from [the official Rust website](https://www.rust-lang.org/).

3. Navigate to the project directory in your terminal.

4. Run the application using the following command:

   ```bash
   cargo run
    ```
5. The application window will open, and you can start converting Markdown to HTML.

## License
This project is licensed under the MIT License - see the [LICENSE](https://github.com/AnthonySchneider2000/markdown-editor-rust/blob/master/LICENSE) file for details.

## Example Input
For testing purposes, a file named "MarkdownExample.txt" has been included in the project directory. This file contains example input text using a variety of Markdown tags. You can use this file to experiment with the converter and see how different Markdown styles are converted to HTML.

## Demo
Below are screenshots displaying the program's GUI, its input and output, and the output HTML file displayed in browser.

![GUI](https://i.imgur.com/UBrvDxQ.png)

![HTML](https://i.imgur.com/pN8gCEv.png)

---

Enjoy converting Markdown to HTML with ease! If you encounter any issues or have suggestions for improvements, please feel free to create an issue on this GitHub repository.