#![windows_subsystem = "windows"]

mod structs;
use structs::*;
mod initial_page;
use initial_page::*;

use im::vector::Vector;

use druid::{AppLauncher, Color, Data, Lens, WidgetId, WindowDesc, WindowState};
use druid::text::ParseFormatter;
use druid::widget::{prelude::*, DisabledIf};
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Flex, Label, MainAxisAlignment, ProgressBar, RadioGroup,
    SizedBox, Slider, Stepper, Switch, TextBox, WidgetExt,
};

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Model MVC       Данные и параметры   
#[derive(Clone, Data, Lens)]
pub struct AppState {
    demo_state: DemoState,
    params: Params,
}

// Структура с множеством вложенностей, данные приложения
#[derive(Clone, Data, Lens)]
pub struct DemoState {
    pub input_text: String,
    pub input_text2: String,
    pub enabled: bool,
    pub enabled2: bool,
}

// Структура с множеством вложенностей, параметры приложения
#[derive(Clone, Data, Lens)]
pub struct Params {
    show_all: bool,
    padding: f64,
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Controller MVC       можно не менять, универсально
struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

// Controller MVC       можно не менять, универсально
impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data.params);
    }
}

// как будет Controller MVC влиять на Model MVC            то же можно не менять? 
impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        if !old_data.params.same(&data.params) {
            self.rebuild_inner(data);
            ctx.children_changed();
        } else {
            self.inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// View MVC
fn build_widget(state: &Params) -> Box<dyn Widget<AppState>> {
    let mut flex = Flex::row();
    
    flex.add_child(initial_page());
    flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Overridable text")
            .lens(DemoState::input_text), |data: &DemoState, _env| !data.enabled)
    );
        
    flex.add_child(
        Button::new("Clear").on_click(|_ctx, data: &mut DemoState, _env| {
            data.input_text.clear();
            data.enabled = false;
        }),
    );
    flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.input_text.clone()).with_text_size(32.0),
    );
    
    flex.add_child(Checkbox::new("Demo2").lens(DemoState::enabled2));
    flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Overridable text2")
            .lens(DemoState::input_text2), |data: &DemoState, _env| !data.enabled2)
    );
    flex.add_child(
        Button::new("Clear2").on_click(|_ctx, data: &mut DemoState, _env| {
            data.input_text2.clear();
            data.enabled2 = false;
        }),
    );
    flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.input_text2.clone()).with_text_size(32.0),
    );

    let flex = flex
        .padding(8.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(AppState::demo_state);
    
    flex.boxed()
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// View MVC
fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(Rebuilder::new().center(), 1.0)
        .padding(10.0)
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let main_window = WindowDesc::new(make_ui())
        .set_window_state(WindowState::Restored)
        .with_min_size((620., 300.))
        .title("AppName");

    let demo_state = DemoState {
        input_text: "hello".into(),
        input_text2: "world".into(),
        enabled: false,
        enabled2: false,
    };

    let params = Params {
        show_all: true,
        padding: 0.5,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState { demo_state, params })
        .expect("Failed to launch application");
}