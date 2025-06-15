use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::geometry::parts::parts::PartTypes;
use crate::materials::material_data::MaterialData;
use crate::materials::material_properties::{
    map_enum_to_indices, MaterialNames, MaterialProperties,
};
use crate::utils::vectors::Vec3D;

/// Part composition for mixed materials.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PartComposition {
    pub material_name: MaterialNames,
    pub material_fraction: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vec3D,
    pub max: Vec3D,
}

impl BoundingBox {
    pub fn is_inside(&self, neutron_position: &Vec3D) -> bool {
        if neutron_position.x < self.min.x || neutron_position.x > self.max.x {
            return false;
        }
        if neutron_position.y < self.min.y || neutron_position.y > self.max.y {
            return false;
        }
        if neutron_position.z < self.min.z || neutron_position.z > self.max.z {
            return false;
        }
        true
    }

    /// Should only be used if we already ran the 'is_inside' already!
    #[inline(always)]
    pub fn distance(&self, p: &Vec3D) -> f64 {
        // For each axis: positive delta if we're outside, 0.0 if we're between the two faces
        let dx = if p.x < self.min.x {
            self.min.x - p.x
        } else {
            (p.x - self.max.x).max(0.0) // only positive when p.x > max.x
        };

        let dy = if p.y < self.min.y {
            self.min.y - p.y
        } else {
            (p.y - self.max.y).max(0.0)
        };

        let dz = if p.z < self.min.z {
            self.min.z - p.z
        } else {
            (p.z - self.max.z).max(0.0)
        };

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Squared distance variant – avoids the costly sqrt().
    /// Should only be used if we already ran the 'is_inside' already!
    #[inline(always)]
    pub fn distance_squared(&self, p: &Vec3D) -> f64 {
        let dx = if p.x < self.min.x {
            self.min.x - p.x
        } else {
            (p.x - self.max.x).max(0.0)
        };
        let dy = if p.y < self.min.y {
            self.min.y - p.y
        } else {
            (p.y - self.max.y).max(0.0)
        };
        let dz = if p.z < self.min.z {
            self.min.z - p.z
        } else {
            (p.z - self.max.z).max(0.0)
        };
        dx * dx + dy * dy + dz * dz
    }
}

/// Struct that contains the material data, cached properties and parts - essentially all the geometry.
#[derive(Debug)]
pub struct Components {
    pub material_data_vector: Vec<MaterialData>,
    pub cached_material_properties: Vec<MaterialProperties>,
    pub parts_vector: Vec<PartTypes>,
    pub material_properties_cache_initialized: bool,
    pub simulation_range_squared: f64,
    pub neutron_position_at_update: Vec3D,
    pub relevant_parts_index_cache: Vec<usize>,
    pub part_cache_maximum_distance_squared: f64,
    pub parts_cache_initialized: bool,
}

impl Components {
    pub fn new(
        material_data_vector: Vec<MaterialData>,
        parts_vector: Vec<PartTypes>,
        parts_cache_distance: f64,
    ) -> Self {
        let simulation_range_squared = f64::INFINITY;

        let material_property = MaterialProperties::default();
        let mut cached_material_properties: Vec<MaterialProperties> = Vec::new();

        for _ in material_data_vector.iter() {
            cached_material_properties.push(material_property.clone());
        }

        let material_properties_cache_initialized = false;
        let parts_cache_initialized = false;

        let relevant_parts_index_cache: Vec<usize> = Vec::default();
        let neutron_position_at_update: Vec3D = Vec3D::default();
        let part_cache_maximum_distance_squared = parts_cache_distance * parts_cache_distance; // squared distance

        Components {
            material_data_vector,
            parts_vector,
            cached_material_properties,
            material_properties_cache_initialized,
            simulation_range_squared,
            relevant_parts_index_cache,
            neutron_position_at_update,
            part_cache_maximum_distance_squared,
            parts_cache_initialized,
        }
    }

    /// Checks the squared Euclidean distance to all the parts, and if it's within the specified range, adds the index to the cached vector.
    pub fn check_parts_distances(&mut self, neutron_position: &Vec3D) {
        // Clearing out
        self.relevant_parts_index_cache.clear();

        for (part_index, part) in self.parts_vector.iter().enumerate() {
            // Checks each option in the enum and returns the matches.
            let (is_inside, min_part_distance) = match part {
                PartTypes::Sphere(sphere) => (
                    sphere.is_inside(neutron_position),
                    sphere.bounding_box.distance_squared(neutron_position),
                ),
                PartTypes::Cylinder(cylinder) => (
                    cylinder.is_inside(neutron_position),
                    cylinder.bounding_box.distance_squared(neutron_position),
                ),
                PartTypes::Cuboid(cuboid) => (
                    cuboid.is_inside(neutron_position),
                    cuboid.bounding_box.distance_squared(neutron_position),
                ),
            };

            // If we're inside: take it either way. If we're not inside: consider the minimum distance to the bounding box.
            if is_inside || min_part_distance <= self.part_cache_maximum_distance_squared {
                self.relevant_parts_index_cache.push(part_index)
            }
        }
    }

