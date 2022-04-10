use gui_rs::guiprocessing;
use gui_rs::guiproperties::guiposition::{GUILength, GUIPosition, GUISize};
use gui_rs::guiproperties::guitraits::{Parent, Widget};
use gui_rs::guiproperties::GUIColor;
use gui_rs::guiresources::{GUIBackend, GUIResources};
use gui_rs::guiwidgets::{GUIBase, GUIButton, GUIWindow};

fn main() {
    let mut guibase = GUIBase::new();

    let window = GUIWindow::default();

    let mut button_one = GUIButton::default();
    button_one.size = GUISize::from_pixels(175., 150.);

    let window_id = guibase.add_window(window);
    guibase.add_child_to_parent(button_one, window_id);

    let mut button_two = GUIButton::default();
    button_two.size = GUISize::from_pixels(50., 20.);
    button_two.radius = GUILength::from_pixels(5.);
    button_two.position = GUIPosition::from_pixels(20., 10.);
    button_two.background_color = GUIColor::from_rgba_u8u8u8u8(255, 0, 0, 255);
    guibase.add_child_to_parent(button_two, window_id);

    let mut button_three = GUIButton::default();
    button_three.size = GUISize::from_pixels(120., 200.);
    button_three.position = GUIPosition::from_pixels(100., 100.);
    button_three.background_color = GUIColor::from_rgba_u8u8u8u8(0, 150, 150, 255);
    guibase.add_child_to_parent(button_three, window_id);

    let resources = GUIResources::default();

    guiprocessing::run(guibase, resources);
}
