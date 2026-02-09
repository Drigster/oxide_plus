use freya::prelude::*;

pub const MIN_ZOOM: f32 = 0.3;
pub const MAX_ZOOM: f32 = 1.6;

pub type Point2D = euclid::Point2D<f32, ()>;

#[derive(Clone, PartialEq)]
pub struct DragableCanvas {
    elements: Vec<Element>,

    pos: Option<Writable<Point2D>>,
    zoom: Option<Writable<f32>>,
    children_size: Option<Readable<Point2D>>,
    interactable: Option<Readable<bool>>,
}

#[allow(dead_code)]
impl DragableCanvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),

            pos: None,
            zoom: None,
            children_size: None,
            interactable: None,
        }
    }

    pub fn pos(mut self, pos: impl Into<Writable<Point2D>>) -> Self {
        self.pos = Some(pos.into());
        self
    }

    pub fn zoom(mut self, zoom: impl Into<Writable<f32>>) -> Self {
        self.zoom = Some(zoom.into());
        self
    }

    pub fn children_size(mut self, children_size: impl Into<Readable<Point2D>>) -> Self {
        self.children_size = Some(children_size.into());
        self
    }

    pub fn interactable(mut self, interactable: impl Into<Readable<bool>>) -> Self {
        self.interactable = Some(interactable.into());
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

        let pos = use_hook(|| {
            self.pos.clone()
                .unwrap_or_else(|| State::create(Point2D::new(0.0, 0.0)).into())
        });
        let zoom = use_hook(|| self.zoom.clone().unwrap_or_else(|| State::create(1.0).into()));
        let children_size = use_hook(|| {
            self.children_size
                .clone()
                .unwrap_or_else(|| Readable::from_value(Point2D::new(0.0, 0.0)))
        });
        let interactable = use_hook(|| {
            self.interactable
                .clone()
                .unwrap_or_else(|| Readable::from_value(true))
        });
        let mut size = use_state(|| Area::default());

        use_drop(move || {
            if hover() || dragging() {
                Cursor::set(CursorIcon::default());
            }
        });

        let mut once = use_state(|| false);

        rect()
            //Center map
            .on_sized({
                let children_size = children_size.clone();
                let mut pos = pos.clone();
                move |e: Event<SizedEventData>| {
                    *size.write() = e.area;
                    if once() {
                        return;
                    }
                    if e.area.width() == 0.0 || e.area.height() == 0.0 {
                        return;
                    }
                    *once.write() = true;

                    *pos.write() = Point2D::new(children_size.read().x / -2.0, children_size.read().y / -2.0);
                }
            })
            .overflow(Overflow::Clip)
            .width(Size::Fill)
            .height(Size::Fill)
            .maybe(*interactable.read(), |rect| {
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
                    let mut pos = pos.clone();
                    move |e: Event<MouseEventData>| {
                        if dragging() {
                            let global_location = e.global_location.to_f32();
                            let new_pos = *pos.read() + (global_location - mouse_coords_global());
                            *pos.write() = new_pos;
                            // TODO: Cancel centering

                            *mouse_coords_global.write() = global_location;
                        }
                        *mouse_coords_local.write() = e.element_location.to_f32();
                    }
                })
                .on_wheel({
                    let mut pos = pos.clone();
                    let mut zoom = zoom.clone();
                    move |e: Event<WheelEventData>| {
                        let current_zoom = *zoom.read();
                        let current_pos = *pos.read();
                        let children_size = *children_size.read();

                        let change = current_zoom * e.delta_y.signum() as f32 * 0.1;
                        let new_zoom = match current_zoom + change {
                            v if v < MIN_ZOOM => MIN_ZOOM,
                            v if v > MAX_ZOOM => MAX_ZOOM,
                            v => v,
                        };

                        if current_zoom == new_zoom {
                            return;
                        }

                        let old_zoomd_size = children_size * current_zoom;
                        let new_zoomed_size = children_size * new_zoom;

                        let zoom_diff = old_zoomd_size - new_zoomed_size;

                        let cursor_image_pos = mouse_coords_local()
                            - current_pos
                            - (size.read().size / 2.0).into()
                            - ((children_size - (children_size * current_zoom)) / 2.0);
                        let cursor_image_pos_percent_x =
                            (cursor_image_pos.x / old_zoomd_size.x) - 0.5;
                        let cursor_image_pos_percent_y =
                            (cursor_image_pos.y / old_zoomd_size.y) - 0.5;

                        *pos.write() = Point2D::new(
                            current_pos.x + (zoom_diff.x * cursor_image_pos_percent_x),
                            current_pos.y + (zoom_diff.y * cursor_image_pos_percent_y),
                        );
                        *zoom.write() = new_zoom;
                    }
                })
            })
            .children(
                self.elements
                    .iter()
                    .map(|child| {
                        let zoom = *zoom.read();
                        let pos = *pos.read();
                        rect()
                            .position(
                                Position::new_absolute()
                                    .left((pos.x as f32) + (size.read().width() / 2.0))
                                    .top((pos.y as f32) + (size.read().height() / 2.0)),
                            )
                            .scale((zoom as f32, zoom as f32))
                            .child(child.clone())
                            .into()
                    })
                    .collect::<Vec<Element>>(),
            )
    }
}
