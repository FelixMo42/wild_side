use crate::pane::{Canvas, Pane};

pub enum LayoutConstraint {
    Fixed(usize),
    Flex(usize),
}

pub type Layout<Event> = Vec<(Box<dyn Pane<Event>>, LayoutConstraint)>;

pub fn layout<Event>(canvas: Canvas, selected: usize, layout: &Layout<Event>) {
    let area = canvas.area();

    let space_used_by_fixed_elements = layout
        .iter()
        .map(|(_, constraint)| match constraint {
            LayoutConstraint::Fixed(size) => size.clone(),
            LayoutConstraint::Flex(_) => 0,
        })
        .sum::<usize>();

    let flex_total = layout
        .iter()
        .map(|(_, constraint)| match constraint {
            LayoutConstraint::Fixed(_) => 0,
            LayoutConstraint::Flex(flex) => flex.clone(),
        })
        .sum::<usize>();

    let flex_space_available = area.1.x - space_used_by_fixed_elements;

    let flex_per_unit = flex_space_available / flex_total;

    let mut offset = 0;

    layout.into_iter().enumerate().for_each(|(y, (pane, constraint))| {
        let size: usize = match constraint {
            LayoutConstraint::Fixed(size) => size.clone(),
            LayoutConstraint::Flex(flex) => flex_per_unit * flex,
        };

        canvas.draw_pane(
            pane.as_ref(),
            area.horizontal_slice(offset, offset + size),
            selected == y
        );

        offset += size;
    });
}