    /// Calculates the distance travelled
    pub fn update_parts_cache(&mut self, neutron_position: &Vec3D) {
        let distance_travelled =
            neutron_position.euclidean_distance_squared(&self.neutron_position_at_update);

        // info!("Part cache length: {}", self.parts_cache)

        if distance_travelled >= self.part_cache_maximum_distance_squared
            || !self.parts_cache_initialized
        {
            self.neutron_position_at_update = *neutron_position;
            self.check_parts_distances(neutron_position);
            self.parts_cache_initialized = true;
        }
    }

    /// Automatically calculates the maximum radius squared, beyond which the neutron is discarded.
    /// If this wasn't set correctly manually, it would mess up (if it's too small, part of the geometry would be ignored; too large, and the simulation becomes very slow if neutrons have to escape first).
    /// The code iterates over all the parts, skipping those with order <= -1, and determines the largest bounds.
    /// This function should be ran after creation of the simulation to set components.simulation_range_squared.
    pub fn get_maximum_radius_squared(&mut self) {
        let mut maximum_radius = 0.0;

        for part in &self.parts_vector {
            let (bounding_box, order) = match part {
                PartTypes::Sphere(sphere) => (&sphere.bounding_box, &sphere.order),
                PartTypes::Cylinder(cylinder) => (&cylinder.bounding_box, &cylinder.order),
                PartTypes::Cuboid(cuboid) => (&cuboid.bounding_box, &cuboid.order),
            };

            // To allow a large background using for example a cube (computationally efficient because no squaring for the radius), without having it included in the simulation range, if the order is specified as -1 or less, it will be skipped.
            if order <= &-1 {
                continue;
            }

            let coordinate_min_radius = bounding_box.min.norm_squared();
            let coordinate_max_radius = bounding_box.max.norm_squared();

            if coordinate_max_radius > maximum_radius {
                maximum_radius = coordinate_max_radius
            }
            if coordinate_min_radius > maximum_radius {
                maximum_radius = coordinate_min_radius
            }
        }

        self.simulation_range_squared = maximum_radius;
    }

    /// Updating the cache of material properties for the given neutron's energy.
    /// This should be done any time the neutron's energy changes significantly, or whenever the simulation starts.
    pub fn update_material_properties_cache(&mut self, neutron_energy: f64) {
        for (index, material_data) in self.material_data_vector.iter().enumerate() {
            self.cached_material_properties[index].get_properties(material_data, neutron_energy);
        }
        self.material_properties_cache_initialized = true;
    }

    /// Calculates the composition's total cross-section for a given neutron's energy from the cached properties.
    /// The total cross-section can then be used in the neutron dynamics to calculate interaction probabilities.
    /// If the cache is not updated, this will mess up.
    pub fn get_composition_total_cross_section(
        &self,
        part_composition_vector: &Vec<PartComposition>,
    ) -> f64 {
        let mut overall_total_cross_section: f64 = 0.0;

        debug_assert!(
            self.material_properties_cache_initialized,
            "Cache was not initialized!"
        );

        for part_composition in part_composition_vector {
            let material_name = part_composition.material_name;
            let material_index = map_enum_to_indices(&material_name);
            let material_composition = &self.cached_material_properties[material_index];

            overall_total_cross_section +=
                material_composition.total_cross_section() * part_composition.material_fraction;

            // debug!(
            //     "In the function: Material cross section for {:?}: {}",
            //     material_name,
            //     material_composition.total_cross_section()
            // );

            // debug!(
            //     "Material: {:?} - {}",
            //     material_name,
            //     material_composition.total_cross_section()
            // );
        }

        // debug!("Overall cross-section: {}", overall_total_cross_section);

        overall_total_cross_section
    }

    /// Sums each of the part's material composition vectors and ensures the fractions add up to 1. If not, it throws an error.
    /// This is probably a very error-prone operation, if we specify the parts manually.
    /// If we move to configuration files later, we can move this to integration tests.
    pub fn check_material_fractions_sum(&self) {
        for part in &self.parts_vector {
            let material_composition_vector = match part {
                PartTypes::Sphere(sphere) => &sphere.material_composition_vector,
                PartTypes::Cylinder(cylinder) => &cylinder.material_composition_vector,
                PartTypes::Cuboid(cuboid) => &cuboid.material_composition_vector,
            };

            let mut material_sum = 0.0;

            for material_composition in material_composition_vector {
                material_sum += material_composition.material_fraction;
            }

            assert!(
                material_sum == 1.0,
                "The material composition summed up to {:?} instead of 1.0 for part:\n{:?}",
                material_sum,
                part,
            );
        }
    }

