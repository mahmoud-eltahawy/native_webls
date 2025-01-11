use std::{path::PathBuf, str::FromStr};

use common::{Action, Unit, UnitKind};
use iced::{
    widget::{column, image, row, Button, Row, Text},
    Color, Element, Task,
};

mod action;

fn main() -> iced::Result {
    iced::application("eltahawy's locker", App::update, App::view).run()
}

#[derive(Debug, Clone, Default)]
struct App {
    units: Vec<Unit>,
}

#[derive(Debug, Clone)]
enum Message {
    Action(Action),
    Ls(Option<Vec<Unit>>),
}

impl App {
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
        }
    }

    fn view(&self) -> Element<Message> {
        let ls = Button::new("ls home").on_press(Message::Action(Action::Ls(PathBuf::new())));
        let rm = Button::new("rm /home/eltahawy/.bash_history").on_press(Message::Action(
            Action::Rm(vec![Unit {
                path: PathBuf::from_str("/home/eltahawy/.bash_profile").unwrap(),
                kind: common::UnitKind::File,
            }]),
        ));
        let mv = Button::new("mv /home/eltahawy/.bash_history /home/eltahawy/Downloads").on_press(
            Message::Action(Action::Mv {
                from: vec![PathBuf::from_str("/home/eltahawy/.bash_history").unwrap()],
                to: PathBuf::from_str("/home/eltahawy/Downloads").unwrap(),
            }),
        );
        let cp = Button::new("cp /home/eltahawy/.bash_history /home/eltahawy/Downloads").on_press(
            Message::Action(Action::Cp {
                from: vec![PathBuf::from_str("/home/eltahawy/.bash_history").unwrap()],
                to: PathBuf::from_str("/home/eltahawy/Downloads").unwrap(),
            }),
        );
        let mp4 = Button::new("mp4 /home/eltahawy/record.mkv").on_press(Message::Action(
            Action::Mp4(vec![PathBuf::from_str("/home/eltahawy/record.mkv").unwrap()]),
        ));

        let units = self
            .units
            .iter()
            .fold(Row::new().spacing(10), |acc, x| acc.push(unit_element(x)));
        row![ls, rm, mv, cp, mp4, units.wrap()]
            .spacing(5)
            .wrap()
            .into()
    }
}

fn unit_element(unit: &Unit) -> Button<'static, Message> {
    let path = match unit.kind {
        UnitKind::Dirctory => "../public/directory.png",
        UnitKind::Video => "../public/video.png",
        UnitKind::Audio => "../public/audio.png",
        UnitKind::File => "../public/file.png",
    };
    let icon = Element::from(image(path).width(40).height(40)).explain(Color::BLACK);
    let button = Button::new(column![icon, Text::new(unit.name())]).on_press_maybe(
        matches!(unit.kind, UnitKind::Dirctory)
            .then_some(Message::Action(Action::Ls(unit.path.clone()))),
    );
    button
}
