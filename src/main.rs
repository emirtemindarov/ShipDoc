#![windows_subsystem = "windows"]

use druid::{AppLauncher, Color, Data, Lens, WidgetId, WindowDesc, WindowState, UnitPoint};
use druid::widget::{prelude::*, DisabledIf, Button, Checkbox, Flex, Label, SizedBox, TextBox, WidgetExt};

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
    pub routes_text: String,
    pub routes_text2: String,
    pub routes_text22: String,
    pub routes_text3: String,
    pub routes_text32: String,
    pub routes_text4: String,
    pub routes_text42: String,
    pub routes_text5: String,
    pub routes_text52: String,
    pub routes_text6: String,
    pub routes_text62: String,
    pub routes_text7: String,
    pub routes_text72: String,
    pub routes_text8: String,
    pub routes_text82: String,
    pub routes_enabled: bool,
    pub routes_enabled2: bool,
    pub routes_enabled22: bool,
    pub routes_enabled3: bool,

    pub container_text: String,
    pub container_text2: String,
    pub container_text22: String,
    pub container_text3: String,
    pub container_text32: String,
    pub container_text4: String,
    pub container_text42: String,
    pub container_enabled: bool,
    pub container_enabled2: bool,
    pub container_enabled22: bool,
    pub container_enabled3: bool,

    pub ship_text: String,
    pub ship_text2: String,
    pub ship_text22: String,
    pub ship_text3: String,
    pub ship_text32: String,
    pub ship_text4: String,
    pub ship_text42: String,
    pub ship_text5: String,
    pub ship_text52: String,
    pub ship_enabled: bool,
    pub ship_enabled2: bool,
    pub ship_enabled22: bool,
    pub ship_enabled3: bool,

    pub cargo_text: String,
    pub cargo_text2: String,
    pub cargo_text22: String,
    pub cargo_text3: String,
    pub cargo_text32: String,
    pub cargo_text4: String,
    pub cargo_text42: String,
    pub cargo_text5: String,
    pub cargo_text52: String,
    pub cargo_enabled: bool,
    pub cargo_enabled2: bool,
    pub cargo_enabled22: bool,
    pub cargo_enabled3: bool,

    pub client_text: String,
    pub client_text2: String,
    pub client_text22: String,
    pub client_text3: String,
    pub client_text32: String,
    pub client_text4: String,
    pub client_text42: String,
    pub client_text5: String,
    pub client_text52: String,
    pub client_text6: String,
    pub client_text62: String,
    pub client_enabled: bool,
    pub client_enabled2: bool,
    pub client_enabled22: bool,
    pub client_enabled3: bool,

    pub document_text: String,
    pub document_text2: String,
    pub document_text22: String,
    pub document_text3: String,
    pub document_text32: String,
    pub document_text4: String,
    pub document_text42: String,
    pub document_text5: String,
    pub document_text52: String,
    pub document_enabled: bool,
    pub document_enabled2: bool,
    pub document_enabled22: bool,
    pub document_enabled3: bool,
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

