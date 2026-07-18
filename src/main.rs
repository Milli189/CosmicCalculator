use cosmic::app::Settings;
use cosmic::iced::window::Settings as WindowSettings;
use cosmic::iced::{Alignment, Length};
use cosmic::iced::{Subscription, window};
use cosmic::prelude::*;
use cosmic::widget;
use cosmic::widget::{button, column, container, row, text};
use cosmic::{Application, Core, Element, Task};

#[derive(Default)]
struct Calculator {
    core: Core,
    display: String,
}

#[derive(Debug, Clone)]
enum Message {
    PressedNumber(u8),
    PressedOperation(String),
    Clear,
    Calculate,
    PressedKey {
        key: cosmic::iced::keyboard::Key,
        text: Option<String>,
    },
    PressedDot,
    PressedBackspace,
    OpenParentheses,
    ClosedParentheses,
}

impl Calculator {
    fn token_value(token: &[&str]) -> Option<f64> {
        if token.is_empty() {
            return None;
        }

        let mut numbers: Vec<f64> = Vec::new();
        let mut operators: Vec<&str> = Vec::new();
        let mut valid_expression = true;
        let mut we_need_a_number = true;

        let mut i = 0;
        while i < token.len() {
            let t = token[i];

            if we_need_a_number {
                if t == "(" {
                    let mut parentheses_level = 1;
                    let mut j = i + 1;

                    while j < token.len() {
                        if token[j] == "(" {
                            parentheses_level += 1;
                        }
                        if token[j] == ")" {
                            parentheses_level -= 1;
                        }
                        if parentheses_level == 0 {
                            break;
                        }
                        j += 1;
                    }

                    if parentheses_level == 0 {
                        if let Some(internal_result) = Self::token_value(&token[i + 1..j]) {
                            numbers.push(internal_result);
                            we_need_a_number = false;
                            i = j + 1;
                            continue;
                        }
                    }
                    valid_expression = false;
                    break;
                }
                if t == "-" {
                    if i + 1 < token.len() {
                        let next_t = token[i + 1];
                        let negative_number_str = format!("-{}", next_t);
                        if let Ok(num) = negative_number_str.parse::<f64>() {
                            numbers.push(num);
                            we_need_a_number = false;
                            i += 2;
                            continue;
                        }
                    }
                    valid_expression = false;
                    break;
                }
                if t == "+" {
                    if i + 1 < token.len() {
                        let next_t = token[i + 1];
                        let positive_number_str = format!("+{}", next_t);
                        if let Ok(num) = positive_number_str.parse::<f64>() {
                            numbers.push(num);
                            we_need_a_number = false;
                            i += 2;
                            continue;
                        }
                    }
                    valid_expression = false;
                    break;
                }

                if let Ok(num) = t.parse::<f64>() {
                    numbers.push(num);
                    we_need_a_number = false;
                    i += 1;
                } else {
                    valid_expression = false;
                    break;
                }
            } else {
                if t == "+" || t == "-" || t == "*" || t == "/" {
                    operators.push(t);
                    we_need_a_number = true;
                    i += 1;
                } else {
                    valid_expression = false;
                    break;
                }
            }
        }

        if !valid_expression || numbers.len() != operators.len() + 1 {
            return None;
        }

        let mut idx = 0;
        while idx < operators.len() {
            if operators[idx] == "*" || operators[idx] == "/" {
                let op = operators.remove(idx);
                let num1 = numbers.remove(idx);
                let num2 = numbers.remove(idx);
                let partial = if op == "*" {
                    num1 * num2
                } else {
                    if num2 == 0.0 { f64::NAN } else { num1 / num2 }
                };

                numbers.insert(idx, partial);
            } else {
                idx += 1;
            }
        }

        let mut idx = 0;
        while idx < operators.len() {
            let op = operators.remove(idx);
            let num1 = numbers.remove(idx);
            let num2 = numbers.remove(idx);

            let partial = if op == "+" { num1 + num2 } else { num1 - num2 };

            numbers.insert(idx, partial);
        }

        numbers.first().cloned()
    }

    fn add_space(calc: &mut Calculator) {
        if calc.display.is_empty() {
            calc.display.push_str(" ");
        } else if !calc.display.ends_with(' ') {
            calc.display.push(' ');
        }
    }

    fn update(calc: &mut Calculator, message: Message) {
        match message {
            Message::PressedNumber(n) => calc.display.push_str(&n.to_string()),
            Message::PressedOperation(op) => {
                Self::add_space(calc);
                calc.display.push_str(&format!("{}", op));
                Self::add_space(calc);
            }
            Message::Clear => calc.display.clear(),
            Message::Calculate => {
                let token: Vec<&str> = calc.display.split_whitespace().collect();

                if !token.is_empty() {
                    if let Some(final_result) = Self::token_value(&token) {
                        if final_result.is_nan() {
                            calc.display = "Error".to_string();
                        } else {
                            calc.display = final_result.to_string();
                        }
                    } else {
                        calc.display.clear();
                    }
                }
            }

            //keyboard input
            Message::PressedKey { key, text } => {
                match key {
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Enter,
                    ) => {
                        Calculator::update(calc, Message::Calculate);
                        return;
                    }
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Backspace,
                    ) => {
                        Calculator::update(calc, Message::PressedBackspace);
                        return;
                    }
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Escape,
                    ) => {
                        Calculator::update(calc, Message::Clear);
                        return;
                    }
                    _ => {}
                }

