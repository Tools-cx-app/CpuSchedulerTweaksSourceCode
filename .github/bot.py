import os
import re
import asyncio
from telegram import Bot, InputMediaDocument
from telegram.constants import ParseMode

# 从环境变量获取配置
BOT_TOKEN = os.getenv('BOT_TOKEN')
CHAT_ID = os.getenv('CHAT_ID')
GITHUB_REPOSITORY = os.getenv('GITHUB_REPOSITORY')
GITHUB_RUN_ID = os.getenv('GITHUB_RUN_ID')
VERSION = os.getenv('VERSION')
MESSAGE_THREAD_ID = os.getenv('THREAD_ID')
COMMIT_MESSAGE = os.getenv('COMMIT_MESSAGE')
COMMIT_AUTHOR_NAME = os.getenv('COMMIT_AUTHOR_NAME')

def escape_markdownv2(text: str) -> str:
    """转义 Telegram MarkdownV2 特殊字符"""
    escape_chars = r'_*[]()~`>#+-=|{}.!'
    return re.sub(f'([{re.escape(escape_chars)}])', r'\\1', text).strip()

async def send_telegram_message():

    # 构建消息内容
    message_text = (
        f"#[ci_{GITHUB_RUN_ID}](https://github.com/{GITHUB_REPOSITORY}/actions/runs/{GITHUB_RUN_ID})\n"
        f"```\n"
        f"{COMMIT_MESSAGE}\n"
        f"```\n"
        f"by `{COMMIT_AUTHOR_NAME}`"
    )

    # 转义特殊字符
    escaped_text = escape_markdownv2(message_text)

    # 初始化 Telegram bot
    bot = Bot(token=BOT_TOKEN)
    
    # 发送带附件的消息
    with open('CpuSchedulerTweaks.zip', 'rb') as file:
        await bot.send_document(
            chat_id=CHAT_ID,
            message_thread_id=MESSAGE_THREAD_ID,
            document=file,
            filename='CpuSchedulerTweaks.zip',
            caption=escaped_text,
            parse_mode=ParseMode.MARKDOWN_V2
        )

if __name__ == "__main__":
    asyncio.run(send_telegram_message())