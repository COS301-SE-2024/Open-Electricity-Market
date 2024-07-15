use crate::grid::generator::Generator;
use crate::grid::load::Load;
use crate::grid::transformer::Transformer;

enum GridEquipment {
    Generator(Generator),
    Load(Load),
    Transformer(Transformer),
}

struct Location {
    latitude: f32,
    longitude: f32,
}

struct At {
    grid_equipment: GridEquipment,
    location: Location,
}
