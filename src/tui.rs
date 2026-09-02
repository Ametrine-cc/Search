use miamore::{
    self, clear_origin, colors_t, draw_border, draw_shape, draw_text, init_miamore, input,
    manage_cursor, position_t, set_fg,
};

// use crate::set_error;

pub fn tui_view(_err: &str) {
    init_miamore(true, true);
    draw_border("Search", miamore::theme_t::thick_l);

    manage_cursor(miamore::cursor_t::move_, Some(position_t { x: 3, y: 4 }));
    draw_text("Hello from Ametrine!");

    set_fg(colors_t::red);
    draw_shape(
        miamore::shape_t::rect,
        miamore::ShapeOptions {
            theme: (miamore::theme_t::thick_l),
            dimensions: (miamore::dimensions_t {
                width: (40),
                height: (20),
            }),
            position: (position_t { x: 3, y: 5 }),
        },
    );

    let mut runtime: bool = true;
    while runtime {
        let input = input();
        if input == b'q' as i32 {
            clear_origin();
            runtime = false;
        }
    }

    // let err = "error here";
    // set_error(err);
}
