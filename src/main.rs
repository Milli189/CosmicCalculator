use cosmic::app::Settings;
use cosmic::iced::window::Settings as WindowSettings;
use cosmic::iced::{Alignment, Length};
use cosmic::iced::{Subscription, window};
use cosmic::prelude::*;
use cosmic::widget;
use cosmic::widget::{button, column, container, row, text};
use cosmic::{Application, Core, Element, Task};

#[derive(Default)]
struct Calcolatrice {
    core: Core,
    display: String,
}

#[derive(Debug, Clone)]
enum Message {
    PremutoNumero(u8),
    PremutaOperazione(String),
    Cancella,
    Calcola,
    TastoPremuto {
        key: cosmic::iced::keyboard::Key,
        text: Option<String>,
    },
    PremutoPunto,
    PremutoBackspace,
    ParentesiAperta,
    ParentesiChiusa,
}

impl Calcolatrice {
    fn valuta_token(token: &[&str]) -> Option<f64> {
        if token.is_empty() {
            return None;
        }

        let mut numeri: Vec<f64> = Vec::new();
        let mut operatori: Vec<&str> = Vec::new();
        let mut espressione_valida = true;
        let mut ci_aspettiamo_un_numero = true;

        let mut i = 0;
        while i < token.len() {
            let t = token[i];

            if ci_aspettiamo_un_numero {
                if t == "(" {
                    let mut livello_parentesi = 1;
                    let mut j = i + 1;

                    while j < token.len() {
                        if token[j] == "(" {
                            livello_parentesi += 1;
                        }
                        if token[j] == ")" {
                            livello_parentesi -= 1;
                        }
                        if livello_parentesi == 0 {
                            break;
                        }
                        j += 1;
                    }

                    if livello_parentesi == 0 {
                        if let Some(risultato_interno) = Self::valuta_token(&token[i + 1..j]) {
                            numeri.push(risultato_interno);
                            ci_aspettiamo_un_numero = false;
                            i = j + 1;
                            continue;
                        }
                    }
                    espressione_valida = false;
                    break;
                }
                if t == "-" {
                    if i + 1 < token.len() {
                        let prossimo_t = token[i + 1];
                        let numero_negativo_str = format!("-{}", prossimo_t);
                        if let Ok(num) = numero_negativo_str.parse::<f64>() {
                            numeri.push(num);
                            ci_aspettiamo_un_numero = false;
                            i += 2;
                            continue;
                        }
                    }
                    espressione_valida = false;
                    break;
                }
                if t == "+" {
                    if i + 1 < token.len() {
                        let prossimo_t = token[i + 1];
                        let numero_positivo_str = format!("+{}", prossimo_t);
                        if let Ok(num) = numero_positivo_str.parse::<f64>() {
                            numeri.push(num);
                            ci_aspettiamo_un_numero = false;
                            i += 2;
                            continue;
                        }
                    }
                    espressione_valida = false;
                    break;
                }

                if let Ok(num) = t.parse::<f64>() {
                    numeri.push(num);
                    ci_aspettiamo_un_numero = false;
                    i += 1;
                } else {
                    espressione_valida = false;
                    break;
                }
            } else {
                if t == "+" || t == "-" || t == "*" || t == "/" {
                    operatori.push(t);
                    ci_aspettiamo_un_numero = true;
                    i += 1;
                } else {
                    espressione_valida = false;
                    break;
                }
            }
        }

        if !espressione_valida || numeri.len() != operatori.len() + 1 {
            return None;
        }

        let mut idx = 0;
        while idx < operatori.len() {
            if operatori[idx] == "*" || operatori[idx] == "/" {
                let op = operatori.remove(idx);
                let num1 = numeri.remove(idx);
                let num2 = numeri.remove(idx);
                let parziale = if op == "*" {
                    num1 * num2
                } else {
                    if num2 == 0.0 { f64::NAN } else { num1 / num2 }
                };

                numeri.insert(idx, parziale);
            } else {
                idx += 1;
            }
        }

        let mut idx = 0;
        while idx < operatori.len() {
            let op = operatori.remove(idx);
            let num1 = numeri.remove(idx);
            let num2 = numeri.remove(idx);

            let parziale = if op == "+" { num1 + num2 } else { num1 - num2 };

            numeri.insert(idx, parziale);
        }

        numeri.first().cloned()
    }

    fn add_space(calc: &mut Calcolatrice) {
        if calc.display.is_empty() {
            calc.display.push_str(" ");
        } else if !calc.display.ends_with(' ') {
            calc.display.push(' ');
        }
    }

