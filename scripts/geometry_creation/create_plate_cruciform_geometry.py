import os
from itertools import product
from pathlib import Path

import numpy as np
from components import control_plate, fuel_plate
from vtk_conversion import convert_csv_vtk

from utils import clear_geometry, write_geometry


def cruciform_control_rod(x, y, z, width, thickness, height):
    template = ""
    template += control_plate(
        x=x, y=y, z=z, width=width, depth=thickness, height=height
    )
    template += control_plate(
        x=x, y=y, z=z, width=thickness, depth=width, height=height
    )
    return template


def main():
    geometry_path = Path("config/geometries/cruciform_rods.toml")
    csv_path = Path("results/geometry/geometry.csv")

    clear_geometry(geometry_path=geometry_path)

    template = ""

    control_rod_x_range = np.arange(start=-2.0, stop=2.0, step=0.25)
    control_rod_y_range = np.arange(start=-2.0, stop=2.0, step=0.25)

    fuel_plate_y_range = np.linspace(start=0.025, stop=0.25 - 0.025, num=6)

    print(fuel_plate_y_range)

    max_control_x = control_rod_x_range[-1]
    max_control_y = control_rod_y_range[-1]

    for control_x, control_y in product(control_rod_x_range, control_rod_y_range):
        # Add cruciform control rod
        template += cruciform_control_rod(
            x=control_x,
            y=control_y,
            z=1.30,
            width=0.25,
            thickness=0.02,
            height=2.0,
        )

        # Skip the fuel plate addition if at the grid's right or top edges
        if control_x < max_control_x and control_y < max_control_y:
            for fuel_y in fuel_plate_y_range:
                template += fuel_plate(
                    x=control_x + 0.125,
                    y=control_y + fuel_y,
                    z=0.0,
                    width=0.15,
                    depth=0.005,
                    height=1.5,
                )

    write_geometry(geometry_path=geometry_path, geometry_string=template)

    os.system("cargo run --release")

    convert_csv_vtk(csv_path)


if __name__ == "__main__":
    main()

    # convert_csv_vtk(
    #     Path(
    #         r"D:\Desktop\nuclear-rust\results\diagnostics\runs\Cruciform rods - 2025-01-19_14-49-40.015022900\neutron_positions.csv"
    #     ),
    #     value_key="neutron_count",
    # )
