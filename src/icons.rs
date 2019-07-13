use rvg::*;

const BACK: &'static [u8] = include_bytes!("../rvg/back.svg.rvg");
const FULLSCREEN: &'static [u8] = include_bytes!("../rvg/fullscreen.svg.rvg");
const GRID: &'static [u8] = include_bytes!("../rvg/grid.svg.rvg");
const HIDE: &'static [u8] = include_bytes!("../rvg/hide.svg.rvg");
const MENU: &'static [u8] = include_bytes!("../rvg/menu.svg.rvg");
const MORE: &'static [u8] = include_bytes!("../rvg/more.svg.rvg");
const NEW: &'static [u8] = include_bytes!("../rvg/new.svg.rvg");
const NEXT: &'static [u8] = include_bytes!("../rvg/next.svg.rvg");
const SEARCH: &'static [u8] = include_bytes!("../rvg/search.svg.rvg");
const VIEW: &'static [u8] = include_bytes!("../rvg/view.svg.rvg");
const ZOOM_IN: &'static [u8] = include_bytes!("../rvg/zoom_in.svg.rvg");
const ZOOM_OUT: &'static [u8] = include_bytes!("../rvg/zoom_out.svg.rvg");

pub fn back(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    let margin = graphic_h / 8;
    let graphic_width = (graphic_h / 2) - (margin);
    let ad = (graphic_h / 2) - (margin / 2);

    render_from_rvg(BACK, pixels, width, margin + x * ad, margin, graphic_width)
}
