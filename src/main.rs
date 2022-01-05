use gtk::gdk::gdk_pixbuf::Colorspace;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::Bytes;
use gtk::prelude::{BuilderExtManual, ImageExt, TextViewExt, WidgetExt};
use gtk::{Builder, Inhibit, TextBufferBuilder, TextView, Window};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

fn main() {
    App::run(()).expect("App::run failed");
}

#[derive(Msg)]
pub enum Msg {
    InputEvent,
    Quit,
}

pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct App {
    state: State,
    gui: Widgets,
}

impl Update for App {
    type Model = State;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, param: Self::ModelParam) -> State {
        State::new()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::InputEvent => println!("input!"),
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn init_view(&mut self) {}

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
        connect!(
            relm,
            main_window,
            connect_key_press_event(_, e),
            return (Some(Msg::InputEvent), Inhibit(false))
        );

        let input_widget: TextView = builder.object("input_widget").unwrap();

        let views: Vec<TextView> = (0..8)
            .map(|i| builder.object(&format!("text_{}_widget", i)))
            .flatten()
            .collect();

        input_widget.set_buffer(Some(
            &TextBufferBuilder::new().text("Start typing here").build(),
        ));

        main_window.show_all();

        App {
            state,
            gui: Widgets {
                input_widget,
                main_window,
            },
        }
    }
}

struct Widgets {
    input_widget: TextView,
    main_window: Window,
}
