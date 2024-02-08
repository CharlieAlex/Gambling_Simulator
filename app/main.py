import gambling_simulator
import polars as pl

def test():
    data = gambling_simulator.play_martingale()
    df = pl.DataFrame(data)
    print(df.head())
    return df
