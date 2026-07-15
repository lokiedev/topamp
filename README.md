# Discontinued
I didn't know that Poweramp already had a built-in feature for this. Too bad, my first instinct was to make it myself whenever I couldn't find a tool for something.

# Topamp
Topamp (meaning "To Poweramp") is a tool that converts a [squig.link](https://squig.link) Parametric Equalizer into a Poweramp preset using Rust WebAssembly. You can try it out at [lokiedev.github.io/topamp](https://lokiedev.github.io/topamp).

## About Topamp
Currently, I don't plan to make any more features to Topamp, as it has worked properly (at least) for me. However, I will continue to fix any bugs if there is one.

## Example
squig.link PEQ (Kefine Klean Silver filter targeted to Harman 2025 MoA Average):
```
Preamp: -6.5 dB
Filter 1: ON PK Fc 29 Hz Gain -5.8 dB Q 0.500
Filter 2: ON PK Fc 130 Hz Gain -2.6 dB Q 1.400
Filter 3: ON PK Fc 540 Hz Gain 2.6 dB Q 1.200
Filter 4: ON PK Fc 1600 Hz Gain -2.7 dB Q 0.800
Filter 5: ON PK Fc 3000 Hz Gain 3.4 dB Q 2.000
Filter 6: ON PK Fc 7200 Hz Gain -3.4 dB Q 2.000
Filter 7: ON PK Fc 9200 Hz Gain 10.3 dB Q 1.900
Filter 8: ON PK Fc 12000 Hz Gain -7.9 dB Q 2.000
Filter 9: OFF PK Fc 0 Hz Gain 0.0 dB Q 0.000
Filter 10: OFF PK Fc 0 Hz Gain 0.0 dB Q 0.000
```

Converted using Topamp:
```json
{
  "name": "Kefine Klean",
  "preamp": -6.5,
  "parametric": true,
  "bands": [
    {
      "type": 0,
      "channels": 0,
      "frequency": 90,
      "q": 0.8,
      "gain": 0.0,
      "color": 0
    },
    {
      "type": 1,
      "channels": 0,
      "frequency": 10000,
      "q": 0.6,
      "gain": 0.0,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 29,
      "q": 0.5,
      "gain": -5.8,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 130,
      "q": 1.4,
      "gain": -2.6,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 540,
      "q": 1.2,
      "gain": 2.6,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 1600,
      "q": 0.8,
      "gain": -2.7,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 3000,
      "q": 2.0,
      "gain": 3.4,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 7200,
      "q": 2.0,
      "gain": -3.4,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 9200,
      "q": 1.9,
      "gain": 10.3,
      "color": 0
    },
    {
      "type": 3,
      "channels": 0,
      "frequency": 12000,
      "q": 2.0,
      "gain": -7.9,
      "color": 0
    }
  ]
}
```
Note: there was a two band that basically does nothing in the output that I've deleted to make things shorter, those band came from Filter 9 and Filter 10 (which was OFF) in the squig.link PEQ example. This is a potential issue as Topamp currently doesn't know how to handle that type of filter.
