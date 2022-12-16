import matplotlib.pyplot as plt
import argparse
import json
import pandas as pd
import numpy as np

"""
parser = argparse.ArgumentParser()

parser.add_argument('--file', type=str, required=True)

args = parser.parse_args()
"""


logs = {}
for i in ["0.1", "0.2", "0.3", "0.4", "0.5"]:
    with open(f'log_{i}.json') as f:
        logs[i] = json.load(f)

features = ['mean_delay', 'max_delay', 't_last']
title = {
    'mean_delay': 'mean delay',
    'max_delay': 'max delay',
    't_last': 'T last'
}
for feature in features:
    print(feature)
    methods = {'FCFS':[], 'DP':[], 'Simulation Annealing':[]}
    lams = []
    for lam, log in logs.items():
        dfs = []
        for l in log:
            traffic = l["traffic"]
            results = l["methods"]
            dic = {k: {feature: l[f'{feature}'][k]} for k in results.keys()}
            df = pd.DataFrame.from_dict(dic)
            dfs.append(df)
        df = pd.concat(dfs)
        meaned = df.mean()
        dic = meaned.to_dict()
        print(f'reduce: {(meaned["DP"] - meaned["Simulation Annealing"]) / meaned["DP"]}')
        for k in methods.keys():
            methods[k].append(dic[k])
        lams.append(lam)
    
    x_axis = np.arange(len(lams))

    length = 0.2 * len(methods)
    for i, k in enumerate(methods.keys()):
        plt.bar(x_axis - length / 2 + length / len(methods) * i, methods[k], 0.2, label = k)

    plt.xticks(x_axis, lams)
    plt.xlabel('lambda')
    plt.ylabel('second')
    plt.legend()
    plt.savefig(f'{feature}.eps', format='eps')
    plt.cla()
    plt.clf()