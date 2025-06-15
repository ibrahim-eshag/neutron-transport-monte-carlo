import matplotlib.pyplot as plt
import pandas as pd

from utils import get_latest_folder

latest_folder = get_latest_folder("results/diagnostics")
df = pd.read_csv(latest_folder / "neutron_energies.csv")

plt.plot(df["timestamp"], df["neutron_energy"])
plt.loglog()
# plt.semilogy()
plt.xlabel("Time (s)")
plt.ylabel("Neutron energy (eV)")
plt.title("Neutron moderation in water")
plt.savefig(
    "figures/28122023 - Neutron Monte Carlo - neutron moderation in light water.png",
    dpi=300,
)
plt.show()