    /// Determines which material from a given composition interacts with the neutron.
    /// This is mainly relevant for mixed materials, such as water (H-1/O-16), U-235/U-238 etc.
    pub fn select_material_from_composition(
        &self,
        rng: &mut rand::rngs::SmallRng,
        part_composition_vector: &Vec<PartComposition>,
        composition_total_cross_section: f64,
    ) -> usize {
        let material_selection_criterion = rng.gen::<f64>();
        let mut cumulative_probability = 0.0;

        debug_assert!(
            self.material_properties_cache_initialized,
            "Cache was not initialized!"
        );

        // debug!("Criterion: {}", material_selection_criterion);

        for part_composition in part_composition_vector.iter() {
            let material_name = part_composition.material_name;
            let material_index = map_enum_to_indices(&material_name);
            let material_composition = &self.cached_material_properties[material_index];
            let normalized_cross_section = material_composition.total_cross_section()
                * part_composition.material_fraction
                / composition_total_cross_section;

            // debug!(
            //     "Material cross section for {:?}: {}. After normalization: {}",
            //     material_name,
            //     material_composition.total_cross_section(),
            //     normalized_cross_section,
            // );

            // debug!(
            //     "Normalized cross-section {} for material {:?}",
            //     normalized_cross_section, material_name
            // );

            // debug!("Cumulative probability: {}", cumulative_probability);

            if material_selection_criterion >= cumulative_probability
                && material_selection_criterion < cumulative_probability + normalized_cross_section
            {
                let cached_material_vector_index = map_enum_to_indices(&material_name);

                // debug!(
                //     "Material: {:?}",
                //     self.cached_material_properties[cached_material_vector_index].name
                // );

                return cached_material_vector_index;
            }

            cumulative_probability += normalized_cross_section;

            assert!(
                cumulative_probability < 1.0,
                "Cumulative probability is {}",
                cumulative_probability
            );
        }

        // debug!("Total cross sections: {}", composition_total_cross_section);

        // The RNG generates \xi \in [0, 1), and we check 0 <= \xi x_i, with x_i the material fraction.
        // If something goes wrong, or if we are outside any material (i.e. when the specified material is Void), we return 0.
        0
    }

    /// Gets the material index based on the neutron's current position by checking each individual part and their order.
    /// The part with the highest order is selected, through constructive solid geometry.
    /// The actual isotope that is interacted with is then selected from a composite material and returned, together with the total cross-section.
    pub fn get_material_index(
        &self,
        rng: &mut rand::rngs::SmallRng,
        neutron_position: &Vec3D,
    ) -> (usize, f64) {
        let mut maximum_order = i32::MIN;

        let empty_reference_vector: Vec<PartComposition> = Vec::default();
        let mut max_part_composition_vector: &Vec<PartComposition> = &empty_reference_vector;

        // Iterating over all the different parts.
        for part_index in &self.relevant_parts_index_cache {
            let part = &self.parts_vector[*part_index];

            // for part in &self.parts_vector {
            // Checks each option in the enum and returns the matches.
            let (is_inside, order, material_composition_vector) = match part {
                PartTypes::Sphere(sphere) => (
                    sphere.is_inside(neutron_position),
                    sphere.order,
                    &sphere.material_composition_vector,
                ),
                PartTypes::Cylinder(cylinder) => (
                    cylinder.is_inside(neutron_position),
                    cylinder.order,
                    &cylinder.material_composition_vector,
                ),
                PartTypes::Cuboid(cuboid) => (
                    cuboid.is_inside(neutron_position),
                    cuboid.order,
                    &cuboid.material_composition_vector,
                ),
            };

            // Keeping track of the highest-order part for Constructive Solid Geometry.
            if order > maximum_order && is_inside {
                maximum_order = order;
                max_part_composition_vector = material_composition_vector;
            }
        }

        let composition_total_cross_section =
            self.get_composition_total_cross_section(max_part_composition_vector);

        // debug!("{}", composition_total_cross_section);

        let max_material_index = self.select_material_from_composition(
            rng,
            max_part_composition_vector,
            composition_total_cross_section,
        );

        // debug!("Max material index: {}", max_material_index);

        // let material_name = map_indices_to_enum(max_material_index);
        // info!("Material name: {:?}", max_material_index);

        (max_material_index, composition_total_cross_section)
    }

    /// Gets the material properties and total cross-section based on the neutron's current position.
    /// This requires the cache to have been updated beforehand.
    /// The function will throw an exception if this has not been done.
    pub fn get_material_properties(
        &self,
        rng: &mut rand::rngs::SmallRng,
        neutron_position: &Vec3D,
    ) -> (&MaterialProperties, f64) {
        // Ensuring everything is correctly initialized.
        debug_assert!(
            self.material_properties_cache_initialized,
            "Cache was not initialized!"
        );
        debug_assert!(
            self.simulation_range_squared > 0.0,
            "Simulation range is not set correctly."
        );
        debug_assert!(
            self.parts_cache_initialized,
            "Parts cache was not initialized!"
        );

        let (material_index, composition_total_cross_section) =
            self.get_material_index(rng, neutron_position);
        let material_properties = &self.cached_material_properties[material_index];

        (material_properties, composition_total_cross_section)
    }
}
