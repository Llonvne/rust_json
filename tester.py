import csv
import subprocess
import time

# 测试次数列表
test_cases = [1, 10, 100, 1000, 10000]

# CSV 文件名称
csv_file = 'rust_run_times.csv'

# 可执行文件的路径（请根据实际情况修改）
executable_path = './target/release/rust_json'

# 生成 CSV 头部
def generate_csv_header(num_runs):
    header = ['Test Cases']
    for i in range(1, num_runs + 1):
        header.append(f'Run {i}')
    return header

# 打开 CSV 文件并写入
with open(csv_file, mode='w', newline='') as file:
    writer = csv.writer(file)

    # 写入动态生成的 CSV 文件头
    writer.writerow(generate_csv_header(100))

    for test in test_cases:
        times = []
        for _ in range(100):  # 每个测试运行 100 次
            start_time = time.time()
            subprocess.run([executable_path, str(test)])  # 运行可执行文件
            end_time = time.time()
            elapsed_time = end_time - start_time
            times.append(elapsed_time)

        # 写入每次运行的时间
        writer.writerow([test] + times)

print(f"Run times have been saved to {csv_file}")
