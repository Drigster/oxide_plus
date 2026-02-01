use freya::prelude::*;

pub const MIN_ZOOM: f32 = 0.3;
pub const MAX_ZOOM: f32 = 2.5;

pub type Point2D = euclid::Point2D<f32, ()>;

#[derive(Clone, PartialEq)]
pub struct DragableCanvas {
    elements: Vec<Element>,

    pos: Point2D,
    children_size: Point2D,
    zoom: f32,
    interactable: bool,

    on_zoom: Option<EventHandler<f32>>,

    size: Area,
}

#[allow(dead_code)]
impl DragableCanvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            on_zoom: None,
            pos: Point2D::new(0.0, 0.0),
            children_size: Point2D::new(0.0, 0.0),
            zoom: 1.0,
            interactable: true,
            size: Area::default(),
        }
    }

    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self
    }

    pub fn zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn pos(mut self, pos: Point2D) -> Self {
        self.pos = pos;
        self
    }

    pub fn pos_centered(mut self, pos: Point2D) -> Self {
        self.pos = Point2D::new(
            -pos.x + (self.size.width() / 2.0),
            -pos.y + (self.size.height() / 2.0),
        );
        self
    }

    pub fn on_zoom(mut self, on_zoom: impl FnMut(f32) + 'static) -> Self {
        self.on_zoom = Some(EventHandler::new(on_zoom));
        self
    }

    pub fn children_size(mut self, children_size: Point2D) -> Self {
        self.children_size = children_size;
        self
    }
}

impl ChildrenExt for DragableCanvas {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl Component for DragableCanvas {
    fn render(&self) -> impl IntoElement {
        let mut dragging = use_state(|| false);
        let mut hover = use_state(|| false);
        let mut mouse_coords_global: State<Point2D> = use_state(|| Point2D::new(0.0, 0.0));
        let mut mouse_coords_local: State<Point2D> = use_state(|| Point2D::new(0.0, 0.0));

        let mut pos = use_state(|| self.pos);
        use_side_effect_with_deps(&self.pos, {
            let mut pos = pos.clone();
            move |self_pos| {
                *pos.write() = *self_pos;
            }
        });

        let mut size = use_state(|| self.size);
        use_side_effect_with_deps(&self.size, {
            let mut size = size.clone();
            move |self_size| {
                *size.write() = *self_size;
            }
        });

        let mut zoom = use_state(|| self.zoom);
        use_side_effect_with_deps(&self.zoom, {
            let mut zoom = zoom.clone();
            move |self_zoom| {
                *zoom.write() = *self_zoom;
            }
        });

        use_side_effect({
            let on_zoom = self.on_zoom.clone();
            let zoom = self.zoom.clone();
            move || {
                if let Some(on_zoom) = &on_zoom {
                    on_zoom.call(zoom);
                }
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
                move |e: Event<SizedEventData>| {
                    *size.write() = e.area;

                    if once() {
                        return;
                    }
                    if e.area.width() == 0.0 || e.area.height() == 0.0 {
                        return;
                    }

                    *once.write() = true;
                    *pos.write() = Point2D::new(
                        (children_size.x / -2.0) + (e.area.width() / 2.0),
                        (children_size.y / -2.0) + (e.area.height() / 2.0),
                    );
                }
            })
            .overflow(Overflow::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .maybe(self.interactable, |rect| {
                rect.on_mouse_down(move |e: Event<MouseEventData>| {
                    if e.button != Some(MouseButton::Left) {
                        return;
                    }
                    *dragging.write() = true;
                    *mouse_coords_global.write() = e.global_location.to_f32();
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
                .on_mouse_move({
                    move |e: Event<MouseEventData>| {
                        if dragging() {
                            let global_location = e.global_location.to_f32();
                            *pos.write() += global_location - mouse_coords_global();
                            *mouse_coords_global.write() = global_location;
                        }
                        *mouse_coords_local.write() = e.element_location.to_f32();
                    }
                })
                .on_wheel({
                    let size = self.children_size.clone();
                    move |e: Event<WheelEventData>| {
                        let change = *zoom.read() * (e.delta_y as f32).signum() * 0.1;
                        let current_zoom = *zoom.read();
                        let new_zoom = match current_zoom + change {
                            v if v < MIN_ZOOM => MIN_ZOOM,
                            v if v > MAX_ZOOM => MAX_ZOOM,
                            v => v,
                        };

                        if current_zoom == new_zoom {
                            return;
                        }

                        let old_zoomd_size = size * current_zoom;
                        let new_zoomed_size = size * new_zoom;

                        let zoom_diff = old_zoomd_size - new_zoomed_size;

                        let cursor_image_pos: euclid::Vector2D<f32, ()> = mouse_coords_local()
                            - *pos.read()
                            - ((size - (size * current_zoom)) / 2.0);
                        let cursor_image_pos_percent_x =
                            (cursor_image_pos.x / old_zoomd_size.x) - 0.5;
                        let cursor_image_pos_percent_y =
                            (cursor_image_pos.y / old_zoomd_size.y) - 0.5;

                        let new_pos = Point2D::new(
                            pos.read().x + (zoom_diff.x * cursor_image_pos_percent_x),
                            pos.read().y + (zoom_diff.y * cursor_image_pos_percent_y),
                        );
                        *pos.write() = new_pos;
                        *zoom.write() = new_zoom;
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
                                    .left(pos.read().x)
                                    .top(pos.read().y),
                            )
                            .scale((*zoom.read(), *zoom.read()))
                            .child(child.clone())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}
