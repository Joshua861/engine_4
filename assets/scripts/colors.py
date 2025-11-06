import json
from coloraide import Color

color_names = []

with open('colors.json', 'r') as f:
    data = json.load(f)
    for color_name in data.keys():
        for brightness in data[color_name]:
            name = f"{color_name}_{brightness}".upper()
            color_names.append(f"Self::{name}");
            oklch = data[color_name][brightness]
            oklch_normalized = [oklch[0] / 100, oklch[1], oklch[2]]
            color = Color("oklch", oklch_normalized)

            rgb = color.convert("srgb-linear").fit("srgb-linear")

            r, g, b = rgb[0], rgb[1], rgb[2]

            print(f"pub const {name}: Self = Self::new({r}, {g}, {b});")

print(f"pub const ALL: [Self; {len(color_names)}] = {color_names};")
