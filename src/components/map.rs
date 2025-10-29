use freya::prelude::*;
use rustplus_rs::{AppMap, AppMapMonument};

#[allow(non_snake_case)]
#[component]
pub fn Map(map_state: ReadOnlySignal<AppMap>, map_size: ReadOnlySignal<u32>) -> Element {
    let mut pos = use_signal(|| CursorPoint::new(0.0, 0.0));
    let mut dragging = use_signal(|| false);
    let mut mouseCoordsGlobal = use_signal(|| CursorPoint::new(0.0, 0.0));
    let mut mouseCoordsLocal = use_signal(|| CursorPoint::new(0.0, 0.0));
    let mut zoom = use_signal(|| 1.0);

    let (node_ref, node_size) = use_node();

    let scale_x = (map_state().width as f64 - map_state().ocean_margin as f64 * 2.0) * zoom() / (map_size() as f64);
    let scale_y = (map_state().height as f64 - map_state().ocean_margin as f64 * 2.0) * zoom() / (map_size() as f64);
    let offset_x = pos().x + map_state().ocean_margin as f64 * zoom() as f64;
    let offset_y = pos().y + map_state().ocean_margin as f64 * zoom() as f64;
                
    let image_width = map_state().width as f64;
    let image_height = map_state().height as f64;

    rsx!(
        // label { 
        //     {format!("Width: {}, Height: {}", node_size.area.width(), node_size.area.height())}
        //     {format!("Pos: {}, {}", pos().x, pos().y)}
        //     {format!("Zoom: {}", zoom())}
        // }

        rect {
            corner_radius: "8",
            overflow: "clip",
            width: "fill",
            height: "fill",
            background: map_state().background,
            reference: node_ref,

            onmousedown: move |e| {
                if e.trigger_button != Some(MouseButton::Left) {
                    return;
                }
                *dragging.write() = true;
                *mouseCoordsGlobal.write() = e.screen_coordinates;
            },
            onmouseup: move |e| {
                if e.trigger_button != Some(MouseButton::Left) {
                    return;
                }
                *dragging.write() = false;
            },
            onmouseleave: move |_| {
                *dragging.write() = false;
            },
            onmousemove: move |e| {
                if *dragging.read() {
                    *pos.write() += e.screen_coordinates - *mouseCoordsGlobal.read();
                    *mouseCoordsGlobal.write() = e.screen_coordinates;
                }
                *mouseCoordsLocal.write() = e.element_coordinates;
            },
            onwheel: move |e| {
                let zoom_delta = e.get_delta_y().signum() * 0.1;
                let change = zoom() * zoom_delta;

                let current_zoom = zoom();
                let current_pos = pos();
                
                let mouse_img_pos = mouseCoordsLocal() - current_pos;
                let current_zoomed_width = image_width * current_zoom;
                let current_zoomed_height = image_height * current_zoom;
                
                let mouse_pos_percent_x = mouse_img_pos.x / current_zoomed_width;
                let mouse_pos_percent_y = mouse_img_pos.y / current_zoomed_height;
                
                let new_zoomed_width = image_width * (current_zoom + change);
                let new_zoomed_height = image_height * (current_zoom + change);
                
                let new_mouse_pos_x = mouse_pos_percent_x * new_zoomed_width;
                let new_mouse_pos_y = mouse_pos_percent_y * new_zoomed_height;
                
                *pos.write() = CursorPoint::new(
                    current_pos.x + (mouse_img_pos.x - new_mouse_pos_x),
                    current_pos.y + (mouse_img_pos.y - new_mouse_pos_y),
                );
                *zoom.write() = current_zoom + change;
            },

            image {
                image_data: dynamic_bytes(map_state().jpg_image.clone()),
                position: "absolute",
                position_left: pos().x.to_string(),
                position_top: pos().y.to_string(),
                width: (image_width * zoom()).to_string(),
                height: (image_height * zoom()).to_string(),
            }

            for monument in &map_state().monuments {
                rect {
                    width: "100",
                    height: "10",
                    background: "red",
                    position: "absolute",
                    main_align: "center",
                    cross_align: "center",
                    text_align: "right",
                    position_left: (monument.x as f64 * scale_x + offset_x).to_string(),
                    position_top: ((map_size() as f64 - monument.y as f64) * scale_y + offset_y).to_string(),

                    label {
                        width: "100",
                        {monument.token.clone()}
                    }
                }
            }
        }
    )
}