use magma_gfx::init_renderer;
use magma_gfx::shapes::Rectangle;
use magma_gfx::core::Color;

fn main() {
    let (gfx, mut window) = init_renderer();
    let size = winit::dpi::LogicalSize::new(1200, 800);
    window = window.with_inner_size(size).with_title("Testing vulkan");
    let rect = Rectangle {
        position: cgmath::Vector2 {
            x: 4.,
            y: 5.
        },
        width: 50,
        height: 50
    };

    let arr: [u8; 3] = [150, 155, 250];

    
    gfx.run(window, move |draw| {
        draw.rectangle(&rect, Color::new(arr[0], arr[1], arr[2]));
    });
}

