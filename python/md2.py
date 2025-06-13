import sys, re;
msg = sys.argv[1]

# 转换 #ci_xxx 为 Markdown 链接
# msg = re.sub(r"#ci_(\w+)", r"[#ci_\1](https://your-ci-link/ci_\1)", msg)

# 特殊处理代码块
parts = re.split(r"(```[^`]*```)", msg, flags=re.DOTALL)
special_chars = "_*[]()~`>#+-=|{}.!"

for i in range(len(parts)):
    # 只处理非代码块部分
    if not parts[i].startswith("```"):
        # 转义所有特殊字符
        parts[i] = re.sub(f"([{re.escape(special_chars)}])", r"\\\1", parts[i])
        
        # 额外处理 # 字符（确保所有 # 都被转义）
        parts[i] = re.sub(r"(?<!\\)#", r"\\#", parts[i])

# 重新组合并处理换行
result = "".join(parts).replace("\n", "\\n")
print(result)