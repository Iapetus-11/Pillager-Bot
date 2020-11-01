from discord.ext import commands
import classyjson as cj
import discord

bot = commands.AutoShardedBot(
    command_prefix='$',
    case_insensitive=True,
    intents=iscord.Intents.all(),
    help_command=None
)

with open('config.json', 'r') as conf:
    bot.conf = cj.load(conf)

bot.cog_list = [
    'cogs.core.events',
    'cogs.cmds.owner'
]

for cog in bot.cog_list:
    bot.load_extension(cog)

bot.run(bot.conf.discord_token)