//-----------------------------------------------------------------

    


    let mut routes_flex = Flex::row();
    routes_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.routes_text2.clone()).with_text_size(16.0),   // New
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Место отправки").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text3), |data: &DemoState, _env| !data.routes_enabled2)
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Место прибытия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text4), |data: &DemoState, _env| !data.routes_enabled2)
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Дата отправки").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text5), |data: &DemoState, _env| !data.routes_enabled2)
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Дата прибытия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text6), |data: &DemoState, _env| !data.routes_enabled2)
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text7), |data: &DemoState, _env| !data.routes_enabled2)
    );
    routes_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Общий вес груза").with_text_color(Color::grey(0.6)).fix_width(200.0)
            .lens(DemoState::routes_text8), |data: &DemoState, _env| !data.routes_enabled2)
    );

    let mut routes_flex2 = Flex::row();
    routes_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.routes_text22.clone()).with_text_size(16.0),   // New
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Место отправки").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text32), |data: &DemoState, _env| !data.routes_enabled22)
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Место прибытия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text42), |data: &DemoState, _env| !data.routes_enabled22)
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Дата отправки").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text52), |data: &DemoState, _env| !data.routes_enabled22)
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Дата прибытия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text62), |data: &DemoState, _env| !data.routes_enabled22)
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::routes_text72), |data: &DemoState, _env| !data.routes_enabled22)
    );
    routes_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Общий вес груза").with_text_color(Color::grey(0.6)).fix_width(200.0)
            .lens(DemoState::routes_text82), |data: &DemoState, _env| !data.routes_enabled22)
    );
    
    let mut routes_top = Flex::row();
    routes_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.routes_text.clone()).with_text_size(24.0),
    );
    routes_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.routes_text2 == ">".to_string() && data.routes_text22 != ">".to_string() {
                data.routes_text22 = ">".into();
                data.routes_enabled22 = true;
                data.routes_text2 = "".into();
                data.routes_enabled2 = false;
            }
            if data.routes_text2 != ">".to_string() && data.routes_text22 != ">".to_string() { 
                data.routes_text2 = ">".into();
                data.routes_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut routes_list = Flex::column();
    routes_list.add_child(routes_top);
    routes_list.add_child(routes_flex);
    routes_list.add_child(routes_flex2);

    let mut routes_area = Flex::row();
    routes_area.add_child(Checkbox::new("").lens(DemoState::routes_enabled3));
    routes_area.add_child(DisabledIf::new(routes_list, |data: &DemoState, _env| !data.routes_enabled3));  
    



    let mut container_flex = Flex::row();
    container_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.container_text2.clone()).with_text_size(16.0),   // New
    );
    container_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вместимость контейнера").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::container_text3), |data: &DemoState, _env| !data.container_enabled2)
    );
    container_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Загрузка контейнера").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::container_text4), |data: &DemoState, _env| !data.container_enabled2)
    );
    

    let mut container_flex2 = Flex::row();
    container_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.container_text22.clone()).with_text_size(16.0),   // New
    );
    container_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вместимость контейнера").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::container_text32), |data: &DemoState, _env| !data.container_enabled22)
    );
    container_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Загрузка контейнера").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::container_text42), |data: &DemoState, _env| !data.container_enabled22)
    );
    
    let mut container_top = Flex::row();
    container_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.container_text.clone()).with_text_size(24.0),
    );
    container_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.container_text2 == ">".to_string() && data.container_text22 != ">".to_string() {
                data.container_text22 = ">".into();
                data.container_enabled22 = true;
                data.container_text2 = "".into();
                data.container_enabled2 = false;
            }
            if data.container_text2 != ">".to_string() && data.container_text22 != ">".to_string() { 
                data.container_text2 = ">".into();
                data.container_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut container_list = Flex::column();
    container_list.add_child(container_top);
    container_list.add_child(container_flex);
    container_list.add_child(container_flex2);

    let mut container_area = Flex::row();
    container_area.add_child(Checkbox::new("").lens(DemoState::container_enabled3));
    container_area.add_child(DisabledIf::new(container_list, |data: &DemoState, _env| !data.container_enabled3)); 


    
    let mut ship_flex = Flex::row();
    ship_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.ship_text2.clone()).with_text_size(16.0),   // New
    );
    ship_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text3), |data: &DemoState, _env| !data.ship_enabled2)
    );
    ship_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вместимость судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text4), |data: &DemoState, _env| !data.ship_enabled2)
    );
    ship_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Загрузка судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text5), |data: &DemoState, _env| !data.ship_enabled2)
    );

    let mut ship_flex2 = Flex::row();
    ship_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.ship_text22.clone()).with_text_size(16.0),   // New
    );
    ship_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text32), |data: &DemoState, _env| !data.ship_enabled22)
    );
    ship_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вместимость судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text42), |data: &DemoState, _env| !data.ship_enabled22)
    );
    ship_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Загрузка судна").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::ship_text52), |data: &DemoState, _env| !data.ship_enabled22)
    );
    
    let mut ship_top = Flex::row();
    ship_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.ship_text.clone()).with_text_size(24.0),
    );
    ship_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.ship_text2 == ">".to_string() && data.ship_text22 != ">".to_string() {
                data.ship_text22 = ">".into();
                data.ship_enabled22 = true;
                data.ship_text2 = "".into();
                data.ship_enabled2 = false;
            }
            if data.ship_text2 != ">".to_string() && data.ship_text22 != ">".to_string() { 
                data.ship_text2 = ">".into();
                data.ship_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut ship_list = Flex::column();
    ship_list.add_child(ship_top);
    ship_list.add_child(ship_flex);
    ship_list.add_child(ship_flex2);

    let mut ship_area = Flex::row();
    ship_area.add_child(Checkbox::new("").lens(DemoState::ship_enabled3));
    ship_area.add_child(DisabledIf::new(ship_list, |data: &DemoState, _env| !data.ship_enabled3)); 



    let mut cargo_flex = Flex::row();
    cargo_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.cargo_text2.clone()).with_text_size(16.0),   // New
    );
    cargo_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text3), |data: &DemoState, _env| !data.cargo_enabled2)
    );
    cargo_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вес груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text4), |data: &DemoState, _env| !data.cargo_enabled2)
    );
    cargo_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Количество груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text5), |data: &DemoState, _env| !data.cargo_enabled2)
    );
    
    let mut cargo_flex2 = Flex::row();
    cargo_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.cargo_text22.clone()).with_text_size(16.0),   // New
    );
    cargo_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text32), |data: &DemoState, _env| !data.cargo_enabled22)
    );
    cargo_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Вес груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text42), |data: &DemoState, _env| !data.cargo_enabled22)
    );
    cargo_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Количество груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::cargo_text52), |data: &DemoState, _env| !data.cargo_enabled22)
    );
    
    let mut cargo_top = Flex::row();
    cargo_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.cargo_text.clone()).with_text_size(24.0),
    );
    cargo_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.cargo_text2 == ">".to_string() && data.cargo_text22 != ">".to_string() {
                data.cargo_text22 = ">".into();
                data.cargo_enabled22 = true;
                data.cargo_text2 = "".into();
                data.cargo_enabled2 = false;
            }
            if data.cargo_text2 != ">".to_string() && data.cargo_text22 != ">".to_string() { 
                data.cargo_text2 = ">".into();
                data.cargo_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut cargo_list = Flex::column();
    cargo_list.add_child(cargo_top);
    cargo_list.add_child(cargo_flex);
    cargo_list.add_child(cargo_flex2);

    let mut cargo_area = Flex::row();
    cargo_area.add_child(Checkbox::new("").lens(DemoState::cargo_enabled3));
    cargo_area.add_child(DisabledIf::new(cargo_list, |data: &DemoState, _env| !data.cargo_enabled3)); 



    let mut client_flex = Flex::row();
    client_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.client_text2.clone()).with_text_size(16.0),   // New
    );
    client_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Имя").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text3), |data: &DemoState, _env| !data.client_enabled2)
    );
    client_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Фамилия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text4), |data: &DemoState, _env| !data.client_enabled2)
    );
    client_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Отчество").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text5), |data: &DemoState, _env| !data.client_enabled2)
    );
    client_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Плата за перевозку груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text6), |data: &DemoState, _env| !data.client_enabled2)
    );
    
    let mut client_flex2 = Flex::row();
    client_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.client_text22.clone()).with_text_size(16.0),   // New
    );
    client_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Имя").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text32), |data: &DemoState, _env| !data.client_enabled22)
    );
    client_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Фамилия").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text42), |data: &DemoState, _env| !data.client_enabled22)
    );
    client_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Отчество").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text52), |data: &DemoState, _env| !data.client_enabled22)
    );
    client_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Плата за перевозку груза").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::client_text62), |data: &DemoState, _env| !data.client_enabled22)
    );
    
    let mut client_top = Flex::row();
    client_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.client_text.clone()).with_text_size(24.0),
    );
    client_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.client_text2 == ">".to_string() && data.client_text22 != ">".to_string() {
                data.client_text22 = ">".into();
                data.client_enabled22 = true;
                data.client_text2 = "".into();
                data.client_enabled2 = false;
            }
            if data.client_text2 != ">".to_string() && data.client_text22 != ">".to_string() { 
                data.client_text2 = ">".into();
                data.client_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut client_list = Flex::column();
    client_list.add_child(client_top);
    client_list.add_child(client_flex);
    client_list.add_child(client_flex2);

    let mut client_area = Flex::row();
    client_area.add_child(Checkbox::new("").lens(DemoState::client_enabled3));
    client_area.add_child(DisabledIf::new(client_list, |data: &DemoState, _env| !data.client_enabled3)); 
    
    
    
    let mut document_flex = Flex::row();
    document_flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.document_text2.clone()).with_text_size(16.0),   // New
    );
    document_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип документа").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::document_text3), |data: &DemoState, _env| !data.document_enabled2)
    );
    document_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Заголовок документа").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::document_text4), |data: &DemoState, _env| !data.document_enabled2)
    );
    document_flex.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Ссылка на документ").with_text_color(Color::grey(0.6)).fix_width(840.0)
            .lens(DemoState::document_text5), |data: &DemoState, _env| !data.document_enabled2)
    );
    
    let mut document_flex2 = Flex::row();
    document_flex2.add_child(
        Label::new(|data: &DemoState, _: &Env| data.document_text22.clone()).with_text_size(16.0),   // New
    );
    document_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Тип документа").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::document_text32), |data: &DemoState, _env| !data.document_enabled22)
    );
    document_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Заголовок документа").with_text_color(Color::grey(0.6)).fix_width(240.0)
            .lens(DemoState::document_text42), |data: &DemoState, _env| !data.document_enabled22)
    );
    document_flex2.add_child(DisabledIf::new(
        TextBox::new()
            .with_placeholder("Ссылка на документ").with_text_color(Color::grey(0.6)).fix_width(840.0)
            .lens(DemoState::document_text52), |data: &DemoState, _env| !data.document_enabled22)
    );
    
    let mut document_top = Flex::row();
    document_top.add_child(
        Label::new(|data: &DemoState, _: &Env| data.document_text.clone()).with_text_size(24.0),
    );
    document_top.add_child(
        Button::new("+").on_click(|_ctx, data: &mut DemoState, _env| {
            if data.document_text2 == ">".to_string() && data.document_text22 != ">".to_string() {
                data.document_text22 = ">".into();
                data.document_enabled22 = true;
                data.document_text2 = "".into();
                data.document_enabled2 = false;
            }
            if data.document_text2 != ">".to_string() && data.document_text22 != ">".to_string() { 
                data.document_text2 = ">".into();
                data.document_enabled2 = true;
            }
        }).padding(5.0),
    );
    
    let mut document_list = Flex::column();
    document_list.add_child(document_top);
    document_list.add_child(document_flex);
    document_list.add_child(document_flex2);

    let mut document_area = Flex::row();
    document_area.add_child(Checkbox::new("").lens(DemoState::document_enabled3));
    document_area.add_child(DisabledIf::new(document_list, |data: &DemoState, _env| !data.document_enabled3)); 




