import os
import plotly.express as px
import plotly.graph_objects as go
import numpy as np
import heatmap
from tqdm import tqdm


def component_size_amounts(subfolder: str):
    print(f"[{subfolder}] start component-size-amounts")
    pairs = []
    with open(f"data/{subfolder}/component-size-amounts.data") as f:
        for line in f:
            line = line.rstrip("\n").split(";")
            size = int(line[0])
            amount = int(line[1])
            pairs.append((size, amount))
    pairs.sort()

    total = sum([amount for (_, amount) in pairs])

    index = 0
    first_few = []
    middle = []
    last_few = []
    for p in pairs:
        index += 1
        if index < 6:
            first_few.append(p)
        elif index < len(pairs) - 3:
            middle.append(p)
        else:
            last_few.append(p)

    print(subfolder, f"total: {total}")
    for f in first_few:
        print(f)

    sums = sum([size * amount for (size, amount) in middle])
    number_of_comps = sum([amount for (_, amount) in middle])
    average_size = sums / number_of_comps
    print(
        f"{middle[0]}--{middle[-1]}, num={number_of_comps}, avg-cmp-size={average_size:.2f}"
    )

    for l in last_few:
        print(l)
    exit(0)


def word_length_neighbors(subfolder: str):
    print(f"[{subfolder}] start word-length-neighbors")
    with open(f"data/{subfolder}/word-length-neighbors.data") as f:
        data = []
        for line in f:
            point = line.rstrip("\n").split(";")
            data.append((min(int(point[1]), 10), min(32, int(point[0]))))
    arr = np.array(data)
    map = px.density_heatmap(
        arr,
        x=0,
        y=1,
        # nbinsx=200,
        # nbinsy=40,
        labels={"0": "saturated number of neighbors", "1": "word length"},
    )
    map.update_layout(
        title={
            "text": f"Word lengths and number of neighbors ({subfolder})",
            "x": 0.5,
            "xanchor": "center",
            "yanchor": "top",
        },
    )

    map.update_xaxes(
        # range=[0, 20],
        # title_text="",
        # showline=True,
        # linewidth=5,
        # linecolor="white",
        # tickmode="linear",
        # tick0=0,
        dtick=1,
        tickmode="array",
        tickvals=np.array(range(0, 11)),
        ticktext=list([str(i) for i in range(0, 10)] + ["10+"]),
    )
    map.update_yaxes(dtick=2)

    map.write_image(f"data/{subfolder}/word-length-neighbors.png", scale=3.0)
    print(f"[{subfolder}] stop word-length-neighbors")


def word_length_comps(subfolder: str):
    print(f"[{subfolder}] start word-lengths-comp")
    with open(f"data/{subfolder}/word-lengths.data") as f:
        data = []
        for line in f:
            point = line.rstrip("\n").split(";")
            data.append((min(int(point[1]), 20), min(35, int(point[0]))))
    arr = np.array(data)
    map = px.density_heatmap(
        arr,
        x=0,
        y=1,
        # nbinsx=200,
        # nbinsy=40,
        labels={"0": "saturated component size", "1": "word length"},
    )
    map.update_layout(
        title={
            "text": f"Distribution of word lengths across component sizes ({subfolder})",
            "x": 0.5,
            "xanchor": "center",
            "yanchor": "top",
        },
    )

    map.update_xaxes(
        # range=[0, 20],
        # title_text="",
        # showline=True,
        # linewidth=5,
        # linecolor="white",
        # tickmode="linear",
        # tick0=0,
        dtick=1,
        tickmode="array",
        tickvals=np.array(range(0, 21)),
        ticktext=list([str(i) for i in range(0, 20)] + ["20+"]),
    )
    map.update_yaxes(dtick=1)

    map.write_image(f"data/{subfolder}/word-lengths-comp.png", scale=3.0)
    print(f"[{subfolder}] stop word-lengths-comp")


if __name__ == "__main__":
    os.chdir("..")
    # heatmap.create_heatmaps("data/word-cat-dist/heatmap.data", "data/word-cat-dist")
    for folder in tqdm(
        ["ird", "ir_", "i_d", "i__", "_rd", "_r_", "__d"], desc="word-length-components"
    ):
        component_size_amounts(folder)
        word_length_neighbors(folder)
        word_length_comps(folder)
