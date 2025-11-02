use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DragableCanvas {
    elements: Vec<Element>,
}

impl DragableCanvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl ChildrenExt for DragableCanvas {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Render for DragableCanvas {
    fn render(&self) -> Element {
        let mut pos = use_state(|| CursorPoint::new(0.0, 0.0));
        let mut dragging = use_state(|| false);
        let mut mouse_coords_global = use_state(|| CursorPoint::new(0.0, 0.0));
        let mut mouse_coords_local = use_state(|| CursorPoint::new(0.0, 0.0));
        let mut zoom = use_state(|| 1.0);

        rect()
            .overflow_mode(OverflowMode::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .on_mouse_down(move |e: Event<MouseEventData>| {
                if e.button != Some(MouseButton::Left) {
                    return;
                }
                *dragging.write() = true;
                *mouse_coords_global.write() = e.global_location;
            })
            .on_mouse_up(move |e: Event<MouseEventData>| {
                if e.button != Some(MouseButton::Left) {
                    return;
                }
                *dragging.write() = false;
            })
            // onmouseleave: move |_| {
            //     *dragging.write() = false;
            // },
            .on_mouse_move(move |e: Event<MouseEventData>| {
                if *dragging.read() {
                    *pos.write() += e.global_location - *mouse_coords_global.read();
                    *mouse_coords_global.write() = e.global_location;
                }
                *mouse_coords_local.write() = e.element_location;
            })
            .on_wheel(move |e: Event<WheelEventData>| {
                let change = zoom() * e.delta_y.signum() * 0.1;
                let current_zoom = zoom();
                // let current_pos = pos();
                // let mouse_img_pos = mouse_coords_local() - current_pos;
                // let current_zoomed_width = image_width * current_zoom;
                // let current_zoomed_height = image_height * current_zoom;
                // let mouse_pos_percent_x = mouse_img_pos.x / current_zoomed_width;
                // let mouse_pos_percent_y = mouse_img_pos.y / current_zoomed_height;
                // let new_zoomed_width = image_width * (current_zoom + change);
                // let new_zoomed_height = image_height * (current_zoom + change);
                // let new_mouse_pos_x = mouse_pos_percent_x * new_zoomed_width;
                // let new_mouse_pos_y = mouse_pos_percent_y * new_zoomed_height;
                // *pos.write() = CursorPoint::new(
                //     current_pos.x + (mouse_img_pos.x - new_mouse_pos_x),
                //     current_pos.y + (mouse_img_pos.y - new_mouse_pos_y),
                // );
                *zoom.write() = current_zoom + change;
            })
            .children(
                self.elements
                    .iter()
                    .map(|child| {
                        rect()
                            .position(
                                Position::new_absolute()
                                    .left(pos().x as f32)
                                    .top(pos().y as f32),
                            )
                            .scale((zoom() as f32, zoom() as f32))
                            .child(child.clone())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .into()
    }
}
