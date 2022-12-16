// Model MVC       Данные и параметры   
#[derive(Clone, Data, Lens)]
struct AppState {
    demo_state: DemoState,
    params: Params,
}

// Структура с множеством вложенностей, данные приложения
#[derive(Clone, Data, Lens)]
struct DemoState {
    pub input_text: String,
    pub enabled: bool,
    volume: f64,
}

// Структура с множеством вложенностей, параметры приложения
#[derive(Clone, Data, Lens)]
struct Params {
    axis: FlexType,
    cross_alignment: CrossAxisAlignment,
    main_alignment: MainAxisAlignment,
    fill_major_axis: bool,
    debug_layout: bool,
    fix_minor_axis: bool,
    fix_major_axis: bool,
    spacers: Spacers,
    spacer_size: f64,
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// Controller MVC       можно не менять Pog , универсально
struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

// Controller MVC       можно не менять Pog , универсально
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

// как будет Controller MVC влиять на Model MVC 
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
fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(make_control_row())
        .with_default_spacer()
        .with_flex_child(Rebuilder::new().center(), 1.0)
        .padding(10.0)
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

// типо Controller MVC, но на самом деле View MVC             видимый контроллер?    не обязательно делать отдельную панельку, можно в View MVC ставить кнопки, которые управляют определенными вещами
fn make_control_row() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new("Type:"))
                .with_default_spacer()
                .with_child(RadioGroup::column(FLEX_TYPE_OPTIONS.to_vec()).lens(Params::axis)),
        )
        .with_default_spacer()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new("CrossAxis:"))
                .with_default_spacer()
                .with_child(
                    RadioGroup::column(CROSS_AXIS_ALIGNMENT_OPTIONS.to_vec())
                        .lens(Params::cross_alignment),
                ),
        )
        .with_default_spacer()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new("MainAxis:"))
                .with_default_spacer()
                .with_child(
                    RadioGroup::column(MAIN_AXIS_ALIGNMENT_OPTIONS.to_vec())
                        .lens(Params::main_alignment),
                ),
        )
        .with_default_spacer()
        .with_child(make_spacer_select())
        .with_default_spacer()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new("Misc:"))
                .with_default_spacer()
                .with_child(Checkbox::new("Debug layout").lens(Params::debug_layout))
                .with_default_spacer()
                .with_child(Checkbox::new("Fill main axis").lens(Params::fill_major_axis))
                .with_default_spacer()
                .with_child(Checkbox::new("Fix minor axis size").lens(Params::fix_minor_axis))
                .with_default_spacer()
                .with_child(Checkbox::new("Fix major axis size").lens(Params::fix_major_axis)),
        )
        .padding(10.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(AppState::params)
}

// View MVC    
fn build_widget(state: &Params) -> Box<dyn Widget<AppState>> {
    let mut flex = match state.axis {
        FlexType::Column => Flex::column(),
        FlexType::Row => Flex::row(),
    }
    .cross_axis_alignment(state.cross_alignment)
    .main_axis_alignment(state.main_alignment)
    .must_fill_main_axis(state.fill_major_axis);

    flex.add_child(
        TextBox::new()
            .with_placeholder("Sample text")
            .lens(DemoState::input_text),
    );
    space_if_needed(&mut flex, state);

    flex.add_child(
        Button::new("Clear").on_click(|_ctx, data: &mut DemoState, _env| {
            data.input_text.clear();
            data.enabled = false;
            data.volume = 0.0;
        }),
    );

    space_if_needed(&mut flex, state);

    flex.add_child(
        Label::new(|data: &DemoState, _: &Env| data.input_text.clone()).with_text_size(32.0),
    );
    space_if_needed(&mut flex, state);
    flex.add_child(Checkbox::new("Demo").lens(DemoState::enabled));
    space_if_needed(&mut flex, state);
    flex.add_child(Switch::new().lens(DemoState::enabled));
    space_if_needed(&mut flex, state);
    flex.add_child(Slider::new().lens(DemoState::volume));
    space_if_needed(&mut flex, state);
    flex.add_child(ProgressBar::new().lens(DemoState::volume));
    space_if_needed(&mut flex, state);
    flex.add_child(
        Stepper::new()
            .with_range(0.0, 1.0)
            .with_step(0.1)
            .with_wraparound(true)
            .lens(DemoState::volume),
    );

    let mut flex = SizedBox::new(flex);
    if state.fix_minor_axis {
        match state.axis {
            FlexType::Row => flex = flex.height(200.),
            FlexType::Column => flex = flex.width(200.),
        }
    }
    if state.fix_major_axis {
        match state.axis {
            FlexType::Row => flex = flex.width(600.),
            FlexType::Column => flex = flex.height(300.),
        }
    }

    let flex = flex
        .padding(8.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(AppState::demo_state);

    if state.debug_layout {
        flex.debug_paint_layout().boxed()
    } else {
        flex.boxed()
    }
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let main_window = WindowDesc::new(make_ui())
        .window_size((720., 600.))
        .with_min_size((620., 300.))
        .title("Flex Container Options");

    let demo_state = DemoState {
        input_text: "hello".into(),
        enabled: false,
        volume: 0.0,
    };

    let params = Params {
        axis: FlexType::Row,
        cross_alignment: CrossAxisAlignment::Center,
        main_alignment: MainAxisAlignment::Start,
        debug_layout: false,
        fix_minor_axis: false,
        fix_major_axis: false,
        spacers: Spacers::None,
        spacer_size: DEFAULT_SPACER_SIZE,
        fill_major_axis: false,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState { demo_state, params })
        .expect("Failed to launch application");
}
