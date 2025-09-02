// Don't treat my app like a terminal. you son of a rust.
#![windows_subsystem = "windows"]
use wry::application::event::{Event, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pal Labs App")
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url("https://palhandler123forwebview.free.nf/BoldExperimental.html?i=1")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}
