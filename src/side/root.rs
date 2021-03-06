use termion::event::Event;
use termion::event::Key;

use ignore::WalkBuilder;

use crate::pane::{Canvas, Pane};
use crate::side::Editor;
use crate::color::{GRAY2, GRAY3, GRAY6};

///
pub struct FileSideBar {
    files: Vec<String>,
    selector: String
}

impl FileSideBar {
    fn new() -> FileSideBar {
        return FileSideBar {
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
}

impl Pane<Event> for FileSideBar {
    fn render(&self, canvas: Canvas) {
        let size = canvas.size();

        for (y, file) in self.files.iter().enumerate() {
            let path = file
                .chars()
                .skip(2)
                .take(size.x - 5 - 2)
                .collect::<String>();

            canvas.draw_line_with_style(
                (1, y + 2).into(),
                format!("{:>3}", y).as_str(),
                &GRAY6.as_fg()
            );

            canvas.draw_line_with_style(
                (5, y + 2).into(),
                path.as_str(),
                &GRAY3.as_fg()
            );
        }

        canvas.draw_line_with_style(
            (5, 1).into(),
            self.selector.as_str(),
            &GRAY2.as_fg()
        );
    }

    fn event(&mut self, event: Event) -> bool {
        return match event {
            Event::Key(Key::Char(c@('0'..='9'))) => {
                self.selector.push(c);
                true
            },
            Event::Key(Key::Backspace) => {
                if self.selector.len() == 0 {
                    false
                } else {
                    self.selector.remove(
                        self.selector
                            .char_indices()
                            .map(|(i, _)| i)
                            .last().unwrap()
                    );
                    true
                }
            },
            Event::Key(Key::Char('\n')) => {
                if let Ok(selection) = self.selector.parse::<usize>() {
                    self.selector = "".to_string();
                    let _file = &self.files.get(selection);
                }

                true
            },
            _ => false
        };
    }
}

enum LayoutConstraint {
    Fixed(usize),
    Flex(usize)
}

type Layout<'a> = Vec<(&'a dyn Pane<Event>, LayoutConstraint)>;

fn layout<Event>(canvas: Canvas, layout: Layout) {
    let area = canvas.area();

    let space_used_by_fixed_elements = layout.iter()
        .map(|(_, constraint)| match constraint {
            LayoutConstraint::Fixed(size) => size.clone(),
            LayoutConstraint::Flex(_) => 0,
        }).sum::<usize>();

    let flex_total = layout.iter()
        .map(|(_, constraint)| match constraint {
            LayoutConstraint::Fixed(_) => 0,
            LayoutConstraint::Flex(flex) => flex.clone(),
        }).sum::<usize>();

    let flex_space_available = area.1.x - space_used_by_fixed_elements;

    let flex_per_unit = flex_space_available / flex_total;

    let mut offset = 0;

    layout.into_iter().for_each(|(pane, constraint)| {
        let size: usize = match constraint {
            LayoutConstraint::Fixed(size) => size.clone(),
            LayoutConstraint::Flex(flex) => flex_per_unit * flex,
        };

        canvas.draw_pane(pane, area.horizontal_slice(offset, offset + size));

        offset += size;
    });
}

///
pub struct SideRootPane {
    left: FileSideBar,
    text: Editor,
    selected: bool,
}

impl SideRootPane {
    pub fn new() -> SideRootPane {
        SideRootPane {
            left: FileSideBar::new(),
            text: Editor::load("./src/main.rs".to_string()),
            selected: true
        }
    }

    pub fn get_selected_pane(&mut self) -> &mut dyn Pane<Event> {
        if self.selected {
            return &mut self.text;
        } else {
            return &mut self.left;
        }
    }
}

impl Pane<Event> for SideRootPane {
    fn render(&self, canvas: Canvas) {
        layout::<Event>(canvas, vec![
               (&self.left, LayoutConstraint::Flex(1)),
               (&self.text, LayoutConstraint::Fixed(84)),
               //(&self.left, LayoutConstraint::Flex(1)),
        ]);
    }

    fn event(&mut self, event: Event) -> bool {
        return match event {
            Event::Key(Key::Char('\t')) => {
                self.selected = !self.selected;
                return false;
            },

            _ => self.get_selected_pane().event(event)
        };
    }
}
