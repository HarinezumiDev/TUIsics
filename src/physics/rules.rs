use crate::models::{
    material::{Category, Material},
    world::World,
};

pub fn step(world: &mut World, materials: &[Material], tick: u64) {
    let mut moved = vec![false; world.cells.len()];

    for y in (0..world.height).rev() {
        if tick & 1 == 0 {
            for x in 0..world.width {
                step_cell(world, materials, &mut moved, x, y, tick);
            }
        } else {
            for x in (0..world.width).rev() {
                step_cell(world, materials, &mut moved, x, y, tick);
            }
        }
    }
}

fn step_cell(
    world: &mut World,
    materials: &[Material],
    moved: &mut [bool],
    x: usize,
    y: usize,
    tick: u64,
) {
    let idx = world.idx(x, y);
    if moved[idx] {
        return;
    }

    let cell = match world.cells[idx] {
        Some(cell) => cell,
        None => return,
    };

    let moved_now = match materials[cell.material].category {
        Category::Liquid => liquid_step(world, materials, moved, x, y, tick, cell.material),
        Category::Granular => granular_step(world, materials, moved, x, y, tick, cell.material),
        Category::Conductive | Category::Solid => false,
    };

    if !moved_now {
        moved[idx] = true;
    }
}

fn liquid_step(
    world: &mut World,
    materials: &[Material],
    moved: &mut [bool],
    x: usize,
    y: usize,
    tick: u64,
    current: usize,
) -> bool {
    if try_move(world, materials, moved, x, y, x as isize, y as isize + 1, current, true) {
        return true;
    }

    let left_first = ((x as u64 + y as u64 + tick) & 1) == 0;
    let diagonals = if left_first {
        [(-1, 1), (1, 1)]
    } else {
        [(1, 1), (-1, 1)]
    };

    for (dx, dy) in diagonals {
        if try_move(
            world,
            materials,
            moved,
            x,
            y,
            x as isize + dx,
            y as isize + dy,
            current,
            true,
        ) {
            return true;
        }
    }

    let viscosity = materials[current].viscosity.max(1) as u64;
    if tick % viscosity != 0 {
        return false;
    }

    let sides = if left_first { [-1, 1] } else { [1, -1] };

    for dx in sides {
        if try_move(
            world,
            materials,
            moved,
            x,
            y,
            x as isize + dx,
            y as isize,
            current,
            false,
        ) {
            return true;
        }
    }

    false
}

fn granular_step(
    world: &mut World,
    materials: &[Material],
    moved: &mut [bool],
    x: usize,
    y: usize,
    tick: u64,
    current: usize,
) -> bool {
    if try_move(world, materials, moved, x, y, x as isize, y as isize + 1, current, true) {
        return true;
    }

    let left_first = ((x as u64 + y as u64 + tick) & 1) == 0;
    let diagonals = if left_first {
        [(-1, 1), (1, 1)]
    } else {
        [(1, 1), (-1, 1)]
    };

    for (dx, dy) in diagonals {
        if try_move(
            world,
            materials,
            moved,
            x,
            y,
            x as isize + dx,
            y as isize + dy,
            current,
            true,
        ) {
            return true;
        }
    }

    false
}

fn try_move(
    world: &mut World,
    materials: &[Material],
    moved: &mut [bool],
    from_x: usize,
    from_y: usize,
    to_x: isize,
    to_y: isize,
    current: usize,
    allow_swap: bool,
) -> bool {
    if to_x < 0 || to_y < 0 {
        return false;
    }

    let tx = to_x as usize;
    let ty = to_y as usize;

    if !world.in_bounds(tx, ty) {
        return false;
    }

    let target_idx = world.idx(tx, ty);

    match world.cells[target_idx] {
        None => {
            world.swap((from_x, from_y), (tx, ty));
            moved[target_idx] = true;
            true
        }
        Some(target_cell) if allow_swap && can_displace(materials, current, target_cell.material) => {
            world.swap((from_x, from_y), (tx, ty));
            moved[target_idx] = true;
            true
        }
        _ => false,
    }
}

fn can_displace(materials: &[Material], current: usize, target: usize) -> bool {
    let current_material = &materials[current];
    let target_material = &materials[target];

    matches!(
        target_material.category,
        Category::Liquid | Category::Granular
    ) && current_material.density > target_material.density
}