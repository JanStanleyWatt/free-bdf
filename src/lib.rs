#![cfg_attr(not(test), no_std)]

/// BDFフォント全体のデータを格納する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct Font<'a> {
    pub format_version: FontVersion,
    pub font_name: &'a str,
    pub font_size: FontSize,
    pub font_version: Option<usize>,
    pub metrics_set: Option<usize>,
    pub font_metrics: Option<FontMetrics>,
    pub char_count: usize,
    pub chars: &'a [Chars<'a>],
}
/// フォントに収録されたそれぞれの文字情報を定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct Chars<'a> {
    pub glyph: char,
    pub encode: u32,
    pub bbx: BBX,
    pub bitmap: &'a [u8],
    pub font_metrics: Option<FontMetrics>,
}

/// BDFフォントのバージョンを定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct FontVersion {
    pub major_version: usize,
    pub minor_version: usize,
}

/// フォントのポイントサイズ並びに縦横それぞれの解像度を定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct FontSize {
    pub point_size: usize,
    pub x_res: usize,
    pub y_res: usize,
}

/// フォントの幅、高さ、左上を頂点とした際の幅と高さそれぞれのオフセットを定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct FontBoundingBox {
    pub f_bbx: usize,
    pub f_bby: usize,
    pub x_off: isize,
    pub y_off: isize,
}

/// フォントが描画されるピクセルの幅、高さ、左上を頂点とした際の幅と高さそれぞれのオフセットを定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct BBX {
    pub bbw: usize,
    pub bbh: usize,
    pub bbx_off_0x: isize,
    pub bbx_off_0y: isize,
}

/// フォントにあるそれぞれのグリフの幅と高さのスケーラブルなサイズを定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct FontMetrics {
    pub s_width: usize,
    pub d_width: usize,
    pub s_width_1: Option<usize>,
    pub d_width_1: Option<usize>,
    pub v_vector: Option<VVector>,
}

/// フォントを描画する際のベクトルサイズを定義する構造体
#[derive(Debug, Copy, Clone, Hash)]
pub struct VVector {
    pub x: isize,
    pub y: isize,
}
