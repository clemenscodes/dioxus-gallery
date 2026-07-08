/// A named viewport size for the preview iframe. Each default preset sits
/// squarely inside one responsive band, so selecting a preset triggers exactly
/// that band's rules inside the iframe. The default bands mirror the consuming
/// app's breakpoints: mobile `< 768`, tablet `768–1279`, laptop `1280–1919`,
/// desktop `1920–2559`, qhd `2560–3839`, uhd `>= 3840`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ViewportPreset {
    label: &'static str,
    width: u32,
    height: u32,
}

impl ViewportPreset {
    pub const fn new(label: &'static str, width: u32, height: u32) -> Self {
        Self {
            label,
            width,
            height,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn defaults() -> Vec<Self> {
        let mobile = Self::new("Mobile", 390, 844);
        let tablet = Self::new("Tablet", 1024, 768);
        let laptop = Self::new("Laptop", 1440, 900);
        let desktop = Self::new("Desktop", 1920, 1080);
        let qhd = Self::new("QHD", 2560, 1440);
        let uhd = Self::new("UHD", 3840, 2160);
        vec![mobile, tablet, laptop, desktop, qhd, uhd]
    }
}
