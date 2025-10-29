use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DropdownItem(
) -> Element {
    rsx!(
        rect {
            width: "fill-min",
            color: "{font_theme.color}",
            a11y_id,
            a11y_role: "button",
            a11y_member_of,
            background: "{background}",
            border,
            padding: "6 10",
            corner_radius: "6",
            main_align: "center",
            onpointerenter,
            onpointerleave,
            onclick,
            onkeydown,
            {children}
        }
    )
}

#[allow(non_snake_case)]
pub fn Dropdown(
) -> Element {

    rsx!(
        rect {
            direction: "vertical",
            rect {
                width: "{width}",
                onpointerenter,
                onpointerleave,
                onclick,
                onglobalkeydown,
                margin: "{margin}",
                a11y_id,
                a11y_member_of,
                background: "{background}",
                color: "{font_theme.color}",
                corner_radius: "8",
                padding: "6 16",
                border,
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                {selected_item}
                // ArrowIcon {
                //     rotate: "0",
                //     fill: "{arrow_fill}",
                //     theme: theme_with!(IconTheme {
                //         margin : "0 0 0 8".into(),
                //     })
                // }
            }
            if *opened.read() {
                rect {
                    height: "0",
                    width: "0",
                    rect {
                        width: "100v",
                        margin: "4 0 0 0",
                        rect {
                            onglobalpointerup,
                            onglobalkeydown,
                            layer: "overlay",
                            margin: "{margin}",
                            border: "1 inner {border_fill}",
                            overflow: "clip",
                            corner_radius: "8",
                            background: "{dropdown_background}",
                            shadow: "0 2 4 0 rgb(0, 0, 0, 0.15)",
                            padding: "6",
                            content: "fit",
                            {children}
                        }
                    }
                }
            }
        }
    )
}