use std::collections::HashMap;

use bevy::asset::HandleId;

use lib::entity::voxel::Voxel;

pub struct NamedMaterials {
    materials: HashMap<String, HandleId>,
}

impl NamedMaterials {
    pub fn empty() -> NamedMaterials {
        NamedMaterials {
            materials: HashMap::new()
        }
    }

    pub fn get(&self, material_name: &str) -> Option<&HandleId> {
        self.materials.get(material_name)
    }

    pub fn add(&mut self, name: String, handle_id: HandleId) {
        self.materials.insert(name, handle_id);
    }
}

pub fn generate_name_for_voxels(voxels: &[Vec<&Voxel>]) -> String {
    voxels.iter()
        .map(|row| {
            row.iter()
                .map(|v| {
                    (v.material as u8).to_string()
                })
                .collect::<Vec<String>>()
                .join(".")
        })
        .collect::<Vec<String>>()
        .join("|")
}