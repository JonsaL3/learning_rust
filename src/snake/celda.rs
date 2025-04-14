#[derive(Clone)]
#[derive(Debug)]
pub struct Celda {
    r_color: u8,
    g_color: u8,
    b_color: u8
}

impl Celda {
    /**
     * Constructor (?)
     */
    pub fn new(r_color: u8, g_color: u8, b_color: u8) -> Self {
        Celda { r_color, g_color, b_color }
    }

    /**
     * Getters
     */
    pub fn r_color(&self) -> u8 {
        self.r_color
    }
    
    pub fn g_color(&self) -> u8 {
        self.g_color
    }

    pub fn b_color(&self) -> u8 {
        self.b_color
    }
}