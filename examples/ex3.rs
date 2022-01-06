use clap::Parser;
use gdk::keys::constants as GtkKeys;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Builder, Inhibit, TextBufferBuilder, TextView, Window};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use omega::{Session, ViewportPtr};

const HEADER_SIZE: i64 = 100;
const BODY_SIZE: i64 = 500;

#[derive(Parser)]
pub struct Args {
    /// Input file
    #[clap(default_value = "data/ex3.txt")]
    pub path: String,
}

fn main() {
    let opts = Args::parse();
    let s = Session::from_file(&opts.path);
    App::run((s, ())).expect("App::run failed");
}

#[derive(Msg)]
pub enum Msg {
    InputEvent(EventKey),
    Quit,
}

pub struct State {
    s: Session,
    h: Option<ViewportPtr>,
    b: Option<ViewportPtr>,
    l: i64,
}

impl State {
    pub fn new(s: Session) -> Self {
        Self {
            s,
            h: None,
            b: None,
            l: HEADER_SIZE,
        }
    }

    pub fn up(&mut self, v: i64) -> i64 {
        if self.l - v > HEADER_SIZE {
            self.l -= v;
            self.l
        } else {
            HEADER_SIZE
        }
    }
    pub fn down(&mut self, v: i64) -> i64 {
        self.l += v;
        self.l
    }
}

pub struct App {
    state: State,
    gui: Widgets,
}

impl Update for App {
    type Model = State;
    type ModelParam = (Session, ());
    type Msg = Msg;

    fn model(_: &Relm<Self>, (p, _): Self::ModelParam) -> State {
        State::new(p)
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::InputEvent(e) => match e.keyval() {
                GtkKeys::Up => {
                    let o = self.state.up(1);
                    self.state.b.as_ref().unwrap().update(o, BODY_SIZE);
                }
                GtkKeys::Down => {
                    let o = self.state.down(1);
                    self.state.b.as_ref().unwrap().update(o, BODY_SIZE);
                }
                GtkKeys::Page_Up => {
                    let o = self.state.up(50);
                    self.state.b.as_ref().unwrap().update(o, BODY_SIZE);
                }
                GtkKeys::Page_Down => {
                    let o = self.state.down(50);
                    self.state.b.as_ref().unwrap().update(o, BODY_SIZE);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn init_view(&mut self) {
        self.state.h = Some(self.state.s.view(0, HEADER_SIZE));
        let body = self.gui.text_body.clone();
        let body = self.state.s.view_cb(
            HEADER_SIZE,
            BODY_SIZE,
            Box::new(move |vp| {
                (*body).set_buffer(Some(
                    &TextBufferBuilder::new().text(&vp.to_string()).build(),
                ))
            }),
        );
        self.state.b = Some(body.clone());
        let v = self.gui.text_body.clone();
        (*v).set_buffer(Some(
            &TextBufferBuilder::new().text(&body.to_string()).build(),
        ));

        self.gui
            .text_header
            .buffer()
            .unwrap()
            .set_text(&self.state.h.as_ref().unwrap().to_string())
    }

    fn root(&self) -> Self::Root {
        self.gui.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("../glade/ex3.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        let text_header: TextView = builder.object("header_view").unwrap();
        let text_body: Box<TextView> = Box::new(builder.object("body_view").unwrap());

        connect!(
            relm,
            text_body,
            connect_key_press_event(_, e),
            return (Msg::InputEvent(e.clone()), Inhibit(false))
        );

        main_window.show_all();

        App {
            state,
            gui: Widgets {
                text_header,
                text_body,
                main_window,
            },
        }
    }
}

struct Widgets {
    text_header: TextView,
    text_body: Box<TextView>,
    main_window: Window,
}
