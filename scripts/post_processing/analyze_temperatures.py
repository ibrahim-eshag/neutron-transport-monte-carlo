from datetime import datetime

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

# df = pd.read_csv(Path(r"D:\Desktop\nuclear-rust\results\geometry\neutron_bins.csv"))

# print(df["fission_count"].sum())
# print(df["fission_count"].describe())

df = pd.read_csv("results/heat_diffusion/temperature_data.csv")

sns.lineplot(x="time", y="mean_temperature", data=df, label="Mean")
sns.lineplot(x="time", y="maximum_temperature", data=df, label="Maximum")
plt.xlabel("Time (s)")
plt.ylabel("Temperature (K)")
plt.title("Fuel plate temperature - cooling with 573.15 K, 10.000 W/m$^2$")
plt.savefig(
    f"figures/general/{datetime.now().strftime('%d%m%Y - ')}Neutron Monte Carlo - plate temperatures at around 23 MW.png",
    dpi=300,
)
plt.show()
