use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, size, uniform_list,
};
use gpui_component::{ActiveTheme, Root, button::Button};
struct UniformListExample {}

impl Render for UniformListExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Use theme colors for proper visibility
        let bg = cx.theme().background;
        let fg = cx.theme().foreground;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .text_color(fg)
            .child(
                uniform_list(
                    "entries",
                    50,
                    cx.processor(move |_this, range, _window, _cx| {
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
                .flex_1(), // Grow to fill available space, but don't push button off screen
            )
            .child(
                div()
                    .p_2() // Add some padding around the button
                    .child(
                        Button::new("my-button")
                            .label("my button")
                            .on_click(|_, _, _| println!("Button Clicked???")),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize gpui-component (Theme, etc.) before using any components
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("GIT GUI by Amaan".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| UniformListExample {});
                // Wrap in Root for proper gpui-component support
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
        .unwrap();
    });
}
