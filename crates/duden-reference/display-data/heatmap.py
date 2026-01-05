import os
import plotly.express as px
from plotly.subplots import make_subplots
import math
import numpy as np
from tqdm import tqdm


def square_form(l: list[int]) -> list[list[int]]:
    length = len(l)
    border = math.ceil(math.sqrt(length))
    grid = [[]]
    for x in l:
        grid[-1].append(x)

        if len(grid[-1]) > border:
            grid.append([])
    if len(grid[-1]) == 0:
        grid.pop()
    while len(grid[-1]) <= border:
        grid[-1].append(0)
    return grid


def create_heatmaps(data_path: str, image_folder: str):
    os.makedirs(image_folder, exist_ok=True)
    overall_category_sizes = []
    with open(data_path, "r") as file:
        for word_length, line in tqdm(
            list(enumerate(file)), desc="category-distribution-heatmaps"
        ):
            line = line.rstrip("\n")
            line = [int(i) for i in line.split(",") if i != ""]
            overall_category_sizes.extend(line)
            if len(line) > 0:
                grid = np.array(square_form(line))
                cat_count = len(line)
                words = sum(line)
                fig = px.imshow(
                    grid,
                )
                fig.update_layout(
                    title={
                        "text": str(
                            f"{words} words in {cat_count} categories (length {word_length})"
                        ),
                        "x": 0.5,
                        "xanchor": "center",
                        "yanchor": "top",
                    },
                )
                fig.write_image(
                    f"{image_folder}/cat-heatmap-{word_length:02}.png", scale=4
                )
    grid = np.array(square_form(overall_category_sizes))
    fig = px.imshow(
        grid,
    )
    fig.update_layout(
        title={
            "text": str(
                f"{sum(overall_category_sizes)} words in {len(overall_category_sizes)} categories (all lengths)"
            ),
            "x": 0.5,
            "xanchor": "center",
            "yanchor": "top",
        },
    )
    fig.write_image(f"{image_folder}/cat-heatmap-total.png", scale=5)
