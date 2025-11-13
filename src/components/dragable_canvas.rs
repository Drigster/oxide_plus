use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DragableCanvas {
    elements: Vec<Element>,

    pos: State<CursorPoint>,
    zoom_state: State<f64>,
    interactable: State<bool>,

    on_zoom: Option<EventHandler<f64>>,
}

impl DragableCanvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            on_zoom: None,
            pos: use_state(|| CursorPoint::new(0.0, 0.0)),
            zoom_state: use_state(|| 1.0),
            interactable: use_state(|| true),
        }
    }

    pub fn interactable(mut self, interactable: bool) -> Self {
        *self.interactable.write() = interactable;
        self
    }

    pub fn zoom(mut self, zoom: f64) -> Self {
        *self.zoom_state.write() = zoom;
        self
    }

    pub fn pos(mut self, pos: CursorPoint) -> Self {
        *self.pos.write() = pos;
        self
    }

    pub fn on_zoom(mut self, on_zoom: impl Into<EventHandler<f64>>) -> Self {
        self.on_zoom = Some(on_zoom.into());
        self
    }
}

impl ChildrenExt for DragableCanvas {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Render for DragableCanvas {
    fn render(&self) -> Element {
        let mut dragging = use_state(|| false);
        let mut hover = use_state(|| false);
        let mut mouse_coords_global = use_state(|| CursorPoint::new(0.0, 0.0));
        let mut mouse_coords_local = use_state(|| CursorPoint::new(0.0, 0.0));

        let on_zoom = self.on_zoom.clone();
        let mut zoom_state = self.zoom_state.clone();
        let mut pos = self.pos.clone();

        use_side_effect(move || {
            if let Some(on_zoom) = &on_zoom {
                on_zoom.call(*zoom_state.read());
            }
        });

        use_drop(move || {
            if hover() || dragging() {
                Cursor::set(CursorIcon::default());
            }
        });

        rect()
            .overflow_mode(OverflowMode::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .maybe(*self.interactable.read(), |rect| {
                rect.on_mouse_down(move |e: Event<MouseEventData>| {
                    if e.button != Some(MouseButton::Left) {
                        return;
                    }
                    *dragging.write() = true;
                    *mouse_coords_global.write() = e.global_location;
                    Cursor::set(CursorIcon::Grabbing);
                })
                .on_mouse_up(move |e: Event<MouseEventData>| {
                    if e.button != Some(MouseButton::Left) {
                        return;
                    }
                    *dragging.write() = false;
                    if hover() {
                        Cursor::set(CursorIcon::Grab);
                    } else {
                        Cursor::set(CursorIcon::default());
                    }
                })
                .on_pointer_enter(move |_| {
                    *hover.write() = true;
                    Cursor::set(CursorIcon::Grab);
                })
                .on_pointer_leave(move |_| {
                    *hover.write() = false;
                    Cursor::set(CursorIcon::default());
                })
                .on_mouse_move(move |e: Event<MouseEventData>| {
                    if *dragging.read() {
                        *pos.write() += e.global_location - *mouse_coords_global.read();
                        *mouse_coords_global.write() = e.global_location;
                    }
                    *mouse_coords_local.write() = e.element_location;
                })
                .on_wheel(move |e: Event<WheelEventData>| {
                    let change = *zoom_state.read() * e.delta_y.signum() * 0.1;
                    let current_zoom = *zoom_state.read();
                    // let current_pos = pos;
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

                    let new_zoom = current_zoom + change;
                    if new_zoom < 0.3 {
                        *zoom_state.write() = 0.3;
                    } else if new_zoom > 2.5 {
                        *zoom_state.write() = 2.5;
                    } else {
                        *zoom_state.write() = new_zoom;
                    }
                })
            })
            .children(
                self.elements
                    .iter()
                    .map(|child| {
                        rect()
                            .position(
                                Position::new_absolute()
                                    .left(pos.read().x as f32)
                                    .top(pos.read().y as f32),
                            )
                            .scale((*zoom_state.read() as f32, *zoom_state.read() as f32))
                            .child(child.clone())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
            .into()
    }
}
