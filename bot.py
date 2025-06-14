import os
import re
import asyncio
from telegram import Bot
from telegram.constants import ParseMode
def escape_markdown_v2(text):
    """正确转义 MarkdownV2 特殊字符"""
    escape_chars = r'_*[]()~`>#+-=|{}.!'
    return re.sub(f'([{re.escape(escape_chars)}])', r'\\\1', text)

async def send_document_async():
    """异步发送文档"""
    # 从环境变量获取配置
    bot_token = os.environ['BOT_TOKEN']
    chat_id = os.environ['CHAT_ID']
    commit_message = os.environ['COMMIT_MESSAGE']
    file_path = 'CpuSchedulerTweaks.zip'  # 要上传的文件名

    # 转义消息内容
    escaped_message = escape_markdown_v2(commit_message)
    
    # 初始化 Telegram Bot
    bot = Bot(token=bot_token)
    
    # 发送文档
    with open(file_path, 'rb') as f:
        await bot.send_document(
            chat_id=chat_id,
            document=f,
            filename=os.path.basename(file_path),
            caption=escaped_message,
            parse_mode=ParseMode.MARKDOWN,
            message_thread_id=5  # 话题ID
        )

def main():
    """主函数运行异步任务"""
    asyncio.run(send_document_async())

if __name__ == '__main__':
    main()