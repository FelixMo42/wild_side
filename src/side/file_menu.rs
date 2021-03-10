use crate::color::*;
use crate::pane::*;
use crate::side::*;

use ignore::WalkBuilder;

use std::sync::mpsc::Sender;

///
pub struct FileMenu {
    files: Vec<String>,
    selector: String,
    emiter: Sender<Event>,
}

impl FileMenu {
    pub fn new(emiter: Sender<Event>) -> Box<FileMenu> {
        return Box::new(FileMenu {
            emiter,
            selector: "".to_string(),
            files: WalkBuilder::new("./")
                .build()
                .filter_map(|result| result.map_or(None, |file| Some(file)))
                .filter(|file| file.file_type().unwrap().is_file())
                .filter_map(|file| file.path().to_str().map(|p| p.to_string()))
                .map(|path| path.to_string())
                .collect::<Vec<String>>(),
        });
    }

    pub fn open_selected_file(&mut self) {
        if let Ok(selection) = self.selector.parse::<usize>() {
            self.selector = "".to_string();
            if let Some(path) = self.files.get(selection) {
                self.emiter.send(Event::OpenFile(path.clone())).unwrap();
            }
        }

    }

    pub fn delete(&mut self) {
        if self.selector.len() == 0 {
            return;
        }

        let index = self.selector
            .char_indices()
            .map(|(i, _)| i)
            .last().unwrap();

        self.selector.remove(index);
    }
}

impl Pane<Event> for FileMenu {
    fn render(&self, mut canvas: Canvas, _focused: bool) {
        let size = canvas.size();

        canvas.style(THEME.style(1));

        for (y, file) in self.files.iter().enumerate() {
            let path = file
                .chars()
                .skip(2)
                .take(size.x - 4 - 1)
                .collect::<String>();

            // draw line number
            canvas.draw_line_with_style(
                (1, y + 1).into(),
                format!("{:>2}", y).chars(),
                THEME.disabled(1).as_fg(),
            );

            // draw line
            canvas.draw_line((4, y + 1).into(), path.chars());
        }

        // draw prompt
        canvas.draw_line_with_style(
            (4, 0).into(),
            self.selector.chars(),
            THEME.focused(1).as_fg()
        );
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::Char(c @ ('0'..='9')) => self.selector.push(c),
            Event::Delete => self.delete(),
            Event::Return => self.open_selected_file(),
            _ => (),
        };
    }
}
