from pathlib import Path

import numpy as np
import pandas as pd
import pyvista as pv


def convert_csv_vtk(file_path: Path, value_key="index"):
    # Load CSV data
    data = pd.read_csv(file_path)

    # Extract unique coordinates and compute dimensions
    x_unique = np.unique(data["x"])
    y_unique = np.unique(data["y"])
    z_unique = np.unique(data["z"])

    nx, ny, nz = len(x_unique), len(y_unique), len(z_unique) if len(z_unique) > 1 else 1

    # Generate structured grid points
    x = np.linspace(x_unique.min(), x_unique.max(), nx)
    y = np.linspace(y_unique.min(), y_unique.max(), ny)
    z = np.linspace(z_unique.min(), z_unique.max(), nz)

    xx, yy, zz = np.meshgrid(x, y, z, indexing="ij")
    points = np.c_[xx.ravel(), yy.ravel(), zz.ravel()]

    # Map the index values to the structured grid
    grid = pv.StructuredGrid()
    grid.points = points
    grid.dimensions = (nx, ny, nz)

    # Reshape index values to match the grid
    index_values = data[value_key].to_numpy().reshape((nx, ny, nz), order="F")
    grid[value_key] = index_values.ravel(order="F")  # Add as scalar data

    # Save the VTK file
    grid.save(file_path.parent / "geometry.vtk")

    print(f"Structured grid created: {nx}x{ny}x{nz}")
