use druid::widget::{Button, Checkbox, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
extern crate pulldown_cmark;
use pulldown_cmark::{html, Options, Parser};
use std::io::Write;
use std::fs::File;
use clipboard::{ClipboardContext, ClipboardProvider};

const CUSTOM_FONT_SIZE: f64 = 30.0;
const TEXT_BOX_WIDTH: f64 = 780.0;
const TEXT_BOX_HEIGHT: f64 = 600.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Text Input and Output");

#[derive(Clone, Data, Lens)]
struct HelloState {
    input_text: String,
    output_text: String,
    save_as_html: bool,
    copy_to_clipboard: bool,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((1600.0, 900.0)); //16:9 aspect ratio

    let initial_state = HelloState {
        input_text: String::new(),
        output_text: String::new(),
        save_as_html: false,
        copy_to_clipboard: false,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn save_to_file(output: &str) {
    let mut file = File::create("output.html").expect("Unable to create file");
    file.write_all(output.as_bytes())
        .expect("Unable to write data");
}

fn copy_to_clipboard(output: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output.to_owned()).unwrap();
}

fn convert_markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new_ext(markdown, Options::all());
    html::push_html(&mut html_output, parser);
    html_output
}

fn build_root_widget() -> impl Widget<HelloState> {
    // Create a TextBox for input
    let input_textbox = TextBox::multiline()
        .with_placeholder("Enter text here...")
        .fix_width(TEXT_BOX_WIDTH)
        .fix_height(TEXT_BOX_HEIGHT)
        .lens(HelloState::input_text);

    // Create a TextBox for output
    let output_textbox = TextBox::multiline()
        .with_placeholder("Output will appear here...")
        .fix_width(TEXT_BOX_WIDTH)
        .fix_height(TEXT_BOX_HEIGHT)
        .lens(HelloState::output_text);

    // Create a Checkbox for file saving
    let save_checkbox = Checkbox::new("Save as HTML").lens(HelloState::save_as_html);

    // Create a Checkbox for copying to clipboard
    let copy_checkbox = Checkbox::new("Copy to clipboard").lens(HelloState::copy_to_clipboard);

    let checkbox_container = Flex::row()
        .with_child(save_checkbox)
        .with_spacer(10.0)
        .with_child(copy_checkbox);

    let submit_button = Button::from_label(Label::new("Submit").with_text_size(CUSTOM_FONT_SIZE))
        .on_click(|_ctx, data: &mut HelloState, _env| {
            data.output_text = convert_markdown_to_html(&data.input_text);
            if data.save_as_html { // if the save checkbox is checked, save the output to a file
                save_to_file(&data.output_text);
            }
            if data.copy_to_clipboard { // if the copy checkbox is checked, copy the output to the clipboard
                copy_to_clipboard(&data.output_text);
            }
        })
        .fix_width(150.0)
        .fix_height(75.0);

    let input_label = Label::new("Input").with_text_size(CUSTOM_FONT_SIZE);

    let output_label = Label::new("Output").with_text_size(CUSTOM_FONT_SIZE);

    let input_text_box = Flex::column()
        .with_child(input_label)
        .with_spacer(20.0)
        .with_child(input_textbox);

    let output_text_box = Flex::column()
        .with_child(output_label)
        .with_spacer(20.0)
        .with_child(output_textbox);

    let text_box_container = Flex::row()
        .with_child(input_text_box)
        .with_spacer(10.0)
        .with_child(output_text_box);

    // Arrange the widgets vertically in a column
    let layout = Flex::column()
        .with_spacer(35.0)
        .with_child(text_box_container)
        .with_spacer(30.0)
        .with_child(submit_button)
        .with_spacer(10.0)
        .with_child(checkbox_container);

    layout
}
