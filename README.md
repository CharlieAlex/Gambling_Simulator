# Martingale Simulator

這是一個以模擬平賭法為主題，練習在 Python 中使用 pyO3 引入 Rust 程式碼，並使用 Streamlit 視覺化的專案。

平賭法是一種在無限本金下，穩賺不賠的投資(賭博?)策略，具體做法簡單來說:
1. 從最低下注金額開始
2. 如果這次下注贏了，下次就一樣出最低下注金額
3. 如果這次下注輸了，下次就根據賠率增加下注金額，把先前輸的金額贏回來

然而實際上，我們想知道的是在擁有的本金與時間限制下，這個策略的表現如何:
4. 本金限制: 給定本金上限，當所有的錢輸光時就出場停止這一輪的投資
5. 時間限制: 給定投資次數上限，當投資次數超過上限時也出場

當然這些限制不代表一生只能就玩這一次，重新將本金投入就可以開始新的一輪投資(賭博?)。

## 使用方法

1. 複製本專案: `git clone https://github.com/CharlieAlex/Martingale-Simulator.git`
2. 安裝套件: `pip install -r requirements.txt`
3. 啟動虛擬環境: `source .venv/bin/activate`
4. 建構 maturin: `maturin develop`
5. 啟動 Streamlit: `streamlit run streamlit.py`
6. 設定參數:
    - Number of simulations: 模擬次數
    - Max games: 投資次數上限
    - Initial wealth: 投資本金
    - Win rate: 每一次的勝率
    - Odds: 賠率
    - Stake: 最低下注金額
<div style="display: flex;">
  <img src="images/parameters.png?raw=true" alt="Parameters" style="width: 70%;">
</div>

備註: 理論上此程式應該要能 deploy 上 Streamlit 使用，然而目前 Streamlit 似乎尚未支援 Rust。

## 模擬結果

我們的目標是計算在所有的模擬次數中，有多少次最後是有盈餘的，即`賺錢機率`。

同時我們會產出最後一次的整個過程，讓使用者大致理解遊戲如何進行。

<div style="display: flex;">
  <img src="images/last_round.png?raw=true" alt="Parameters" style="width: 70%;">
</div>

當然我們也會產出所有模擬結果的統計與分配圖。

<div style="display: flex;">
  <img src="images/dist.png?raw=true" alt="Parameters" style="width: 70%;">
</div>
<div style="display: flex;">
  <img src="images/statistic.png?raw=true" alt="Parameters" style="width: 70%;">
</div>

從大部分的統計結果可以看出此方法的賺錢機率並沒有想像中高，
主要當然是因為我們設定了本金以及時間的限制。
不過如果偶而進賭場前想測試在各種條件下使用平賭法的勝率時，
還是推薦大家可以用用看這個程式！

## 參考資源

[平賭法vs.逆平賭法 》幾乎都贏和不會大輸，你要選哪一個?](https://rich01.com/blog-pos-22/)