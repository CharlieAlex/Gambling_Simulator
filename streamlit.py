import streamlit as st
from app.main import (
    play_martingale_game,
    plot_wealth_dist, plot_process_lineplot,
    compute_stats, compute_earnrate
)

st.title('Martingale Simulator')

params = {
    'n_simulations': st.slider('Number of simulations', min_value=1000, max_value=100_000, value=10_000, step=1000),
    'max_games': st.number_input('Max games', min_value=10, max_value=1000, value=100),
    'init_wealth': st.number_input('Initial wealth', value=10000, max_value=1_000_000),
    'win_rate': st.slider('Win rate', min_value=0.0, max_value=1.0, value=0.5, step=0.01),
    'odds': st.number_input('Odds', value=1.75),
    'stake': st.number_input('Stake', value=100)
}

if st.button('Start Simulation'):
    output, last_process = play_martingale_game(params)
    st.write(f'賺錢機率: {compute_earnrate(output, params["init_wealth"])}')
    st.plotly_chart(plot_process_lineplot(last_process))
    st.plotly_chart(plot_wealth_dist(output, params['init_wealth']))
    st.table(compute_stats(output))