use gui_rs::guiprocessing;
use gui_rs::guiresources::GUIResources;
use gui_rs::guiwidgets::{GUIBase, GUIWindow};

fn main() {
    let mut guibase = GUIBase::new();
    // guibase::new_window();
    let window = GUIWindow::new(guibase.next_window_id());
    guibase.add_window(window);

    let resources = GUIResources::default();

    guiprocessing::run(guibase, resources);
}
