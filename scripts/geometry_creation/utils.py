from pathlib import Path


def clear_geometry(geometry_path: Path):
    with open(geometry_path, "w") as _:
        pass


def write_geometry(geometry_path: Path, geometry_string: str):
    with open(geometry_path, "a") as f:
        f.write(geometry_string)
        f.write("\n\n")
