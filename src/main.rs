use druid::widget::{Button, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
extern crate pulldown_cmark;
use pulldown_cmark::{html, Options, Parser};

const CUSTOM_FONT_SIZE: f64 = 30.0;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// 16:9 aspect ratio
const TEXT_BOX_WIDTH: f64 = 800.0;
const TEXT_BOX_HEIGHT: f64 = 600.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Text Input and Output");

#[derive(Clone, Data, Lens)]
struct HelloState {
    input_text: String,
    output_text: String,
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((1600.0 + 50.0, 900.0)); //16:9 aspect ratio + 50px for some padding

    let initial_state = HelloState {
        input_text: String::new(),
        output_text: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
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

    let submit_button = Button::from_label(Label::new("Submit").with_text_size(CUSTOM_FONT_SIZE))
        .on_click(|_ctx, data: &mut HelloState, _env| {
            data.output_text = convert_markdown_to_html(&data.input_text);
        })
        .fix_width(150.0)
        .fix_height(75.0);

    let input_label = Label::new("Input").with_text_size(CUSTOM_FONT_SIZE);

    let output_label = Label::new("Output").with_text_size(CUSTOM_FONT_SIZE);

    let input_text_box = Flex::column()
        .with_child(input_label)
        .with_spacer(10.0)
        .with_child(input_textbox);

    let output_text_box = Flex::column()
        .with_child(output_label)
        .with_spacer(10.0)
        .with_child(output_textbox);

    let text_box_container = Flex::row()
        .with_child(input_text_box)
        .with_spacer(10.0)
        .with_child(output_text_box);

    // Arrange the widgets vertically in a column
    let layout = Flex::column()
        .with_spacer(75.0)
        .with_child(text_box_container)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(submit_button);

    layout
}
