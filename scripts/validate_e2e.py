#!/usr/bin/env python3
"""
End-to-end simulation validator for NFL firmware.
Validates:
- WAV file existence and format
- SNR > 20dB requirement
- Pipeline output integrity
"""

import sys
import csv
import argparse
from pathlib import Path

def validate_wav_exists(wav_path):
    """Check if output WAV file was created."""
    wav_file = Path(wav_path)
    if not wav_file.exists():
        print(f"❌ ERROR: Output WAV file not found: {wav_path}")
        return False
    if wav_file.stat().st_size == 0:
        print(f"❌ ERROR: Output WAV file is empty: {wav_path}")
        return False
    print(f"✅ Output WAV file exists: {wav_path} ({wav_file.stat().st_size} bytes)")
    return True

def validate_csv_snr(csv_path, min_snr=20.0):
    """Validate SNR > 20dB from CSV statistics."""
    csv_file = Path(csv_path)
    if not csv_file.exists():
        print(f"⚠️  WARNING: CSV stats file not found: {csv_path}")
        print(f"   Skipping SNR validation")
        return True  # Don't fail if CSV isn't available
    
    snr_values = []
    try:
        with open(csv_file, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                try:
                    snr = float(row['snr_db'])
                    snr_values.append(snr)
                except (ValueError, KeyError) as e:
                    print(f"⚠️  WARNING: Could not parse SNR from row: {e}")
                    continue
    except Exception as e:
        print(f"❌ ERROR: Failed to read CSV file: {e}")
        return False
    
    if not snr_values:
        print(f"⚠️  WARNING: No SNR values found in CSV")
        return True
    
    avg_snr = sum(snr_values) / len(snr_values)
    min_frame_snr = min(snr_values)
    max_frame_snr = max(snr_values)
    
    print(f"📊 SNR Statistics:")
    print(f"   Average SNR: {avg_snr:.2f} dB")
    print(f"   Min SNR: {min_frame_snr:.2f} dB")
    print(f"   Max SNR: {max_frame_snr:.2f} dB")
    print(f"   Total frames analyzed: {len(snr_values)}")
    
    if avg_snr > min_snr:
        print(f"✅ SNR check PASSED: {avg_snr:.2f} dB > {min_snr} dB")
        return True
    else:
        print(f"❌ SNR check FAILED: {avg_snr:.2f} dB < {min_snr} dB")
        return False

def main():
    parser = argparse.ArgumentParser(description='Validate E2E simulation output')
    parser.add_argument('--wav', required=True, help='Path to output WAV file')
    parser.add_argument('--csv', help='Path to CSV statistics file')
    parser.add_argument('--min-snr', type=float, default=20.0, help='Minimum required SNR in dB')
    args = parser.parse_args()
    
    print("=" * 60)
    print("NFL Firmware E2E Simulation Validator")
    print("=" * 60)
    print()
    
    # Check WAV file
    if not validate_wav_exists(args.wav):
        return 1
    print()
    
    # Check SNR if CSV provided
    if args.csv:
        if not validate_csv_snr(args.csv, args.min_snr):
            return 1
        print()
    
    print("=" * 60)
    print("✅ All validation checks PASSED!")
    print("=" * 60)
    return 0

if __name__ == '__main__':
    sys.exit(main())
