use crate::pane::*;
use crate::side::*;

use ignore::WalkBuilder;

use std::sync::mpsc::Sender;

pub struct FileMenu {
    menu: Menu,
    emiter: Sender<Event>
}

///
impl FileMenu {
    pub fn new(emiter: Sender<Event>) -> Box<FileMenu> {
        let files = WalkBuilder::new("./")
            .build()
            .filter_map(|result| result.map_or(None, |file| Some(file)))
            .filter(|file| file.file_type().unwrap().is_file())
            .filter_map(|file| file.path().to_str().map(|p| p.to_string()))
            .map(|path| path.to_string())
            .collect::<Vec<String>>();

        return Box::new(FileMenu { emiter, menu: Menu::new(files) });
    }
}

impl Pane<Event> for FileMenu {
    fn render(&self, canvas: Canvas, focused: bool) {
        self.menu.render(canvas, focused);
    }

    fn event(&mut self, event: Event) {
        if let Some(path) = self.menu.event(event) {
            self.menu.clear_selector();
            self.emiter.send(Event::OpenFile(path)).unwrap();
        }
    }
}
