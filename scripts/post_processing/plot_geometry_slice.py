import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.colors import BoundaryNorm, ListedColormap


def main():
    df = pd.read_csv("results/geometry/plane.csv")

    # Assuming df is your DataFrame and it has columns 'x', 'y', and 'material_index'

    # [void, u235, u238, fe54, be9]

    # Create a dictionary mapping numerical values to materials

    # Void, h1, be9, o16, fe54, u235, u238

    material_dict = {
        0: "Void",
        1: "H-1",
        2: "Be-9",
        3: "O-16",
        4: "Fe-54",
        5: "U-235",
        6: "U-238",
    }

    df["material_index_named"] = df["material_index"].map(material_dict)
    # df["material_index_num"] = pd.factorize(df["material_index"])[0]

    grid_x, grid_y = np.meshgrid(np.unique(df["x"]), np.unique(df["y"]))
    grid_z = df["material_index"].values.reshape(grid_x.shape)

    grid_x, grid_y, grid_z = grid_x.T, grid_y.T, grid_z.T

    plt.imshow(
        grid_z,
        origin="lower",
        aspect="auto",
        cmap="tab10",
        interpolation="none",
        extent=(grid_x.min(), grid_x.max(), grid_y.min(), grid_y.max()),
    )
    # plt.axis("equal")

    plt.xlabel("x (m)")
    plt.ylabel("y (m)")

    plt.xlim([-0.15, 0.65])
    plt.ylim([-0.4, 0.4])

    plt.title("Material slice")
    plt.savefig(
        "figures/19022024 - Neutron Monte Carlo - plate-type reactor - critical reactor.png",
        dpi=300,
    )
    plt.show()


if __name__ == "__main__":
    main()
