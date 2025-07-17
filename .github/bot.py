from telethon import TelegramClient
import asyncio
from telethon.sessions import StringSession
import os

API_ID = 611335
API_HASH = "d524b414d21f4d37f08684c1df41ac9c"

BOT_TOKEN = os.environ.get("BOT_TOKEN")
CHAT_ID = int(os.environ.get("CHAT_ID"))
MESSAGE_THREAD_ID = 5
RUN_ID = os.environ.get("RUN_ID")
RUN_URL = os.environ.get("RUN_URL")
COMMIT_MESSAGE = os.environ.get("COMMIT_MESSAGE")
BRANCH = os.environ.get("BRANCH")
HASH = os.environ.get("HASH")
BOT_CI_SESSION = os.environ.get("BOT_CI_SESSION")
MSG_TEMPLATE = """
#ci_{run_id}
branch: {branch}
hash: {hash}
```
{commit_message}
```
[Workflow run]({run_url})
""".strip()


def get_caption():
    msg = MSG_TEMPLATE.format(
        run_id=RUN_ID,
        branch=BRANCH,
        hash=HASH,
        commit_message=COMMIT_MESSAGE,
        run_url=RUN_URL,
    )
    if len(msg) > 1024:
        return COMMIT_URL
    return msg

async def send_telegram_message():
    async with TelegramClient(StringSession(BOT_CI_SESSION), api_id=API_ID, api_hash=API_HASH) as client:
        await client.start(bot_token=BOT_TOKEN)
        print("[+] Caption: ")
        print("---")
        print("---")
        print("[+] Sending")
        await client.send_file(
            entity=CHAT_ID,
            reply_to=MESSAGE_THREAD_ID,
            file='./CpuSchedulerTweaks.zip',
            caption=get_caption(),
            parse_mode="markdown"
        )

if __name__ == '__main__':
    try:
        asyncio.run(send_telegram_message())
    except Exception as e:
        print(f"[-] An error occurred: {e}")
