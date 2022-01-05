use gtk::prelude::{BuilderExtManual, TextBufferExt, TextViewExt, WidgetExt};
use gtk::{Builder, Inhibit, TextBufferBuilder, TextView, Window};
use omega_sys::Session;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

fn main() {
    let s = Session::new();

    App::run((s, ())).expect("App::run failed");
}

#[derive(Msg)]
pub enum Msg {
    InputEvent,
    Quit,
}

pub struct State {
    s: Session,
}

impl State {
    pub fn new(s: Session) -> Self {
        Self { s }
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
            Msg::InputEvent => {
                let buf = self.gui.input_widget.buffer().unwrap();
                let txt = buf.text(&buf.start_iter(), &buf.end_iter(), false);
                let txt = txt.unwrap();
                println!("{}", txt.as_str());
                self.state.s.delete(0, 1000);
                self.state.s.insert(txt.as_str(), 0);
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn init_view(&mut self) {
        self.gui
            .output_widgets
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                let i = Box::new(i as i64);
                let y = v.clone();
                self.state.s.view_cb(
                    *i.clone() * 3,
                    3,
                    Box::new(move |vp| {
                        let x = &y;
                        (*x).set_buffer(Some(
                            &TextBufferBuilder::new().text(&vp.to_string()).build(),
                        ))
                    }),
                );
            });
    }

    fn root(&self) -> Self::Root {
        self.gui.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("resources/ed.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        connect!(
            relm,
            main_window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        let input_widget: TextView = builder.object("input_widget").unwrap();
        input_widget.set_buffer(Some(
            &TextBufferBuilder::new().text("Start typing here").build(),
        ));

        // connect to input
        connect!(
            relm,
            input_widget,
            connect_key_press_event(_, e),
            return (Some(Msg::InputEvent), Inhibit(false))
        );

        let output_widgets = (0..8)
            .map(|i| builder.object(&format!("text_{}_widget", i)))
            .flatten()
            .map(Box::new)
            .collect();

        main_window.show_all();

        App {
            state,
            gui: Widgets {
                input_widget,
                output_widgets,
                main_window,
            },
        }
    }
}

struct Widgets {
    input_widget: TextView,
    output_widgets: Vec<Box<TextView>>,
    main_window: Window,
}
