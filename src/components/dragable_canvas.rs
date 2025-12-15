use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DragableCanvas {
    elements: Vec<Element>,

    pos: State<CursorPoint>,
    children_size: State<CursorPoint>,
    zoom_state: State<f64>,
    interactable: State<bool>,

    on_zoom: Option<EventHandler<f64>>,

    size: State<Area>,
}

impl MaybeExt for DragableCanvas {}

#[allow(dead_code)]
impl DragableCanvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            on_zoom: None,
            pos: use_state(|| CursorPoint::new(0.0, 0.0)),
            children_size: use_state(|| CursorPoint::new(0.0, 0.0)),
            zoom_state: use_state(|| 1.0),
            interactable: use_state(|| true),
            size: use_state(Area::default),
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

    pub fn pos_centered(mut self, pos: CursorPoint) -> Self {
        *self.pos.write() = CursorPoint::new(
            -pos.x + (self.size.read().width() as f64 / 2.0),
            -pos.y + (self.size.read().height() as f64 / 2.0),
        );
        self
    }

    pub fn on_zoom(mut self, on_zoom: impl Into<EventHandler<f64>>) -> Self {
        self.on_zoom = Some(on_zoom.into());
        self
    }

    pub fn children_size(mut self, children_size: CursorPoint) -> Self {
        *self.children_size.write() = children_size;
        self
    }
}

impl ChildrenExt for DragableCanvas {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Render for DragableCanvas {
    fn render(&self) -> impl IntoElement {
        let mut dragging = use_state(|| false);
        let mut hover = use_state(|| false);
        let mut mouse_coords_global: State<CursorPoint> = use_state(|| CursorPoint::new(0.0, 0.0));
        let mut mouse_coords_local: State<CursorPoint> = use_state(|| CursorPoint::new(0.0, 0.0));

        let on_zoom = self.on_zoom.clone();
        let mut zoom_state = self.zoom_state.clone();
        let mut pos = self.pos.clone();

        use_side_effect(move || {
            if let Some(on_zoom) = &on_zoom {
                on_zoom.call(zoom_state());
            }
        });

        use_drop(move || {
            if hover() || dragging() {
                Cursor::set(CursorIcon::default());
            }
        });

        let mut once = use_state(|| false);

        rect()
            //Center map
            .on_sized({
                let children_size = self.children_size.clone();
                let mut size = self.size.clone();
                move |e: Event<SizedEventData>| {
                    size.set(e.area);

                    if once() {
                        return;
                    }
                    if e.area.width() == 0.0 || e.area.height() == 0.0 {
                        return;
                    }

                    *once.write() = true;
                    *pos.write() = CursorPoint::new(
                        (children_size.read().x / -2.0) + (e.area.width() as f64 / 2.0),
                        (children_size.read().y / -2.0) + (e.area.height() as f64 / 2.0),
                    );
                }
            })
            .overflow(Overflow::Clip)
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
                .on_global_mouse_up(move |e: Event<MouseEventData>| {
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
                    if dragging() {
                        let global_location = e.global_location;
                        *pos.write() += global_location - mouse_coords_global();
                        *mouse_coords_global.write() = global_location;
                    }
                    *mouse_coords_local.write() = e.element_location;
                })
                .on_wheel({
                    let size = self.children_size.clone();
                    move |e: Event<WheelEventData>| {
                        let change = zoom_state() * e.delta_y.signum() * 0.1;
                        let current_zoom = zoom_state();
                        let new_zoom = match current_zoom + change {
                            v if v < 0.3 => 0.3,
                            v if v > 2.5 => 2.5,
                            v => v,
                        };

                        if current_zoom == new_zoom {
                            return;
                        }

                        let old_zoomd_size = size() * current_zoom;
                        let new_zoomed_size = size() * new_zoom;

                        let zoom_diff = old_zoomd_size - new_zoomed_size;

                        let cursor_image_pos = mouse_coords_local()
                            - pos()
                            - ((size() - (size() * current_zoom)) / 2.0);
                        let cursor_image_pos_percent_x =
                            (cursor_image_pos.x / old_zoomd_size.x) - 0.5;
                        let cursor_image_pos_percent_y =
                            (cursor_image_pos.y / old_zoomd_size.y) - 0.5;

                        *pos.write() = CursorPoint::new(
                            pos().x + (zoom_diff.x * cursor_image_pos_percent_x),
                            pos().y + (zoom_diff.y * cursor_image_pos_percent_y),
                        );
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
                                    .left(pos().x as f32)
                                    .top(pos().y as f32),
                            )
                            .scale((zoom_state() as f32, zoom_state() as f32))
                            .child(child.clone())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}
