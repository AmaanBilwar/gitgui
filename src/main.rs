use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowKind, WindowOptions, div, prelude::*, px, rgb, size, uniform_list
};

struct UniformListExample {}

impl Render for UniformListExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0xffffff)).child(
            uniform_list(
                "entries",
                50,
                cx.processor(|_this, range, _window, _cx| {
                    let mut items = Vec::new();
                    for ix in range {
                        let item = ix + 1;

                        items.push(
                            div()
                                .id(ix)
                                .px_2()
                                .cursor_pointer()
                                .on_click(move |_event, _window, _cx| {
                                    println!("clicked Item {item:?}");
                                })
                                .child(format!("Item {item}")),
                        );
                    }
                    items
                }),
            )
            .h_full(),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds),
            ),
            titlebar: Some(TitlebarOptions { title: Some("GIT GUI by Amaan".into()), appears_transparent: false, traffic_light_position: None }),
            focus: true, 
            show: true,
            kind: WindowKind::Normal,
            is_movable: true, 
            is_minimizable: true, 
            app_id: Some("1".into()),
            display_id: None,
            window_background: gpui::WindowBackgroundAppearance::Opaque,
            is_resizable: true, 
            window_min_size: Some(size(px(400.0), px(300.0))),
            window_decorations: Some(WindowDecorations::Client),
            tabbing_identifier: None,
            },
            |_, cx| cx.new(|_| UniformListExample {}),
        )
        .unwrap();
    });
}