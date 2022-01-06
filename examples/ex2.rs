use clap::Parser;
use gdk::keys::constants as GtkKeys;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Builder, Inhibit, TextBufferBuilder, TextView, Window};
use omega::{Session, ViewportPtr};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

const VPSIZE: i64 = 500;

#[derive(Parser)]
pub struct Args {
    /// Input file
    #[clap(default_value = "/tmp/foo.txt")]
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
    v: Option<ViewportPtr>,
    l: i64,
}

impl State {
    pub fn new(s: Session) -> Self {
        Self { s, v: None, l: 0 }
    }

    pub fn up(&mut self, v: i64) -> i64 {
        if self.l > v {
            self.l -= v;
            self.l
        } else {
            0
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
                    self.state.v.as_ref().unwrap().update(o, VPSIZE);
                }
                GtkKeys::Down => {
                    let o = self.state.down(1);
                    self.state.v.as_ref().unwrap().update(o, VPSIZE);
                }
                GtkKeys::Page_Up => {
                    let o = self.state.up(50);
                    self.state.v.as_ref().unwrap().update(o, VPSIZE);
                }
                GtkKeys::Page_Down => {
                    let o = self.state.down(50);
                    self.state.v.as_ref().unwrap().update(o, VPSIZE);
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
        let v = self.gui.text_display.clone();
        let vp = self.state.s.view_cb(
            0,
            VPSIZE,
            Box::new(move |vp| {
                (*v).set_buffer(Some(
                    &TextBufferBuilder::new().text(&vp.to_string()).build(),
                ))
            }),
        );
        self.state.v = Some(vp.clone());
        let v = self.gui.text_display.clone();
        (*v).set_buffer(Some(
            &TextBufferBuilder::new().text(&vp.to_string()).build(),
        ));
    }

    fn root(&self) -> Self::Root {
        self.gui.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("../glade/ex2.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        let text_display: Box<TextView> = Box::new(builder.object("text_display").unwrap());
        text_display.set_buffer(Some(
            &TextBufferBuilder::new()
                .text("Open a file to start")
                .build(),
        ));

        connect!(
            relm,
            main_window,
            connect_key_press_event(_, e),
            return (Msg::InputEvent(e.clone()), Inhibit(false))
        );

        main_window.show_all();

        App {
            state,
            gui: Widgets {
                text_display,
                main_window,
            },
        }
    }
}

struct Widgets {
    text_display: Box<TextView>,
    main_window: Window,
}