//-----------------------------------------------------------------

    let mut final_area = Flex::column();
    final_area.add_child(routes_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));
    final_area.add_child(container_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));
    final_area.add_child(ship_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));
    final_area.add_child(cargo_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));
    final_area.add_child(client_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));
    final_area.add_child(document_area
                            .padding(16.0)
                            .border(Color::grey(0.6), 2.0)
                            .rounded(5.0));

    let flex = final_area
        .padding(8.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(AppState::demo_state);

    flex.boxed()
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// View MVC
fn make_ui() -> impl Widget<AppState> {
    Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(Rebuilder::new().align_vertical(UnitPoint::new(0.5, 0.5)), 1.0)
        .padding(10.0)
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let main_window = WindowDesc::new(make_ui())
        .set_window_state(WindowState::Maximized)
        .with_min_size((620., 300.))
        .title("Морские грузоперевозки");

    let demo_state = DemoState {
        routes_text: "Маршруты".into(),
        routes_text2: "".into(),
        routes_text22: "".into(),
        routes_text3: "".into(),
        routes_text32: "".into(),
        routes_text4: "".into(),
        routes_text42: "".into(),
        routes_text5: "".into(),
        routes_text52: "".into(),
        routes_text6: "".into(),
        routes_text62: "".into(),
        routes_text7: "".into(),
        routes_text72: "".into(),
        routes_text8: "".into(),
        routes_text82: "".into(),
        routes_enabled: false,
        routes_enabled2: false,
        routes_enabled22: false,
        routes_enabled3: true,

        container_text: "Контейнеры".into(),
        container_text2: "".into(),
        container_text22: "".into(),
        container_text3: "".into(),
        container_text32: "".into(),
        container_text4: "".into(),
        container_text42: "".into(),
        container_enabled: false,
        container_enabled2: false,
        container_enabled22: false,
        container_enabled3: true,

        ship_text: "Судна".into(),
        ship_text2: "".into(),
        ship_text22: "".into(),
        ship_text3: "".into(),
        ship_text32: "".into(),
        ship_text4: "".into(),
        ship_text42: "".into(),
        ship_text5: "".into(),
        ship_text52: "".into(),
        ship_enabled: false,
        ship_enabled2: false,
        ship_enabled22: false,
        ship_enabled3: true,

        cargo_text: "Грузы".into(),
        cargo_text2: "".into(),
        cargo_text22: "".into(),
        cargo_text3: "".into(),
        cargo_text32: "".into(),
        cargo_text4: "".into(),
        cargo_text42: "".into(),
        cargo_text5: "".into(),
        cargo_text52: "".into(),
        cargo_enabled: false,
        cargo_enabled2: false,
        cargo_enabled22: false,
        cargo_enabled3: true,

        client_text: "Клиенты".into(),
        client_text2: "".into(),
        client_text22: "".into(),
        client_text3: "".into(),
        client_text32: "".into(),
        client_text4: "".into(),
        client_text42: "".into(),
        client_text5: "".into(),
        client_text52: "".into(),
        client_text6: "".into(),
        client_text62: "".into(),
        client_enabled: false,
        client_enabled2: false,
        client_enabled22: false,
        client_enabled3: true,
        
        document_text: "Документы".into(),
        document_text2: "".into(),
        document_text22: "".into(),
        document_text3: "".into(),
        document_text32: "".into(),
        document_text4: "".into(),
        document_text42: "".into(),
        document_text5: "".into(),
        document_text52: "".into(),
        document_enabled: false,
        document_enabled2: false,
        document_enabled22: false,
        document_enabled3: true,
    };

    let params = Params {
        show_all: true,
        padding: 0.5,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState { demo_state, params })
        .expect("Не удалось запустить приложжение");
}