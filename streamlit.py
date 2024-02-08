import streamlit as st
from app.main import play_martingale_game, plot_wealth_dist, compute_stats, compute_earnrate

st.title('Martingale Simulator')

params = {
    'n_simulations': st.slider('Number of simulations', min_value=1000, max_value=100_000, value=10_000, step=1000),
    'max_games': st.number_input('Max games', min_value=10, max_value=1000, value=100),
    'init_wealth': st.number_input('Initial wealth', value=10000, max_value=1_000_000),
    'win_rate': st.slider('Win rate', min_value=0.0, max_value=1.0, value=0.5, step=0.01),
    'odds': st.number_input('Odds', value=1.75),
    'stake': st.number_input('Stake', value=100)
}

# if st.button('Start Simulation'):
df = play_martingale_game(params)
st.write(f'賺錢機率: {compute_earnrate(df, params["init_wealth"])}')
st.dataframe(compute_stats(df))
st.plotly_chart(plot_wealth_dist(df, params['init_wealth']))