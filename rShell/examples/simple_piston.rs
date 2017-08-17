extern crate piston_window;
extern crate find_folder;

use self::piston_window::*;

fn main(){
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 420;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("All Widgets - Piston Backend", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2) // If not working, try `OpenGL::V2_1`.
            //.samples(4)
            .exit_on_esc(true)
            // .vsync(true)
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            });
        let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    // let ref font = assets.join("FiraSans-Regular.ttf");
    let ref font = assets.join("simhei.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 8).draw(
                "Hello world!你好",
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }

}