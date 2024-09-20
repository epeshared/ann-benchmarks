#!/bin/bash
set -e

if [ "$(cat /sys/kernel/mm/transparent_hugepage/enabled | grep -o '\[always\]')" != "[always]" ]; then \
        echo "Transparent Huge Pages is not set to 'always'. Exiting..."; \
        exit 1; \
fi

# python3 install.py --algorithm faiss_amx
# python3 install.py --algorithm faiss

# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,128-135" --mem-limit 16 --dataset glove-100-angular   --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "8-15,136-143" --mem-limit 16 --dataset glove-100-angular  --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "16-23,144-151" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "24-31,152-159" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "32-39,160-167" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "40-47,168-175" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "48-55,176-183" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "56-63,184-191" --mem-limit 16 --dataset glove-100-angular --force

# python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,128-135" --mem-limit 16 --dataset glove-100-angular   --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "8-15,136-143" --mem-limit 16 --dataset glove-100-angular  --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "16-23,144-151" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "24-31,152-159" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "32-39,160-167" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "40-47,168-175" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "48-55,176-183" --mem-limit 16 --dataset glove-100-angular --force
# python3 run.py --algorithm faiss-indexflatip --cpu-range "56-63,184-191" --mem-limit 16 --dataset glove-100-angular --force

# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,128-135" --mem-limit 16 --dataset glove-100-angular   --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "8-15,136-143" --mem-limit 16 --dataset glove-100-angular  --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "16-23,144-151" --mem-limit 16 --dataset glove-100-angular --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "24-31,152-159" --mem-limit 16 --dataset glove-100-angular --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "32-39,160-167" --mem-limit 16 --dataset glove-100-angular --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "40-47,168-175" --mem-limit 16 --dataset glove-100-angular --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "48-55,176-183" --mem-limit 16 --dataset glove-100-angular --force --batch &
# python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "56-63,184-191" --mem-limit 16 --dataset glove-100-angular --force --batch &

# python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,128-135" --mem-limit 16 --dataset glove-100-angular   --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "8-15,136-143" --mem-limit 16 --dataset glove-100-angular  --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "16-23,144-151" --mem-limit 16 --dataset glove-100-angular --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "24-31,152-159" --mem-limit 16 --dataset glove-100-angular --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "32-39,160-167" --mem-limit 16 --dataset glove-100-angular --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "40-47,168-175" --mem-limit 16 --dataset glove-100-angular --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "48-55,176-183" --mem-limit 16 --dataset glove-100-angular --force --batch
# python3 run.py --algorithm faiss-indexflatip --cpu-range "56-63,184-191" --mem-limit 16 --dataset glove-100-angular --force --batch

# rm -rf emon.dat
# rm -rf emon_data/*
# mkdir -p emon_data

# emon -collect-edp > emon.dat &
numactl -m "0,1,2" python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-42,43-84,85-127,256-298,299-340,341-383" --dataset glove-100-angular   --force --batch
# emon -stop
# emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
# mv summary.xlsx emon_data/emr_indexflatip-amx_sum_glove-100-angular.xlsx
# rm -rf emon.dat
# rm -rf *.csv

# emon -collect-edp > emon.dat &
numactl -m "0,1,2" python3 run.py --algorithm faiss-indexflatip --cpu-range "0-42,43-84,85-127,256-298,299-340,341-383" --dataset glove-100-angular   --force --batch
# emon -stop
# emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
# mv summary.xlsx emon_data/emr_indexflatip_sum_glove-100-angular.xlsx
# rm -rf emon.dat
# rm -rf *.csv

# emon -collect-edp > emon.dat &
numactl -m "0,1,2" python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-42,43-84,85-127,256-298,299-340,341-383"  --dataset nytimes-256-angular    --force --batch
# emon -stop
# emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
# mv summary.xlsx emon_data/emr_indexflatip-amx_sum_nytimes-256-angular.xlsx
# rm -rf emon.dat
# rm -rf *.csv

# emon -collect-edp > emon.dat &
numactl -m "0,1,2" python3 run.py --algorithm faiss-indexflatip --cpu-range "0-42,43-84,85-127,256-298,299-340,341-383"  --dataset nytimes-256-angular   --force --batch
# emon -stop
# emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
# mv summary.xlsx emon_data/emr_indexflatip_sum_nytimes-256-angular.xlsx
# rm -rf emon.dat
# rm -rf *.csv

# python data_export.py --out res.csv

rm -rf website/*
mkdir -p website
python create_website.py --outputdir website