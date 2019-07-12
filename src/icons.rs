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

pub fn back(pixels: &mut [crate::footile::Rgba8], width: u16, graphic_width: u16) {
    render_from_rvg(BACK, pixels, width, graphic_width / 2)
}