    fn update(calc: &mut Calcolatrice, message: Message) {
        match message {
            Message::PremutoNumero(n) => calc.display.push_str(&n.to_string()),
            Message::PremutaOperazione(op) => {
                Self::add_space(calc);
                calc.display.push_str(&format!("{}", op));
                Self::add_space(calc);
            }
            Message::Cancella => calc.display.clear(),
            Message::Calcola => {
                let token: Vec<&str> = calc.display.split_whitespace().collect();

                if !token.is_empty() {
                    if let Some(risultato_finale) = Self::valuta_token(&token) {
                        if risultato_finale.is_nan() {
                            calc.display = "Error".to_string();
                        } else {
                            calc.display = risultato_finale.to_string();
                        }
                    } else {
                        calc.display.clear();
                    }
                }
            }

            //keyboard input
            Message::TastoPremuto { key, text } => {
                match key {
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Enter,
                    ) => {
                        Calcolatrice::update(calc, Message::Calcola);
                        return;
                    }
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Backspace,
                    ) => {
                        Calcolatrice::update(calc, Message::PremutoBackspace);
                        return;
                    }
                    cosmic::iced::keyboard::Key::Named(
                        cosmic::iced::keyboard::key::Named::Escape,
                    ) => {
                        Calcolatrice::update(calc, Message::Cancella);
                        return;
                    }
                    _ => {}
                }

                if let Some(carattere) = text {
                    match carattere.as_str() {
                        "0" => Calcolatrice::update(calc, Message::PremutoNumero(0)),
                        "1" => Calcolatrice::update(calc, Message::PremutoNumero(1)),
                        "2" => Calcolatrice::update(calc, Message::PremutoNumero(2)),
                        "3" => Calcolatrice::update(calc, Message::PremutoNumero(3)),
                        "4" => Calcolatrice::update(calc, Message::PremutoNumero(4)),
                        "5" => Calcolatrice::update(calc, Message::PremutoNumero(5)),
                        "6" => Calcolatrice::update(calc, Message::PremutoNumero(6)),
                        "7" => Calcolatrice::update(calc, Message::PremutoNumero(7)),
                        "8" => Calcolatrice::update(calc, Message::PremutoNumero(8)),
                        "9" => Calcolatrice::update(calc, Message::PremutoNumero(9)),
                        "(" => Calcolatrice::update(calc, Message::ParentesiAperta),
                        ")" => Calcolatrice::update(calc, Message::ParentesiChiusa),
                        "." | "," => Calcolatrice::update(calc, Message::PremutoPunto),
                        "+" => {
                            Calcolatrice::update(calc, Message::PremutaOperazione("+".to_string()))
                        }
                        "-" => {
                            Calcolatrice::update(calc, Message::PremutaOperazione("-".to_string()))
                        }
                        "*" => {
                            Calcolatrice::update(calc, Message::PremutaOperazione("*".to_string()))
                        }
                        "/" => {
                            Calcolatrice::update(calc, Message::PremutaOperazione("/".to_string()))
                        }
                        "=" => Calcolatrice::update(calc, Message::Calcola),
                        _ => {}
                    }
                }
            }

            Message::PremutoPunto => {
                if let Some(ultimo_numero) = calc.display.split_whitespace().last() {
                    if ultimo_numero.contains('.') {
                        return;
                    }
                } else {
                    calc.display.push_str("0");
                }

                calc.display.push_str(".");
            }

            Message::PremutoBackspace => {
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

            Message::ParentesiAperta => {
                Self::add_space(calc);
                calc.display.push_str("(");
                Self::add_space(calc);
            }

            Message::ParentesiChiusa => {
                Self::add_space(calc);
                calc.display.push_str(")");
                Self::add_space(calc);
            }
        }
    }

    fn view(calc: &Calcolatrice) -> Element<'_, Message> {
        let testo_schermo = if calc.display.is_empty() {
            ""
        } else {
            &calc.display
        };
        let schermo = text(testo_schermo)
            .size(40)
            .width(Length::Fill)
            .align_x(Alignment::End);

        //to center the text inside of a button you need both .width and .height, and then .center
        let tasto = |etichetta: &'static str, messaggio: Message| {
            button::custom(
                text(etichetta)
                    .size(24)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(),
            )
            .on_press(messaggio)
            .width(Length::Fill)
            .height(Length::Fill)
        };

        let tasto2 = |etichetta: &'static str, messaggio: Message| {
            button::custom(
                text(etichetta)
                    .size(24)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center(),
            )
            .on_press(messaggio)
            .width(Length::Fill)
            .height(Length::Fill)
            .class(cosmic::theme::Button::Suggested)
        };

        //grid 4x5
        let riga1 = row![
            tasto("C", Message::Cancella),
            tasto("(", Message::ParentesiAperta),
            tasto(")", Message::ParentesiChiusa),
            tasto("⌫", Message::PremutoBackspace),
        ]
        .spacing(10)
        .height(Length::Fill);

        let riga2 = row![
            tasto("7", Message::PremutoNumero(7)),
            tasto("8", Message::PremutoNumero(8)),
            tasto("9", Message::PremutoNumero(9)),
            tasto("/", Message::PremutaOperazione("/".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let riga3 = row![
            tasto("4", Message::PremutoNumero(4)),
            tasto("5", Message::PremutoNumero(5)),
            tasto("6", Message::PremutoNumero(6)),
            tasto("*", Message::PremutaOperazione("*".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let riga4 = row![
            tasto("1", Message::PremutoNumero(1)),
            tasto("2", Message::PremutoNumero(2)),
            tasto("3", Message::PremutoNumero(3)),
            tasto("-", Message::PremutaOperazione("-".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let riga5 = row![
            tasto("0", Message::PremutoNumero(0)),
            tasto(".", Message::PremutoPunto),
            tasto2("=", Message::Calcola),
            tasto("+", Message::PremutaOperazione("+".to_string())),
        ]
        .spacing(10)
        .height(Length::Fill);

        let layout_calcolatrice = column![schermo, riga1, riga2, riga3, riga4, riga5,]
            .spacing(10)
            .padding(15)
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout_calcolatrice)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl cosmic::Application for Calcolatrice {
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
                let testo_standard = text.map(|s| s.to_string());
                Some(Message::TastoPremuto {
                    key,
                    text: testo_standard,
                })
            }
            _ => None,
        })
    }

    fn init(core: Core, _flags: ()) -> (Self, Task<cosmic::Action<Message>>) {
        let mut app = Calcolatrice {
            core,
            display: String::new(),
        };

        //window title
        let command = app.set_window_title("Cosmic Calculator".to_string(), window::Id::RESERVED);

        (app, command)
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        Calcolatrice::update(self, message);
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        Calcolatrice::view(self)
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

    cosmic::app::run::<Calcolatrice>(settings, ())
}
