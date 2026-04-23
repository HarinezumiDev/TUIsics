use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Conductive,
    Liquid,
    Granular,
    Solid,
}

impl Category {
    pub fn title(self) -> &'static str {
        match self {
            Category::Conductive => "electro-conductive",
            Category::Liquid => "liquids",
            Category::Granular => "granular",
            Category::Solid => "solids",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub name: &'static str,
    pub symbol: char,
    pub color: Color,
    pub category: Category,
    pub density: f32,
    pub viscosity: u8,
}

pub fn catalog() -> Vec<Material> {
    vec![
        Material {
            name: "Copper Wire",
            symbol: '=',
            color: Color::Yellow,
            category: Category::Conductive,
            density: 8.9,
            viscosity: 1,
        },
        Material {
            name: "Iron Wire",
            symbol: '=',
            color: Color::Gray,
            category: Category::Conductive,
            density: 7.9,
            viscosity: 1,
        },
        Material {
            name: "Water",
            symbol: '~',
            color: Color::Rgb(80, 170, 255),
            category: Category::Liquid,
            density: 1.0,
            viscosity: 1,
        },
        Material {
            name: "Oil",
            symbol: '~',
            color: Color::LightYellow,
            category: Category::Liquid,
            density: 0.8,
            viscosity: 4,
        },
        Material {
            name: "Sand",
            symbol: ':',
            color: Color::Yellow,
            category: Category::Granular,
            density: 1.6,
            viscosity: 1,
        },
        Material {
            name: "Gravel",
            symbol: '.',
            color: Color::Gray,
            category: Category::Granular,
            density: 2.1,
            viscosity: 1,
        },
        Material {
            name: "Stone",
            symbol: '#',
            color: Color::White,
            category: Category::Solid,
            density: 2.7,
            viscosity: 1,
        },
        Material {
            name: "Wood",
            symbol: 'H',
            color: Color::Rgb(150, 100, 60),
            category: Category::Solid,
            density: 0.6,
            viscosity: 1,
        },
    ]
}