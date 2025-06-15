def fuel_plate(x, y, z, width, depth, height):
    template = """
[[cuboids]] # Fuel element
center = {{ x = {x}, y = {y}, z = {z} }}
width = {width}
depth = {depth}
height = {height}
material_name = "U235"
material_composition_vector = [
  {{ material_name = "U238", material_fraction = 0.25 }},
  {{ material_name = "U235", material_fraction = 0.75 }},
]
order = 4
"""
    entry = template.format(x=x, y=y, z=z, width=width, depth=depth, height=height)
    return entry


def control_rod(x_value, y_value, z_value):
    template = """
[[cylinders]] # Control rod
center = {{ x = {x}, y = {y}, z = {z} }}
direction = {{ x = 0.0, y = 0.0, z = 1.0 }}
length = 4.0
radius = 0.02
material_name = "B10"
material_composition_vector = [
  {{ material_name = "B10", material_fraction = 1.0 }},
]
order = 5
"""
    entry = template.format(x=x_value, y=y_value, z=z_value)
    return entry


def control_plate(x, y, z, width, depth, height):
    template = """
[[cuboids]] # Control rod
center = {{ x = {x}, y = {y}, z = {z} }}
width = {width}
depth = {depth}
height = {height}
material_name = "B10"
material_composition_vector = [
  {{ material_name = "B10", material_fraction = 1.0 }},
]
order = 5
"""
    entry = template.format(x=x, y=y, z=z, width=width, depth=depth, height=height)
    return entry
