#!/bin/bash
set -euo pipefail  # 启用严格模式：出错退出、未定义变量报错、管道错误检测

# 原始版本号
VERSION="0.1.1"

# 使用awk分割和处理版本号
NEW_VERSION=$(echo "$VERSION" | awk -F. '{
    # 将最后一个字段加1
    $NF = $NF + 1
    # 重新组合为点分隔的字符串
    for (i=1; i<=NF; i++) {
        printf "%s", $i
        if (i < NF) printf "."
    }
}')

echo "原始版本: $VERSION"
echo "新版本:   $NEW_VERSION"
echo "该文件未修改, 仅供参考"