import subprocess
import json
from statistics import mean

exp_path = "../target/release/lane_merging_experiment"

lambdas = [0.1, 0.2, 0.3, 0.4, 0.5]
for i in lambdas:
    result = subprocess.run([exp_path, "-n", "100", "-l", str(i), "-t", "100"], capture_output=True).stdout.decode().strip()
    with open(f'log_{i}.json', 'w') as f:
        f.write(result)