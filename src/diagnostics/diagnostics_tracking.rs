use crate::diagnostics::geometry_diagnostics::GeometryDiagnostics;
use crate::diagnostics::halt_causes::SimulationHaltCauses;
use crate::utils::vectors::Vec3D;
use log::info;

use crate::diagnostics::{BinData, NeutronDiagnostics};

impl BinData {
    // Function to add two BinData instances
    pub fn add(&mut self, other: &BinData) {
        self.neutron_count += other.neutron_count;
        self.fission_count += other.fission_count;
    }
}

impl NeutronDiagnostics {
    pub fn new(
        estimate_k: bool,
        track_bins: bool,
        track_mean_free_path: bool,
        track_fission_positions: bool,
        track_from_generation: i64,
        bin_parameters: GeometryDiagnostics,
        initial_neutron_count: i64,
    ) -> NeutronDiagnostics {
        let bin_data = BinData::default();

        let neutron_position_bins = vec![
            bin_data;
            (bin_parameters.length_count + 1)
                * (bin_parameters.depth_count + 1)
                * (bin_parameters.height_count + 1)
        ];
        let neutron_position_bins_previous = neutron_position_bins.clone();

        let convergence_tracking = Vec::<(i64, f64)>::new();
        let neutron_travel_distance = Vec::<f64>::new();

        let previous_bin_generation = 0;

        let max_generation_value = 0;
        let averaged_k = 0.0;
        let halt_cause = SimulationHaltCauses::default();
        let total_neutrons_tracked = 0;
        let total_fissions = 0;
        let average_power: f64 = 0.0;
        let total_energy: f64 = 0.0;

        NeutronDiagnostics {
            neutron_generation_counts: Vec::<i64>::new(),
            neutron_travel_distance,
            bin_parameters,
            neutron_position_bins,
            estimate_k,
            track_bins,
            track_mean_free_path,
            max_generation_value,
            averaged_k,
            halt_cause,
            initial_neutron_count,
            total_neutrons_tracked,
            total_fissions,
            track_from_generation,
            power_generated: average_power,
            neutron_fission_locations: Vec::<Vec3D>::new(),
            track_fission_positions,
            total_energy,
            neutron_position_bins_previous,
            previous_bin_generation,
            convergence_tracking,
        }
    }

    pub fn get_total_fissions(&self) -> i64 {
        self.neutron_fission_locations.len() as i64
    }

    pub fn get_current_bin(&self, neutron_position: Vec3D) -> Option<usize> {
        self.bin_parameters.get_current_bin(neutron_position)
    }

    pub fn track_neutron_location_fission(
        &mut self,
        generation_number: i64,
        neutron_position: Vec3D,
    ) {
        if self.track_fission_positions && generation_number >= self.track_from_generation {
            self.neutron_fission_locations.push(neutron_position);
        }
    }

    pub fn track_neutron_bin_presence(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].neutron_count += 1
            }
        }
    }

    pub fn track_neutron_bin_fission(&mut self, generation_number: i64, neutron_position: Vec3D) {
        if self.track_bins && generation_number >= self.track_from_generation {
            if let Some(current_bin) = self.get_current_bin(neutron_position) {
                self.neutron_position_bins[current_bin].fission_count += 1
            }
        }
    }

    pub fn track_neutron_travel_distance(
        &mut self,
        generation_number: i64,
        neutron_creation_position: Vec3D,
        neutron_position: Vec3D,
    ) {
        if self.track_mean_free_path && generation_number >= self.track_from_generation {
            self.neutron_travel_distance.push(
                neutron_creation_position
                    .euclidean_distance_squared(&neutron_position)
                    .sqrt(),
            )
        }

        // if neutron_creation_position
        //     .dot(neutron_position)
        //     .abs()
        //     .sqrt()
        //     .is_nan()
        // {
        //     println!(
        //         "First position: {}. Second position: {}. Distance squared: {}. Distance: {}",
        //         neutron_creation_position,
        //         neutron_position,
        //         neutron_creation_position.dot(neutron_position),
        //         neutron_creation_position.dot(neutron_position).sqrt(),
        //     );
        // }
    }

    pub fn update_convergence(&mut self, current_generation: i64) {
        let current_neutron_count = self
            .neutron_position_bins
            .iter()
            .map(|current_bin| current_bin.neutron_count)
            .sum::<i64>()
            .max(1) as f64;

        let previous_neutron_count = self
            .neutron_position_bins_previous
            .iter()
            .map(|current_bin| current_bin.neutron_count)
            .sum::<i64>()
            .max(1) as f64;

        /* The idea is that the convergence has to be:
        1. Interpretable: the values have to be meaningful (unlike KL-divergence).
        2. Initially close to 1.0 then tending to 0.0.
        3. Independent of the grid or the number of neutrons.

        I think this current version does that okay-ish. The number of neutrons is independent of the number of bins, so we don't actually need to normalize by that.
        We don't sample more or less with more or fewer bins; we simply place all the neutrons within a certain volume in a bin. Therefore, normalizing over just the
        total number of neutrons in each step is already enough.
         */
        let convergence: f64 = self
            .neutron_position_bins
            .iter()
            .zip(self.neutron_position_bins_previous.iter())
            .map(|(current_bin, previous_bin)| {
                (current_bin.neutron_count as f64 / current_neutron_count
                    - previous_bin.neutron_count as f64 / previous_neutron_count)
                    .abs()
            })
            .sum::<f64>();

        self.convergence_tracking
            .push((current_generation, convergence));

        // Update the old ones with the new set
        self.neutron_position_bins_previous = self.neutron_position_bins.clone();
    }

    pub fn track_simulation_halt(
        &mut self,
        neutron_generation: i64,
        neutron_generation_history: Vec<i64>,
        halt_cause: SimulationHaltCauses,
    ) {
        self.neutron_generation_counts = neutron_generation_history;
        self.total_neutrons_tracked = self.neutron_generation_counts.iter().sum();
        self.halt_cause = halt_cause;
        self.max_generation_value = neutron_generation;

        info!("Simulation halted: {}", self.halt_cause);
    }
}
