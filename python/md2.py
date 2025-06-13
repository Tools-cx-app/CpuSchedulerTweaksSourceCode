import sys, re

msg = sys.argv[1]
parts = re.split(r"(```.*?```)", msg, flags=re.DOTALL)
special_chars = r"_*[]()~`>#+-=|{}.!"
for i in range(len(parts)):
    if not parts[i].startswith("```"):
        parts[i] = re.sub(f"([{re.escape(special_chars)}])", r"\\\1", parts[i])
result = "".join(parts).replace("\n", "\\n")
print(result)
