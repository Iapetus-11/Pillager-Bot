from discord.ext import commands
import classyjson as cj
import discord

intents = discord.Intents.default()
intents.guilds = True
intents.members = True
intents.bans = False
intents.emojis = True
intents.integrations = False
intents.webhooks = False
intents.invites = False
intents.voice_states = False
intents.presences = True
intents.messages = True
# intents.guild_messages = True
# intents.dm_messages = True
intents.reactions = True
# intents.guild_reactions = True
# intents.dm_reactions = True
intents.typing = False
# intents.guild_typing = False
# intents.dm_typing = False

bot = commands.AutoShardedBot(
    command_prefix='$',
    case_insensitive=True,
    intents=intents,
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
