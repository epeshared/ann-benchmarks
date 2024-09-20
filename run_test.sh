#!/bin/bash
set -e

python3 install.py --algorithm faiss_amx
python3 install.py --algorithm faiss

emon_enable=0  # 设置为1时启用emon相关代码，设置为0时禁用

if [ "$(cat /sys/kernel/mm/transparent_hugepage/enabled | grep -o '\[always\]')" != "[always]" ]; then \
        echo "Transparent Huge Pages is not set to 'always'. Exiting..."; \
        exit 1; \
fi

# 检查emon_enable是否为1
if [ "$emon_enable" -eq 1 ]; then
  rm -rf emon.dat
  rm -rf emon_data/*
  rm -rf *.csv
  mkdir -p emon_data
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,256-263" --mem-limit 16 --dataset glove-100-angular --force --runs 1
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip-amx_sum_glove-100-angular_single_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,256-263" --mem-limit 16 --dataset glove-100-angular --force --runs 1
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip_sum_glove-100-angular_single_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,256-263" --mem-limit 16 --dataset glove-100-angular --force --batch
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip-amx_sum_glove-100-angular_batch_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,256-263" --mem-limit 16 --dataset glove-100-angular --force --batch
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip_sum_glove-100-angular_batch_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,256-263" --mem-limit 16 --dataset nytimes-256-angular --force --runs 1
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip-amx_sum_nytimes-256-angular_single_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,256-263" --mem-limit 16 --dataset nytimes-256-angular --force --runs 1
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip_sum_nytimes-256-angular_single_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip-amx --cpu-range "0-7,256-263" --mem-limit 16 --dataset nytimes-256-angular --force --batch
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip-amx_sum_nytimes-256-angular_batch_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi


if [ "$emon_enable" -eq 1 ]; then
  emon -collect-edp > emon.dat &
fi
python3 run.py --algorithm faiss-indexflatip --cpu-range "0-7,256-263" --mem-limit 16 --dataset nytimes-256-angular --force --batch
if [ "$emon_enable" -eq 1 ]; then
  emon -stop
  emon -process-pyedp /opt/intel/sep/config/edp/pyedp_config.txt
  mv summary.xlsx emon_data/gnr_indexflatip_sum_nytimes-256-angular_batch_8c16t.xlsx
  rm -rf emon.dat
  rm -rf *.csv
fi

rm -rf res.csv
python data_export.py --out res.csv

rm -rf website/*
mkdir -p website
python create_website.py --outputdir website