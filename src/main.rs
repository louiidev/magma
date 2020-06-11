use magma_gfx::init_renderer;
use magma_gfx::shapes::Rectangle;
use magma_gfx::core::Color;
use magma_gfx::textures::Texture2D;

extern crate nalgebra_glm as glm;



fn main() {
    let (mut gfx, window) = init_renderer();
    // let size = winit::dpi::LogicalSize::new(1200, 800);
    let rect = Rectangle {
        position: nalgebra_glm::Vec2::new(0., 0.),
        width: 50,
        height: 50
    };

    let arr: [u8; 3] = [150, 155, 250];

    let mut texture = Texture2D::load(&mut gfx, "./test.png".to_string());

    
    gfx.run(window, move |draw| {
        draw.rectangle(&rect, Color::new(50, 50, 50));
        draw.texture_pro(&mut texture,  nalgebra_glm::Vec2::new(600., 400.), 5.);
    });
}

