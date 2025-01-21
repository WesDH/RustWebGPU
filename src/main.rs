use glfw::{fail_on_errors, Action, Context, Key, Window, WindowEvent, WindowMode};

// Holds everything needed for gpu programming
struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
}


fn main() {
    let mut glfw = glfw::init(fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "It's WGPU time!", WindowMode::Windowed)
        .unwrap();

    window.set_key_polling(true);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
        window.swap_buffers();
    }
}
