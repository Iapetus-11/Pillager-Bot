from discord.ext import commands
import discord
import os


class Owner(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.command(name='load')
    @commands.is_owner()
    async def load_cog(self, ctx, cog):
        self.bot.load_extension(f'cogs.{cog}')
        await ctx.send('Done.')

    @commands.command(name='unload')
    @commands.is_owner()
    async def unload_cog(self, ctx, cog):
        self.bot.unload_extension(f'cogs.{cog}')
        await ctx.send('Done.')

    @commands.command(name='reload')
    @commands.is_owner()
    async def reload_cog(self, ctx, cog):
        if cog == 'all':
            await self.reload_all_cogs(ctx)
        else:
            self.bot.reload_extension(f'cogs.{cog}')
            await ctx.send('Done.')

    @commands.command(name='reloadall')
    @commands.is_owner()
    async def reload_all_cogs(self, ctx):
        for cog in self.bot.cog_list:
            self.bot.reload_extension(cog)

        await ctx.send('Done.')

    @commands.command(name='eval')
    @commands.is_owner()
    async def eval_stuff(self, ctx, *, stuff):
        await ctx.send(f'```{eval(stuff)}```')

    @commands.command(name='exec')
    @commands.is_owner()
    async def exec_stuff(self, ctx, *, stuff):
        await ctx.send(f'```{exec(stuff)}```')

    @commands.command(name='awaiteval')
    @commands.is_owner()
    async def await_eval_stuff(self, ctx, *, stuff):
        await ctx.send(f'```{await eval(stuff)}```')

    @commands.command(name='gitpull')
    @commands.max_concurrency(1, per=commands.BucketType.default, wait=True)
    @commands.is_owner()
    async def gitpull(self, ctx):
        with ctx.typing():
            os.system('sudo git pull > git_pull_log 2>&1')

        with open('git_pull_log', 'r') as f:
            await ctx.send(f'```diff\n{f.read()}\n```')

        os.remove('git_pull_log')

def setup(bot):
    bot.add_cog(Owner(bot))
