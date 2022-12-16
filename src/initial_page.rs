use crate::*;

pub fn initial_page() -> impl Widget<DemoState> {
    let mut flex = Flex::row();
    flex.add_child(Checkbox::new("Demo").lens(DemoState::enabled));
    flex.add_child(Checkbox::new("Demo").lens(DemoState::enabled));
    flex
}