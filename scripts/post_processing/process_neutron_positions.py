import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

from utils import get_latest_df, get_latest_folder


def plot_histogram(position_array):
    radius = np.linalg.norm(position_array, axis=1)

    plt.hist(x=radius, bins=100)
    plt.xlabel("Radius (m)")
    plt.ylabel("Neutron count")
    plt.title("Neutron radial position")
    plt.savefig(
        "figures/13122023 - Neutron Monte Carlo - neutron radial position histogram at 8 cm.png",
        dpi=300,
    )
    plt.show()


def main():
    latest_folder = get_latest_folder("results/diagnostics")
    df = pd.read_csv(latest_folder / "position_results.csv")

    print(df.describe())

    radius = np.linalg.norm(df.values, axis=1)

    print(radius.max())
    print(radius.mean())
    print(radius.min())

    x = df["x"]
    y = df["y"]
    z = df["z"]

    # fig, (ax1, ax2, ax3) = plt.subplots(nrows=1, ncols=3, figsize=(10, 10))
    # fig, ax1 = plt.subplots(nrows=1, ncols=1)

    plt.hexbin(x, y, cmap="jet", gridsize=1000, bins="log")
    # hb2 = ax2.hexbin(x, z, cmap="jet")
    # hb3 = ax3.hexbin(y, z, cmap="jet")

    # # colorbar = fig.colorbar(
    # #     hb1, ax=[ax1, ax2, ax3], orientation="horizontal", pad=-0.5, shrink=0.6
    # # )

    # colorbar = fig.colorbar(
    #     hb1, ax=[ax1], orientation="horizontal", pad=-0.5, shrink=0.6
    # )

    # colorbar.set_label("Neutron count")

    plt.title("Critical core")

    plt.xlabel("x (m)")
    plt.ylabel("y (m)")
    # ax1.set_aspect("equal")

    # ax2.set_xlabel("x (m)")
    # ax2.set_ylabel("z (m)")
    # ax2.set_aspect("equal")

    # ax3.set_xlabel("y (m)")
    # ax3.set_ylabel("z (m)")
    # ax3.set_aspect("equal")

    plt.xlim([-0.2, 0.8])
    plt.ylim([-0.5, 0.5])

    plt.tight_layout()

    plt.savefig(
        "figures/13122023 - Nuclear Monte Carlo - hexbin nuclear reactor core - improved run, no cbar.png",
        dpi=300,
    )
    plt.show()


if __name__ == "__main__":
    main()
