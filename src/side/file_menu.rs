use crate::color::{GRAY2, GRAY3, GRAY6};
use crate::pane::{Canvas, Pane};
use crate::side::Event;
use ignore::WalkBuilder;
use std::sync::mpsc::Sender;

///
pub struct FileMenu {
    files: Vec<String>,
    selector: String,
    emiter: Sender<Event>,
}

impl FileMenu {
    pub fn new(emiter: Sender<Event>) -> FileMenu {
        return FileMenu {
            emiter,
            selector: "".to_string(),
            files: WalkBuilder::new("./")
                .build()
                .filter_map(|result| result.map_or(None, |file| Some(file)))
                .filter(|file| file.file_type().unwrap().is_file())
                .filter_map(|file| file.path().to_str().map(|p| p.to_string()))
                .map(|path| path.to_string())
                .collect::<Vec<String>>(),
        };
    }

    pub fn open_selected_file(&mut self) {
        if let Ok(selection) = self.selector.parse::<usize>() {
            self.selector = "".to_string();
            if let Some(path) = self.files.get(selection) {
                self.emiter.send(Event::OpenFile(path.clone())).unwrap();
            }
        }

    }
}

impl Pane<Event> for FileMenu {
    fn render(&self, canvas: Canvas) {
        let size = canvas.size();

        for (y, file) in self.files.iter().enumerate() {
            let path = file
                .chars()
                .skip(2)
                .take(size.x - 4 - 1)
                .collect::<String>();

            canvas.draw_line_with_style(
                (1, y + 2).into(),
                format!("{:>2}", y).as_str(),
                &GRAY6.as_fg(),
            );

            canvas.draw_line_with_style((4, y + 2).into(), path.as_str(), &GRAY3.as_fg());
        }

        canvas.draw_line_with_style((4, 1).into(), self.selector.as_str(), &GRAY2.as_fg());
    }

    fn event(&mut self, event: Event) -> bool {
        return match event {
            Event::Char(c @ ('0'..='9')) => {
                self.selector.push(c);
                true
            }
            Event::Delete => {
                if self.selector.len() == 0 {
                    false
                } else {
                    self.selector
                        .remove(self.selector.char_indices().map(|(i, _)| i).last().unwrap());
                    true
                }
            }
            Event::Return => {
                self.open_selected_file();
                true
            }
            _ => false,
        };
    }
}
