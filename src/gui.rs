use crate::FontGroup;
use crate::Image;
use fonterator::PathOp::{self, *};

// Import entity component system for use.
// mod ecs;
// use self::ecs::*;

/*mod components {
    use fonterator::PathOp;

    #[derive(Clone)]
    pub struct Widget {
        pub(super) path: Vec<PathOp>,
        pub(super) text: String,
    }

    #[derive(Clone)]
    pub struct Press {
        pub(super) press: Option<fn()>,
    }

    #[derive(Clone)]
    pub struct Swipe {
        // Function executes if press and drag.  `edge` is true if the user started pressing near
        // the edge of the window.  `done` is true if the swipe was successful.  `amount` is
        // -1 for all the way up or left, 1 for all the way down or right.  0 for release and no
        //  swipe event. "All the way" refers to the length of the screen.
        pub(super) swipe: Option<fn(edge: bool, done: bool, x_amount: f32, y_amount: f32)>,
    }
}*/

/// Id for widget.
// #[derive(Copy, Clone)]
// pub struct Id(u32);

/*impl Into<u32> for Id {
    fn into(self) -> u32 {
        self.0
    }
}*/

/// A GUI (Graphical User Interface).
///
/// ```
/// fn generator(row: usize) -> Option<Widget> {
///     
/// }
/// // Build the GUI.
/// let mut gui = Gui::new();
/// // Set the non-scrolling header.
/// gui.header(generator);
/// // Set the scrolling page.
/// gui.page(generator);
/// // Set the scrolling sidebar.
/// gui.sidebar(generator);
/// // Set the non-scrolling centered popup.
/// gui.popup(generator);
/// // Set the
///
///
/// // Create an row of entities.
/// let header = gui.add(0x303030FF, Widget::Row);
/// ```
pub struct Gui<'a> {
    // The data for the entity component system.
    //    c_widget: Storage<components::Widget, Id>,
    //    c_press: Storage<components::Press, Id>,
    //    c_swipe: Storage<components::Swipe, Id>,

    // Page scroll amount
    scroll: u32,
    // GUI Size.
    size: u32,
    // Fast user input row height cache.  Start y and then start index.
    ydif_id: Vec<(u32, u32)>,
    //
    font: FontGroup<'a>,
}

impl<'a> Gui<'a> {
    /// Create a new GUI.  `color` is the background color of the first row.
    pub fn new(font: FontGroup<'a>) -> Gui<'a> {
        Gui {
            // Initially at top of page.
            scroll: 0,
            // 48 pixels default.
            size: 36,
            //
            ydif_id: vec![],
            // The font
            font,
            /*            c_widget: Storage::new(components::Widget {
                path: vec![],
                text: "".to_string(),
            }),
            c_press: Storage::new(components::Press {
                press: None,
            }),
            c_swipe: Storage::new(components::Swipe {
                swipe: None,
            }),
            ydif_id: Vec::new(),*/
        }
    }

    /// Redraw window head.  For `head`, the `generator` only gets called once.  This is because the
    /// head is always `gui_size` high.
    pub fn head<'b>(
        &mut self,
        image: &mut Image,
        buffer: &mut [u8],
        generator: &Fn(
            usize,
            &mut [u8; 5],
        )
            -> &'b [(&'b [([u8; 4], &'b [PathOp])], &'b str)],
    ) {
        let crate::Size(w, h) = image.size();
        let mut x = 0.0;
        let y = 0.0; // Not mutable because we only call generator once.
        let mut color = [0; 5];

        // Render From Generator.
        let iter = generator(0, &mut color);
        let bg = [color[0], color[1], color[2], color[3]];
        let fg = fgcolor_from_bg(bg);

        // Render Background.
        let shape = [
            Move(0.0, 0 as f32),
            Line(w as f32, 0 as f32),
            Line(w as f32, (self.size + 1) as f32),
            Line(0.0, (self.size + 1) as f32),
        ];
        image.fill(bg /*color*/, &shape /*path*/, buffer /**/);
        draw_window_border(image, buffer, (self.size) as f32, color[4]);

        // Render Slice
        for (g, t) in iter {
            // Iterate over columns in row.
            for p in g.iter() {
                // Iterate over paths in graphic.
                image.fill(
                    p.0, // color
                    p.1, // path
                    buffer,
                );
            }

            x += image
                .text(
                    fg,
                    (
                        x + (self.size as f32 * 0.125),
                        y + (self.size as f32 * 0.125),
                        self.size as f32 * 0.75,
                    ),
                    &self.font,
                    t,
                    buffer,
                )
                .0;
        }
    }

    /// Redraw window page.  For `page`, the `generator` only gets called infinitely because it can
    /// scroll.
    pub fn page<'b>(
        &mut self,
        image: &mut Image,
        buffer: &mut [u8],
        generator: &Fn(
            usize,
            &mut [u8; 5],
        )
            -> &'b [(&'b [([u8; 4], &'b [PathOp])], &'b str)],
    ) {
        let crate::Size(w, h) = image.size();
        let mut x = 0.0;
        let y = 0.0; // Not mutable because we only call generator once.
        let mut color = [0; 5];

        // Render From Generator.
        let iter = generator(0, &mut color);
        let bg = [color[0], color[1], color[2], color[3]];
        let fg = fgcolor_from_bg(bg);

        // Render Background.
        let shape = [
            Move(0.0, (self.size) as f32),
            Line(w as f32, (self.size) as f32),
            Line(w as f32, (2 * self.size) as f32),
            Line(0.0, (2 * self.size) as f32),
        ];
        image.fill(bg /*color*/, &shape /*path*/, buffer /**/);
        draw_window_border(image, buffer, (self.size * 2 - 1) as f32, color[4]);

        // Render Slice
        for (g, t) in iter {
            // Iterate over columns in row.
            for p in g.iter() {
                // Iterate over paths in graphic.
                image.fill(
                    p.0, // color
                    p.1, // path
                    buffer,
                );
            }

            // TODO: Increase X starting position for each column
            x += image
                .text(
                    fg,
                    (
                        x + (self.size as f32 * 0.125),
                        y + (self.size as f32 * 1.125),
                        self.size as f32 * 0.75,
                    ),
                    &self.font,
                    t,
                    buffer,
                )
                .0;
        }
    }
}

// Create foreground color from background.
fn fgcolor_from_bg(bg: [u8; 4]) -> [u8; 4] {
    let mut brightness = 0;
    for i in bg.iter() {
        brightness += *i as u32;
    }
    if brightness > 512 {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    }
}

// Draw the window border, and optional separator.
fn draw_window_border(
    image: &mut Image,
    buffer: &mut [u8],
    y: f32,
    separator: u8,
) {
    let crate::Size(w, h) = image.size();

    if separator == 0 {
        image.stroke(
            [0, 0, 0, 255], /*color*/
            &[
                PenWidth(2.0),
                Move(0.0, 0.0),
                Line(w as f32, 0.0),
                Line(w as f32, h as f32),
                Line(0.0, h as f32),
                Line(0.0, 0.0),
            ], /*path*/
            buffer,         /**/
        );
    } else {
        image.stroke(
            [0, 0, 0, 255], /*color*/
            &[
                PenWidth(2.0),
                Move(0.0, 0.0),
                Line(w as f32, 0.0),
                Line(w as f32, h as f32),
                Line(0.0, h as f32),
                Line(0.0, 0.0),
                Close(),
                PenWidth(0.5),
                Move(0.0, y),
                Line(w as f32, y),
                Close(),
            ], /*path*/
            buffer,         /**/
        );
    }
}
