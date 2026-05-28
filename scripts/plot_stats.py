#!/usr/bin/env python3
"""Plot per-frame stats CSV produced by tools/host_sim

CSV columns: frame_idx,input_rms_db,output_rms_db,noise_rms,snr_db
"""
import sys
import matplotlib.pyplot as plt
import csv

if len(sys.argv) < 2:
    print("Usage: plot_stats.py stats.csv")
    sys.exit(1)

path = sys.argv[1]
frame = []
input_db = []
output_db = []
noise = []
snr = []

with open(path, newline='') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        frame.append(int(row['frame_idx']))
        input_db.append(float(row['input_rms_db']))
        output_db.append(float(row['output_rms_db']))
        noise.append(float(row['noise_rms']))
        snr.append(float(row['snr_db']))

plt.figure(figsize=(10,6))
plt.subplot(2,1,1)
plt.plot(frame, input_db, label='Input RMS dB')
plt.plot(frame, output_db, label='Output RMS dB')
plt.legend()
plt.grid(True)

plt.subplot(2,1,2)
plt.plot(frame, snr, label='SNR dB', color='tab:orange')
plt.legend()
plt.grid(True)

plt.xlabel('Frame Index')
plt.tight_layout()
plt.show()
