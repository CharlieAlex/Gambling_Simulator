import gambling_simulator
import polars as pl
import plotly.express as px
import plotly.graph_objs as go

def play_martingale_game(params:dict[str, float])->tuple[pl.DataFrame, pl.DataFrame]:

    output, last_process = gambling_simulator.play_martingale(
        params['n_simulations'],
        params['max_games'],
        params['init_wealth'],
        params['win_rate'],
        params['odds'],
        params['stake'],
    )
    return pl.DataFrame(output), pl.DataFrame(last_process)

def compute_stats(df:pl.DataFrame)->pl.DataFrame:
    return (df
        .select(pl.col('final_wealth'))
        .describe()
        .with_columns(pl.col('final_wealth').cast(pl.Int64))
    )

def compute_earnrate(df:pl.DataFrame, init_wealth:int)->str:
    earn_rate = (
        df.filter(pl.col('final_wealth') > init_wealth).count()
        /
        df.select(pl.len())
    )
    earn_rate = round(earn_rate.to_numpy()[0][0]*100, 2)
    return str(earn_rate) + '%'

def plot_wealth_dist(df:pl.DataFrame, init_wealth:int)->go.Figure:
    fig = px.histogram(df, x='final_wealth', title='Final Wealth Distribution')

    fig.update_layout(
        xaxis_title="Final Wealth",
        yaxis_title="Frequency",
        bargap=0.01,  # gap between bars of adjacent location coordinates
    )

    fig.add_vline(x=init_wealth, line_dash="solid", line_color="red")

    return fig

def plot_process_lineplot(df:pl.DataFrame)->go.Figure:
    df = df.with_columns(pl.Series('round', range(0, df.height)))

    fig = go.Figure()
    fig.add_scatter(x=df['round'], y=df['wealth'], mode='lines+markers', name='Wealth')
    fig.add_scatter(x=df['round'], y=df['return'], mode='lines+markers', name='Return')
    fig.add_hline(y=df['wealth'][0], line_dash="solid", line_color="red")
    fig.add_hline(y=df['return'][0], line_dash="solid", line_color="red")
    fig.update_layout(
        title="Last Round Result",
        xaxis_title="Games",
        yaxis_title="Money",
    )
    fig.update_xaxes(range=[0, df.height])

    return fig