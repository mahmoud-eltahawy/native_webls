use std::{path::PathBuf, str::FromStr, sync::Arc};

use common::{Action, Unit, UnitKind};
use iced::{
    Alignment::Center,
    Color, Element, Event, Subscription, Task, event,
    keyboard::{self, Key, Modifiers, key::Named},
    widget::{Button, Row, Text, column, image, row},
    window,
};

mod action;

fn main() -> iced::Result {
    iced::application("eltahawy's locker", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Default)]
struct App {
    units: Box<[Arc<Unit>]>,
    selected: Vec<Arc<Unit>>,
    select_mode: bool,
}

#[derive(Debug, Clone)]
enum Message {
    RemoteAction(Action),
    Order(Order),
    LsValue(Vec<Unit>),
    EventOccurred(Event),
}
#[derive(Debug, Clone)]
enum Order {
    Select(Arc<Unit>),
    UnSelect(Arc<Unit>),
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RemoteAction(action) => match action {
                Action::Ls(path_buf) => {
                    self.selected.clear();
                    Task::perform(action::ls(path_buf), |x| {
                        Message::LsValue(x.unwrap_or_default())
                    })
                }
                Action::Rm(vec) => {
                    println!("removing {:#?}", vec);
                    Task::none()
                }
                Action::Mv { from, to } => {
                    println!("moving from {:#?} to {:#?}", from, to);
                    Task::none()
                }
                Action::Cp { from, to } => {
                    println!("copy from {:#?} to {:#?}", from, to);
                    Task::none()
                }
                Action::Mp4(vec) => {
                    println!("remuxing {:#?}", vec);
                    Task::none()
                }
            },
            Message::LsValue(mut units) => {
                units.sort_by_key(|x| (x.kind.clone(), x.name()));
                self.units = units.into_iter().map(Arc::new).collect::<Vec<_>>().into();
                Task::none()
            }
            Message::EventOccurred(event) => match event {
                Event::Keyboard(event) => {
                    match event {
                        keyboard::Event::ModifiersChanged(Modifiers::SHIFT) => {
                            self.select_mode = true;
                        }
                        keyboard::Event::KeyReleased {
                            key: Key::Named(Named::Shift),
                            ..
                        } => {
                            self.select_mode = false;
                        }
                        _ => (),
                    };
                    println!("{:#?}", event);
                    Task::none()
                }
                Event::Mouse(event) => {
                    println!("{:#?}", event);
                    Task::none()
                }
                Event::Window(event) => {
                    println!("{:#?}", event);
                    match event {
                        window::Event::Opened { .. } => {
                            Task::perform(action::ls(PathBuf::new()), |x| {
                                Message::LsValue(x.unwrap_or_default())
                            })
                        }
                        _ => Task::none(),
                    }
                }
                Event::Touch(event) => {
                    println!("{:#?}", event);
                    Task::none()
                }
            },
            Message::Order(Order::Select(unit)) => {
                self.selected.push(unit);
                Task::none()
            }
            Message::Order(Order::UnSelect(unit)) => {
                self.selected.retain(|x| *x != unit);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let ls = Button::new("Home").on_press(Message::RemoteAction(Action::Ls(PathBuf::new())));
        let rm = Button::new("rm .bash_history").on_press(Message::RemoteAction(Action::Rm(vec![
            Unit {
                path: PathBuf::from_str(".bash_profile").unwrap(),
                kind: common::UnitKind::File,
            },
        ])));
        let mv =
            Button::new("mv .bash_history Downloads").on_press(Message::RemoteAction(Action::Mv {
                from: vec![PathBuf::from_str(".bash_history").unwrap()],
                to: PathBuf::from_str("Downloads").unwrap(),
            }));
        let cp =
            Button::new("cp .bash_history Downloads").on_press(Message::RemoteAction(Action::Cp {
                from: vec![PathBuf::from_str(".bash_history").unwrap()],
                to: PathBuf::from_str("Downloads").unwrap(),
            }));
        let mp4 = Button::new("mp4 record.mkv").on_press(Message::RemoteAction(Action::Mp4(vec![
            PathBuf::from_str("record.mkv").unwrap(),
        ])));

        let nav_bar = row![ls, rm, mv, cp, mp4].spacing(5).wrap();

        struct UnitElement {
            unit: Arc<Unit>,
            selected: bool,
            select_mode: bool,
        }

        macro_rules! dark_icon {
            ($is_it:expr,$name:literal) => {
                if $is_it {
                    concat!("../public/dark/", $name)
                } else {
                    concat!("../public/", $name)
                }
            };
        }

        let units = self
            .units
            .iter()
            .map(|x| UnitElement {
                unit: x.clone(),
                selected: self.selected.contains(x),
                select_mode: self.select_mode,
            })
            .fold(Row::new(), |row, e| {
                row.push({
                    let icon = Element::from(
                        image(match e.unit.kind {
                            UnitKind::Dirctory => dark_icon!(e.selected, "directory.png"),
                            UnitKind::Video => dark_icon!(e.selected, "video.png"),
                            UnitKind::Audio => dark_icon!(e.selected, "audio.png"),
                            UnitKind::File => dark_icon!(e.selected, "file.png"),
                        })
                        .width(40)
                        .height(40),
                    )
                    .explain(Color::BLACK);
                    let title = Text::new(e.unit.name());
                    let block = column![icon, title].align_x(Center);
                    let on_press = if e.select_mode {
                        Some(if e.selected {
                            Message::Order(Order::UnSelect(e.unit.clone()))
                        } else {
                            Message::Order(Order::Select(e.unit.clone()))
                        })
                    } else if matches!(e.unit.kind, UnitKind::Dirctory) {
                        Some(Message::RemoteAction(Action::Ls(e.unit.path.clone())))
                    } else {
                        None
                    };
                    Button::new(block).on_press_maybe(on_press)
                })
            })
            .spacing(10)
            .wrap();

        column![nav_bar, units].spacing(20).into()
    }
}
