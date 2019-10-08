# ta-lib-rust
[![Build Status](https://travis-ci.org/CLevasseur/ta-lib-rust.svg?branch=master)](https://travis-ci.org/CLevasseur/ta-lib-rust)  

TA-lib bindings for Rust

TA-lib is a multi-platform library for market analysis. It includes 200 indicators such as ADX, MACD, RSI, Stochastic, Bollinger Bands, as well as Candlestick pattern recognition.
    
https://www.ta-lib.org/

## Installation

Add the following dependency in your `Cargo.toml` file:

```toml
[dependencies]
ta-lib-wrapper = "0.2.0"
```

And this to your crate root

```rust
extern crate ta_lib_wrapper;
```

## Dependencies

This wrapper requires that you have installed the TA-lib C library.  
You can download it at https://www.ta-lib.org/hdr_dw.html.  
The instructions to install the library can be found at https://www.ta-lib.org/d_api/d_api.html and are summarized below.

##### Linux

```bash
wget http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-src.tar.gz
tar xvzf ta-lib-0.4.0-src.tar.gz
rm ta-lib-0.4.0-src.tar.gz
cd ta-lib
./configure
make
sudo make install
```

##### macOS

```bash
brew install ta-lib
```

##### Windows

Download [ta-lib-0.4.0-msvc.zip](http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-msvc.zip)
and unzip to ``C:\ta-lib``

> This is a 32-bit release.  If you want to use 64-bit Python, you will need
> to build a 64-bit version of the library.
> Some unofficial (and unsupported) instructions for building on 64-bit Windows 10, here for reference:
> 1. Download and Unzip ``ta-lib-0.4.0-msvc.zip``
> 2. Move the Unzipped Folder ``ta-lib`` to ``C:\``
> 3. Download and Install Visual Studio Community 2015
> * Remember to Select ``[Visual C++]`` Feature
> 4. Build TA-Lib Library
> * From Windows Start Menu, Start ``[VS2015 x64 Native Tools Command Prompt]``
> * Move to ``C:\ta-lib\c\make\cdr\win32\msvc``
> * Build the Library ``nmake``

## Documentation

The `examples` folder shows how to compute some common indicators using this library.  
The full list of functions provided by these bindings can be obtained by building your project and open the file `target/*/build/ta-lib-wrapper-*/out/bindings.rs`

## Supported Indicators

All indicators provided by TA-lib are available:

```
AD                  Chaikin A/D Line
ADOSC               Chaikin A/D Oscillator
ADX                 Average Directional Movement Index
ADXR                Average Directional Movement Index Rating
APO                 Absolute Price Oscillator
AROON               Aroon
AROONOSC            Aroon Oscillator
ATR                 Average True Range
AVGPRICE            Average Price
BBANDS              Bollinger Bands
BETA                Beta
BOP                 Balance Of Power
CCI                 Commodity Channel Index
CDL2CROWS           Two Crows
CDL3BLACKCROWS      Three Black Crows
CDL3INSIDE          Three Inside Up/Down
CDL3LINESTRIKE      Three-Line Strike 
CDL3OUTSIDE         Three Outside Up/Down
CDL3STARSINSOUTH    Three Stars In The South
CDL3WHITESOLDIERS   Three Advancing White Soldiers
CDLABANDONEDBABY    Abandoned Baby
CDLADVANCEBLOCK     Advance Block
CDLBELTHOLD         Belt-hold
CDLBREAKAWAY        Breakaway
CDLCLOSINGMARUBOZU  Closing Marubozu
CDLCONCEALBABYSWALL Concealing Baby Swallow
CDLCOUNTERATTACK    Counterattack
CDLDARKCLOUDCOVER   Dark Cloud Cover
CDLDOJI             Doji
CDLDOJISTAR         Doji Star
CDLDRAGONFLYDOJI    Dragonfly Doji
CDLENGULFING        Engulfing Pattern
CDLEVENINGDOJISTAR  Evening Doji Star
CDLEVENINGSTAR      Evening Star
CDLGAPSIDESIDEWHITE Up/Down-gap side-by-side white lines
CDLGRAVESTONEDOJI   Gravestone Doji
CDLHAMMER           Hammer
CDLHANGINGMAN       Hanging Man
CDLHARAMI           Harami Pattern
CDLHARAMICROSS      Harami Cross Pattern
CDLHIGHWAVE         High-Wave Candle
CDLHIKKAKE          Hikkake Pattern
CDLHIKKAKEMOD       Modified Hikkake Pattern
CDLHOMINGPIGEON     Homing Pigeon
CDLIDENTICAL3CROWS  Identical Three Crows
CDLINNECK           In-Neck Pattern
CDLINVERTEDHAMMER   Inverted Hammer
CDLKICKING          Kicking
CDLKICKINGBYLENGTH  Kicking - bull/bear determined by the longer marubozu
CDLLADDERBOTTOM     Ladder Bottom
CDLLONGLEGGEDDOJI   Long Legged Doji
CDLLONGLINE         Long Line Candle
CDLMARUBOZU         Marubozu
CDLMATCHINGLOW      Matching Low
CDLMATHOLD          Mat Hold
CDLMORNINGDOJISTAR  Morning Doji Star
CDLMORNINGSTAR      Morning Star
CDLONNECK           On-Neck Pattern
CDLPIERCING         Piercing Pattern
CDLRICKSHAWMAN      Rickshaw Man
CDLRISEFALL3METHODS Rising/Falling Three Methods
CDLSEPARATINGLINES  Separating Lines
CDLSHOOTINGSTAR     Shooting Star
CDLSHORTLINE        Short Line Candle
CDLSPINNINGTOP      Spinning Top
CDLSTALLEDPATTERN   Stalled Pattern
CDLSTICKSANDWICH    Stick Sandwich
CDLTAKURI           Takuri (Dragonfly Doji with very long lower shadow)
CDLTASUKIGAP        Tasuki Gap
CDLTHRUSTING        Thrusting Pattern
CDLTRISTAR          Tristar Pattern
CDLUNIQUE3RIVER     Unique 3 River
CDLUPSIDEGAP2CROWS  Upside Gap Two Crows
CDLXSIDEGAP3METHODS Upside/Downside Gap Three Methods
CMO                 Chande Momentum Oscillator
CORREL              Pearson's Correlation Coefficient (r)
DEMA                Double Exponential Moving Average
DX                  Directional Movement Index
EMA                 Exponential Moving Average
HT_DCPERIOD         Hilbert Transform - Dominant Cycle Period
HT_DCPHASE          Hilbert Transform - Dominant Cycle Phase
HT_PHASOR           Hilbert Transform - Phasor Components
HT_SINE             Hilbert Transform - SineWave
HT_TRENDLINE        Hilbert Transform - Instantaneous Trendline
HT_TRENDMODE        Hilbert Transform - Trend vs Cycle Mode
KAMA                Kaufman Adaptive Moving Average
LINEARREG           Linear Regression
LINEARREG_ANGLE     Linear Regression Angle
LINEARREG_INTERCEPT Linear Regression Intercept
LINEARREG_SLOPE     Linear Regression Slope
MA                  All Moving Average
MACD                Moving Average Convergence/Divergence
MACDEXT             MACD with controllable MA type
MACDFIX             Moving Average Convergence/Divergence Fix 12/26
MAMA                MESA Adaptive Moving Average
MAX                 Highest value over a specified period
MAXINDEX            Index of highest value over a specified period
MEDPRICE            Median Price
MFI                 Money Flow Index
MIDPOINT            MidPoint over period
MIDPRICE            Midpoint Price over period
MIN                 Lowest value over a specified period
MININDEX            Index of lowest value over a specified period
MINMAX              Lowest and highest values over a specified period
MINMAXINDEX         Indexes of lowest and highest values over a specified period
MINUS_DI            Minus Directional Indicator
MINUS_DM            Minus Directional Movement
MOM                 Momentum
NATR                Normalized Average True Range
OBV                 On Balance Volume
PLUS_DI             Plus Directional Indicator
PLUS_DM             Plus Directional Movement
PPO                 Percentage Price Oscillator
ROC                 Rate of change : ((price/prevPrice)-1)*100
ROCP                Rate of change Percentage: (price-prevPrice)/prevPrice
ROCR                Rate of change ratio: (price/prevPrice)
ROCR100             Rate of change ratio 100 scale: (price/prevPrice)*100
RSI                 Relative Strength Index
SAR                 Parabolic SAR
SAREXT              Parabolic SAR - Extended
SMA                 Simple Moving Average
STDDEV              Standard Deviation
STOCH               Stochastic
STOCHF              Stochastic Fast
STOCHRSI            Stochastic Relative Strength Index
SUM                 Summation
T3                  Triple Exponential Moving Average (T3)
TEMA                Triple Exponential Moving Average
TRANGE              True Range
TRIMA               Triangular Moving Average
TRIX                1-day Rate-Of-Change (ROC) of a Triple Smooth EMA
TSF                 Time Series Forecast
TYPPRICE            Typical Price
ULTOSC              Ultimate Oscillator
VAR                 Variance
WCLPRICE            Weighted Close Price
WILLR               Williams' %R
WMA                 Weighted Moving Average
```
