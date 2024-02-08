import gambling_simulator
import polars as pl
import plotly.express as px
import plotly.graph_objs as go

def play_martingale_game(params:dict[str, float])->pl.DataFrame:

    data = gambling_simulator.play_martingale(
        params['n_simulations'],
        params['max_games'],
        params['init_wealth'],
        params['win_rate'],
        params['odds'],
        params['stake'],
    )
    df = pl.DataFrame(data)
    print(df.head())
    return df

def compute_stats(df:pl.DataFrame)->pl.DataFrame:
    return df.select(pl.col('final_wealth')).describe()

def compute_winrate(df:pl.DataFrame, init_wealth:int)->float:
    return (
        df.filter(pl.col('final_wealth') > init_wealth).count()
        /
        df.select(pl.len())
    )

def plot_wealth_dist(df:pl.DataFrame)->go.Figure:
    fig = px.histogram(df, x='final_wealth', title='Final Wealth Distribution')

    fig.update_layout(
        xaxis_title="Final Wealth",
        yaxis_title="Frequency",
        bargap=0.01,  # gap between bars of adjacent location coordinates
    )

    return fig