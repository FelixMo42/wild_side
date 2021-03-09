use crate::pane::*;

pub enum FlexConstraint {
    Fixed(usize),
    Flex(usize),
}

/* pub fn layout<Event>(canvas: Canvas, selected: usize, layout: &Vec<Box<dyn Pane<Event>>>) {
    let area = canvas.area();

    let space_used_by_fixed_elements = layout.iter()
        .map(|(_, constraint)| match constraint {
            FlexConstraint::Fixed(size) => size.clone(),
            FlexConstraint::Flex(_) => 0,
        }).sum::<usize>();

    let flex_total = layout.iter()
        .map(|(_, constraint)| match constraint {
            FlexConstraint::Fixed(_) => 0,
            FlexConstraint::Flex(flex) => flex.clone(),
        }).sum::<usize>();

    let flex_space_available = area.1.x - space_used_by_fixed_elements;

    let flex_per_unit = flex_space_available / flex_total;

    let mut offset = 0;

    layout.into_iter().enumerate().for_each(|(y, (pane, constraint))| {
        let size: usize = match constraint {
            FlexConstraint::Fixed(size) => size.clone(),
            FlexConstraint::Flex(flex) => flex_per_unit * flex,
        };

        canvas.draw_pane(
            pane.as_ref(),
            area.horizontal_slice(offset, offset + size),
            selected == y
        );

        offset += size;
    });
} */

///
type FlexLayout<Event> = Vec<(Box<dyn Pane<Event>>, FlexConstraint)>;

fn space_used_by_fixed_panes<Event>(layout: &FlexLayout<Event>) -> usize {
    layout.iter().map(|(_, constraint)| match constraint {
        FlexConstraint::Fixed(size) => size.clone(),
        FlexConstraint::Flex(_) => 0
    }).sum()
}

fn total_flex<Event>(layout: &FlexLayout<Event>) -> usize {
    layout.iter().map(|(_, constraint)| match constraint {
        FlexConstraint::Fixed(_) => 0,
        FlexConstraint::Flex(flex) => flex.clone(),
    }).sum::<usize>()
}

fn char_per_flex<Event>(layout: &FlexLayout<Event>, total_space_available: usize) -> usize {
    let space_available_for_flex = total_space_available - space_used_by_fixed_panes(layout);

    return space_available_for_flex / total_flex(layout);
} 

pub struct VertFlexPane<Event>(pub usize, pub FlexLayout<Event>);

impl<Event> VertFlexPane<Event> {
    pub fn next(&mut self) {
        self.0 = (self.0 + 1) % self.1.len();
    }

    pub fn prev(&mut self) {
        if self.0 == 0 {
            self.0 = self.1.len()
        } else {
            self.0 -=  1;
        }
    }
}

impl<Event> Pane<Event> for VertFlexPane<Event> {
    fn render(&self, canvas: Canvas) {
        let area = canvas.area();
        let char_per_flex = char_per_flex(&self.1, area.1.y);

        self.1.iter().fold(canvas, |canvas, (pane, constraint)| {
            let (left, right) = canvas.splitv(match constraint {
                FlexConstraint::Fixed(size) => size.clone(),
                FlexConstraint::Flex(flex) => flex * char_per_flex,
            });
            
            left.draw_pane(pane);

            return right;
        });
    }

    fn event(&mut self, event: Event) {
        self.1[self.0].0.event(event);
    }
}

pub struct HorzFlexPane<Event>(pub usize, pub FlexLayout<Event>);

impl<Event> HorzFlexPane<Event> {
    pub fn next(&mut self) {
        self.0 = (self.0 + 1) % self.1.len();
    }

    pub fn prev(&mut self) {
        if self.0 == 0 {
            self.0 = self.1.len()
        } else {
            self.0 -=  1;
        }
    }
}

impl<Event> Pane<Event> for HorzFlexPane<Event> {
    fn render(&self, canvas: Canvas) {
        let area = canvas.area();
        let char_per_flex = char_per_flex(&self.1, area.1.x);

        self.1.iter().fold(canvas, |canvas, (pane, constraint)| {
            let (top, bottom) = canvas.splith(match constraint {
                FlexConstraint::Fixed(size) => size.clone(),
                FlexConstraint::Flex(flex) => flex * char_per_flex,
            });
            
            top.draw_pane(pane);

            return bottom;
        });
    }

    fn event(&mut self, event: Event) {
        self.1[self.0].0.event(event);
    }
}
