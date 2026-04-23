use crate::models::{
    material::{catalog, Category, Material},
    world::World,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivePanel {
    Simulation,
    Materials,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialsFocus {
    Categories,
    Items,
}

pub struct App {
    pub world: World,
    pub materials: Vec<Material>,
    pub active_panel: ActivePanel,
    pub materials_focus: MaterialsFocus,
    pub selected_category: usize,
    pub selected_in_category: usize,
    pub selected_material: usize,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let materials = catalog();
        let selected_material = materials
            .iter()
            .position(|m| m.name == "Water")
            .unwrap_or(0);
        let selected_category = Self::category_index(materials[selected_material].category);

        Self {
            world: World::new(width, height),
            materials,
            active_panel: ActivePanel::Simulation,
            materials_focus: MaterialsFocus::Categories,
            selected_category,
            selected_in_category: 0,
            selected_material,
            cursor_x: width / 2,
            cursor_y: height / 2,
        }
    }

    pub fn categories() -> [Category; 4] {
        [
            Category::Conductive,
            Category::Liquid,
            Category::Granular,
            Category::Solid,
        ]
    }

    pub fn category_index(category: Category) -> usize {
        match category {
            Category::Conductive => 0,
            Category::Liquid => 1,
            Category::Granular => 2,
            Category::Solid => 3,
        }
    }

    pub fn materials_in_category_indices(&self, category_index: usize) -> Vec<usize> {
        let categories = Self::categories();
        let category = categories[category_index];

        self.materials
            .iter()
            .enumerate()
            .filter_map(|(idx, material)| (material.category == category).then_some(idx))
            .collect()
    }

    pub fn move_cursor(&mut self, dx: isize, dy: isize) {
        let max_x = self.world.width.saturating_sub(1) as isize;
        let max_y = self.world.height.saturating_sub(1) as isize;

        let x = (self.cursor_x as isize + dx).clamp(0, max_x);
        let y = (self.cursor_y as isize + dy).clamp(0, max_y);

        self.cursor_x = x as usize;
        self.cursor_y = y as usize;
    }

    pub fn spawn_selected(&mut self) {
        let _ = self
            .world
            .spawn_at(self.cursor_x, self.cursor_y, self.selected_material);
    }

    pub fn clear_world(&mut self) {
        self.world.clear();
    }

    pub fn enter_category(&mut self) {
        self.materials_focus = MaterialsFocus::Items;
        self.selected_in_category = 0;
        self.sync_selected_material_with_category();
    }

    pub fn back_to_categories(&mut self) {
        self.materials_focus = MaterialsFocus::Categories;
        self.sync_category_with_selected_material();
    }

    pub fn select_next_category(&mut self) {
        let len = Self::categories().len();
        if len == 0 {
            return;
        }

        self.selected_category = (self.selected_category + 1) % len;
        self.selected_in_category = 0;
    }

    pub fn select_prev_category(&mut self) {
        let len = Self::categories().len();
        if len == 0 {
            return;
        }

        self.selected_category = if self.selected_category == 0 {
            len - 1
        } else {
            self.selected_category - 1
        };
        self.selected_in_category = 0;
    }

    pub fn select_next_material(&mut self) {
        let items = self.materials_in_category_indices(self.selected_category);
        if items.is_empty() {
            return;
        }

        self.selected_in_category = (self.selected_in_category + 1) % items.len();
        self.selected_material = items[self.selected_in_category];
    }

    pub fn select_prev_material(&mut self) {
        let items = self.materials_in_category_indices(self.selected_category);
        if items.is_empty() {
            return;
        }

        self.selected_in_category = if self.selected_in_category == 0 {
            items.len() - 1
        } else {
            self.selected_in_category - 1
        };
        self.selected_material = items[self.selected_in_category];
    }

    pub fn select_current_material(&mut self) {
        let items = self.materials_in_category_indices(self.selected_category);
        if items.is_empty() {
            return;
        }

        let index = self.selected_in_category.min(items.len() - 1);
        self.selected_material = items[index];
        self.sync_category_with_selected_material();
    }

    fn sync_category_with_selected_material(&mut self) {
        let material = self.materials[self.selected_material];
        self.selected_category = Self::category_index(material.category);

        let items = self.materials_in_category_indices(self.selected_category);
        self.selected_in_category = items
            .iter()
            .position(|&idx| idx == self.selected_material)
            .unwrap_or(0);
    }

    fn sync_selected_material_with_category(&mut self) {
        let items = self.materials_in_category_indices(self.selected_category);
        if let Some(&idx) = items.get(self.selected_in_category.min(items.len().saturating_sub(1))) {
            self.selected_material = idx;
        }
    }
}