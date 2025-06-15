import matplotlib.pyplot as plt
import pandas as pd

df = pd.read_csv("results/power_data.csv")
first_runs = df[df["power"] < 20]

for unique_time in first_runs["halt_time"].unique():
    plt.hist(
        first_runs[first_runs["halt_time"] == unique_time]["power"],
        label=str(unique_time) + "s, 20k neutrons",
        alpha=0.8,
    )

second_runs = df[(df["power"] > 20) & (df["power"] < 35)]

for unique_time in second_runs["halt_time"].unique():
    plt.hist(
        second_runs[second_runs["halt_time"] == unique_time]["power"],
        label=str(unique_time) + "s, 40k neutrons",
        alpha=0.8,
    )


third_runs = df[(df["power"] > 35) & (df["power"] < 80)]
for unique_time in third_runs["halt_time"].unique():
    plt.hist(
        third_runs[third_runs["halt_time"] == unique_time]["power"],
        label=str(unique_time) + "s, 60k neutrons",
        alpha=0.8,
    )

fourth_runs = df[(df["power"] > 80) & (df["power"] < 150)]
for unique_time in fourth_runs["halt_time"].unique():
    plt.hist(
        fourth_runs[fourth_runs["halt_time"] == unique_time]["power"],
        label=str(unique_time) + "s, 120k neutrons",
        alpha=0.8,
    )

plt.legend()
plt.xlabel("Estimated power (W)")
plt.ylabel("Counts")
plt.title("Reactor power")

plt.savefig(
    "figures/07042024 - Neutron Monte Carlo - power estimates - different runtimes.png",
    dpi=300,
)
plt.show()

print(first_runs["power"].describe())
