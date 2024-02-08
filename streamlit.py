import streamlit as st
from app.main import test

st.title('Martingale Simulator')
df = test()
st.dataframe(df.head())