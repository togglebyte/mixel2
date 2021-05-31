use serde::Deserialize;

// -----------------------------------------------------------------------------
//     - Actions -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,

    CanvasLeft,
    CanvasRight,
    CanvasUp,
    CanvasDown,
    
    CanvasZoomIn,
    CanvasZoomOut,

    NextXPixel,
    PrevXPixel,
    NextYPixel,
    PrevYPixel,

    SplitViewportVert,
    SplitViewportHorz,

    Noop,
}
