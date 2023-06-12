

pub enum Color {
    White,
    Black
}

impl Color {
    fn print(&self) {
        println!("Color Enum");
        match self {
            Color::White => println!("white"),
            Color::Black => println!("black"),
        }
    }
}

pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions struct");
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

pub struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

impl ShippingBox {
    // rustでは、インスタンスを作成するのに慣習的にnewを使う
    // new 関連関数（associated function）
    pub fn new(weight:f64, color:Color, dimensions: Dimensions ) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    pub fn print(&self) {
        self.color.print(); // Color の impl printメソッドにアクセス
        self.dimensions.print(); // 同様に dimensionsにも
        println!("{}", self.weight);
    }
}
