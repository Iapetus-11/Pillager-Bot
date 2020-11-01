from discord.ext import commands
import traceback
import discord
import random


class Events(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

        self.conf = bot.conf

    @commands.Cog.listener()
    async def on_ready(self):
        print('\n CONNECTED \n')

    @commands.Cog.listener()
    async def on_member_join(self, member):
        g_conf = self.conf.welcoming.guilds.get(str(member.guild.id))

        if g_conf is not None:
            if g_conf.welcome:
                await self.bot.get_channel(g_conf.channel).send(random.choice(self.conf.welcoming.welcomes).format(member.mention))

            if g_conf.role_id:
                await member.add_roles(member.guild.get_role(g_conf.role_id))

    @commands.Cog.listener()
    async def on_member_remove(self, member):
        g_conf = self.conf.welcoming.guilds.get(str(member.guild.id))

        if g_conf is not None:
            if g_conf.farewell:
                await self.bot.get_channel(g_conf.channel).send(f'Goodbye, {member.mention} ({member})')

    @commands.Cog.listener()
    async def on_command_error(self, ctx, e):
        for e_type in (commands.CommandNotFound, commands.NotOwner, discord.errors.Forbidden,):
            if isinstance(e, e_type) or isinstance(e.__dict__.get('original'), e_type):
                return

        if isinstance(e, commands.CommandOnCooldown):
            seconds = round(e.retry_after, 2)

            if seconds <= .05:
                await ctx.reinvoke()
                return

            hours = int(seconds / 3600)
            minutes = int(seconds / 60) % 60
            seconds -= round((hours * 60 * 60) + (minutes * 60), 2)

            time = ''
            if hours > 0: time += f'{hours} hours, '
            if minutes > 0: time += f'{minutes} minutes, '
            time += f'{round(seconds, 2)} seconds'

            await ctx.send(f'Be patient and wait {time} stupidhead')
        elif isinstance(e, commands.NoPrivateMessage):
            await ctx.send('Can\'t use that command here fuckwad')
        elif isinstance(e, commands.MissingPermissions):
            await ctx.send('I dare you to again motherfucker *stares motherfuckerly*')
        elif isinstance(e, commands.BotMissingPermissions):
            await ctx.send('Imagine not giving me admin perms degenerate poopy head')
        elif isinstance(e, commands.MaxConcurrencyReached):
            await ctx.send('Try again in a bit retard')
        elif isinstance(e, commands.MissingRequiredArgument):
            await ctx.send('FUCKING USE THE COMMAND RIGHT DEGENERATE SCUM')
        elif isinstance(e, commands.BadArgument) or isinstance(e, commands.errors.ExpectedClosingQuoteError) or isinstance(e, commands.errors.BadUnionArgument):
            await ctx.send('YOU FUCKING TYPED SOMETHING WRONG USE THE COMMAND RIGHT DEGENERATE')
        else:
            traceback_text = ''.join(traceback.format_exception(type(e), e, e.__traceback__, 4))
            final = f'{ctx.author}: {ctx.message.content}\n\n{traceback_text}'.replace('``', '\`\`\`')
            await ctx.send(f'```{final[:2000 - 6]}```')

def setup(bot):
    bot.add_cog(Events(bot))
