import sys, re

msg = sys.argv[1]

# 不再将 #ci_xxx 转换为链接，而是直接保留

# 特殊处理代码块
parts = re.split(r"(```[^`]*```)", msg, flags=re.DOTALL)
special_chars = "_*[]()~`>#+-=|{}.!"

for i in range(len(parts)):
    # 只处理非代码块部分
    if not parts[i].startswith("```"):
        # 转义所有特殊字符
        parts[i] = re.sub(f"([{re.escape(special_chars)}])", r"\\\\1", parts[i])

# 重新组合并处理换行
result = "".join(parts).replace("\n", "\\n")
print(result)