                if let Some(character) = text {
                    match character.as_str() {
                        "0" => Calculator::update(calc, Message::PressedNumber(0)),
                        "1" => Calculator::update(calc, Message::PressedNumber(1)),
                        "2" => Calculator::update(calc, Message::PressedNumber(2)),
                        "3" => Calculator::update(calc, Message::PressedNumber(3)),
                        "4" => Calculator::update(calc, Message::PressedNumber(4)),
                        "5" => Calculator::update(calc, Message::PressedNumber(5)),
                        "6" => Calculator::update(calc, Message::PressedNumber(6)),
                        "7" => Calculator::update(calc, Message::PressedNumber(7)),
                        "8" => Calculator::update(calc, Message::PressedNumber(8)),
                        "9" => Calculator::update(calc, Message::PressedNumber(9)),
                        "(" => Calculator::update(calc, Message::OpenParentheses),
                        ")" => Calculator::update(calc, Message::ClosedParentheses),
                        "." | "," => Calculator::update(calc, Message::PressedDot),
                        "+" => Calculator::update(calc, Message::PressedOperation("+".to_string())),
                        "-" => Calculator::update(calc, Message::PressedOperation("-".to_string())),
                        "*" => Calculator::update(calc, Message::PressedOperation("*".to_string())),
                        "/" => Calculator::update(calc, Message::PressedOperation("/".to_string())),
                        "=" => Calculator::update(calc, Message::Calculate),
                        _ => {}
                    }
                }
            }

            Message::PressedDot => {
                if let Some(last_number) = calc.display.split_whitespace().last() {
                    if last_number.contains('.') {
                        return;
                    }
                } else {
                    calc.display.push_str("0");
                }

                calc.display.push_str(".");
            }

            Message::PressedBackspace => {
                if calc.display == "Error" {
                    calc.display.clear();
                } else if !calc.display.is_empty() && calc.display.ends_with(' ') {
                    calc.display.pop();
                    calc.display.pop();
                } else if !calc.display.is_empty() && calc.display.ends_with(char::is_numeric) {
                    calc.display.pop();
                } else if !calc.display.is_empty() && calc.display.ends_with(".") {
                    calc.display.pop();
                }
            }

            Message::OpenParentheses => {
                Self::add_space(calc);
                calc.display.push_str("(");
                Self::add_space(calc);
            }

            Message::ClosedParentheses => {
                Self::add_space(calc);
                calc.display.push_str(")");
                Self::add_space(calc);
            }
        }
    }

    fn view(calc: &Calculator) -> Element<'_, Message> {
        let display_text = if calc.display.is_empty() {
            ""
        } else {
            &calc.display
        };
        let screen = text(display_text)
            .size(40)
            .width(Length::Fill)
            .align_x(Alignment::End);

        //to center the text inside of a button you need both .width and .height, and then .center
        let button1 = |label: &'static str, message: Message| {
            button::custom(
                text(label)
                    .size(24)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(),
            )
            .on_press(message)
            .width(Length::Fill)
            .height(Length::Fill)
        };

        let button2 = |label: &'static str, message: Message| {
            button::custom(
                text(label)
                    .size(24)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(),
            )
            .on_press(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .class(cosmic::theme::Button::Suggested)
        };

        //grid 4x5
        let row1 = row![
            button1("C", Message::Clear),
            button1("(", Message::OpenParentheses),
            button1(")", Message::ClosedParentheses),
            button1("⌫", Message::PressedBackspace),
        ]
        .spacing(10)
        .height(Length::Fill);

        let row2 = row![
            button1("7", Message::PressedNumber(7)),
            button1("8", Message::PressedNumber(8)),
            button1("9", Message::PressedNumber(9)),
            button1("/", Message::PressedOperation("/".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let row3 = row![
            button1("4", Message::PressedNumber(4)),
            button1("5", Message::PressedNumber(5)),
            button1("6", Message::PressedNumber(6)),
            button1("*", Message::PressedOperation("*".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let row4 = row![
            button1("1", Message::PressedNumber(1)),
            button1("2", Message::PressedNumber(2)),
            button1("3", Message::PressedNumber(3)),
            button1("-", Message::PressedOperation("-".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let row5 = row![
            button1("0", Message::PressedNumber(0)),
            button1(".", Message::PressedDot),
            button2("=", Message::Calculate),
            button1("+", Message::PressedOperation("+".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let layout_calculator = column![screen, row1, row2, row3, row4, row5,]
            .spacing(10)
            .padding(15)
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout_calculator)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl cosmic::Application for Calculator {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.Milli189.CosmicCalculator";

    fn core(&self) -> &Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::keyboard::listen().filter_map(|evento| match evento {
            cosmic::iced::keyboard::Event::KeyPressed { key, text, .. } => {
                let standard_text = text.map(|s| s.to_string());
                Some(Message::PressedKey {
                    key,
                    text: standard_text,
                })
            }
            _ => None,
        })
    }

    fn init(core: Core, _flags: ()) -> (Self, Task<cosmic::Action<Message>>) {
        let mut app = Calculator {
            core,
            display: String::new(),
        };

        //window title
        let command = app.set_window_title("Cosmic Calculator".to_string(), window::Id::RESERVED);

        (app, command)
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        Calculator::update(self, message);
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        Calculator::view(self)
    }
}

fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(350.0)
            .min_height(500.0)
            .max_width(350.0)
            .max_height(500.0),
    );

    cosmic::app::run::<Calculator>(settings, ())
}
