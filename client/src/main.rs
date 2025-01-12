use std::{path::PathBuf, str::FromStr};

use common::{Action, Unit, UnitKind};
use iced::{
    event,
    keyboard::{self, Modifiers},
    widget::{column, image, row, Button, Row, Text},
    Alignment::Center,
    Color, Element, Event, Subscription, Task,
};

mod action;

fn main() -> iced::Result {
    iced::application("eltahawy's locker", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    units: Vec<Unit>,
    selected: Vec<Unit>,
    shift: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Action(Action),
    Ls(Option<Vec<Unit>>),
    Select(Unit),
    UnSelect(Unit),
    EventOccurred(Event),
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Action(action) => match action {
                Action::Ls(path_buf) => {
                    Task::perform(action::ls(path_buf.clone()), |x| Message::Ls(x.ok()))
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
            Message::Ls(units) => {
                if let Some(mut units) = units {
                    units.sort_by_key(|x| (x.kind.clone(), x.name()));
                    self.units = units;
                }
                Task::none()
            }
            Message::EventOccurred(event) => {
                match event {
                    Event::Keyboard(event) => {
                        println!("{:#?}", event);
                        self.shift = event == keyboard::Event::ModifiersChanged(Modifiers::SHIFT);
                    }
                    Event::Mouse(event) => {
                        println!("{:#?}", event);
                    }
                    Event::Window(event) => {
                        println!("{:#?}", event);
                    }
                    Event::Touch(event) => {
                        println!("{:#?}", event);
                    }
                }
                Task::none()
            }
            Message::Select(unit) => {
                self.selected.push(unit);
                Task::none()
            }
            Message::UnSelect(unit) => {
                self.selected.retain(|x| *x != unit);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let ls = Button::new("ls home").on_press(Message::Action(Action::Ls(PathBuf::new())));
        let rm =
            Button::new("rm .bash_history").on_press(Message::Action(Action::Rm(vec![Unit {
                path: PathBuf::from_str(".bash_profile").unwrap(),
                kind: common::UnitKind::File,
            }])));
        let mv = Button::new("mv .bash_history Downloads").on_press(Message::Action(Action::Mv {
            from: vec![PathBuf::from_str(".bash_history").unwrap()],
            to: PathBuf::from_str("Downloads").unwrap(),
        }));
        let cp = Button::new("cp .bash_history Downloads").on_press(Message::Action(Action::Cp {
            from: vec![PathBuf::from_str(".bash_history").unwrap()],
            to: PathBuf::from_str("Downloads").unwrap(),
        }));
        let mp4 = Button::new("mp4 record.mkv").on_press(Message::Action(Action::Mp4(vec![
            PathBuf::from_str("record.mkv").unwrap(),
        ])));

        let nav_bar = row![ls, rm, mv, cp, mp4].spacing(5).wrap();

        let units = self
            .units
            .iter()
            .map(|x| UnitElement::new(x, self.selected.contains(x)))
            .fold(Row::new().spacing(10), |acc, x| acc.push(x.display()))
            .wrap();

        column![nav_bar, units].spacing(20).into()
    }
}

struct UnitElement {
    unit: Unit,
    selected: bool,
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

impl UnitElement {
    fn new(unit: &Unit, selected: bool) -> Self {
        Self {
            unit: unit.clone(),
            selected,
        }
    }
    fn display(&self) -> Button<'static, Message> {
        let path = match self.unit.kind {
            UnitKind::Dirctory => dark_icon!(self.selected, "directory.png"),
            UnitKind::Video => dark_icon!(self.selected, "video.png"),
            UnitKind::Audio => dark_icon!(self.selected, "audio.png"),
            UnitKind::File => dark_icon!(self.selected, "file.png"),
        };
        let icon = Element::from(image(path).width(40).height(40)).explain(Color::BLACK);
        let button = Button::new(column![icon, Text::new(self.unit.name())].align_x(Center))
            .on_press(if self.selected {
                Message::UnSelect(self.unit.clone())
            } else {
                Message::Select(self.unit.clone())
            });
        //TODO : on double click send the following message
        // .on_press_maybe(
        //     matches!(self.unit.kind, UnitKind::Dirctory)
        //         .then_some(Message::Action(Action::Ls(self.unit.path.clone()))),
        // );
        button
    }
}
