import asyncio
import os
import sys
from telethon import TelegramClient

API_ID = 11
API_HASH = "4fc82b26aecb47d2868c4efbe3581732a3e7cbcc6c2efb32062c08170a05eeb8"

BOT_TOKEN = os.getenv('BOT_TOKEN')
CHAT_ID = os.getenv('CHAT_ID')
GITHUB_REPOSITORY = os.getenv('GITHUB_REPOSITORY')
GITHUB_RUN_ID = os.getenv('GITHUB_RUN_ID')
VERSION = os.getenv('VERSION')
MESSAGE_THREAD_ID = os.getenv('THREAD_ID')
COMMIT_MESSAGE = os.getenv('COMMIT_MESSAGE')
COMMIT_AUTHOR_NAME = os.getenv('COMMIT_AUTHOR_NAME')
MSG_TEMPLATE = """
#[ci_{run_id}](https://github.com/{repo}/actions/runs/{run_id})
```
{msg}
```
by `{anthor_name}`
""".strip()

def get_caption():
    msg = MSG_TEMPLATE.format(
        run_id=GITHUB_RUN_ID,
        repo=GITHUB_REPOSITORY,
        msg=COMMIT_MESSAGE,
        anthor_name=COMMIT_AUTHOR_NAME,
    )
    if len(msg) > 1024:
        return COMMIT_URL
    return msg

async def main():
    print("[+] Uploading to telegram")
    print("[+] Logging in Telegram with bot")
    script_dir = os.path.dirname(os.path.abspath("CpuSchedulerTweaks.zip"))
    session_dir = os.path.join(script_dir, "ci")
    async with await TelegramClient(session=session_dir, api_id=API_ID, api_hash=API_HASH).start(bot_token=BOT_TOKEN) as bot:
        caption = [""] * len(files)
        caption[-1] = get_caption()
        print("[+] Caption: ")
        print("---")
        print(caption)
        print("---")
        print("[+] Sending")
        await bot.send_file(entity=CHAT_ID, file=files, caption=caption, reply_to=MESSAGE_THREAD_ID, parse_mode="markdown")
        print("[+] Done!")

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except Exception as e:
        print(f"[-] An error occurred: {e}")