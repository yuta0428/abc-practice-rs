use proconio::input;

fn main() {
    input! {
        deg: f32,
        dis: f32
    }
    
    let disp = (dis / 60.0 * 10.0).round() / 10.0;
    let mut w = 0;
    if disp >= 0.3 { w += 1;}
    if disp >= 1.6 { w += 1;}
    if disp >= 3.4 { w += 1;}
    if disp >= 5.5 { w += 1;}
    if disp >= 8.0 { w += 1;}
    if disp >= 10.8 { w += 1;}
    if disp >= 13.9 { w += 1;}
    if disp >= 17.2 { w += 1;}
    if disp >= 20.8 { w += 1;}
    if disp >= 24.5 { w += 1;}
    if disp >= 28.5 { w += 1;}
    if disp >= 32.7 { w += 1;}

    let mut dir = match deg {
        i if i >= 112.5 && i < 337.5 => "NNE",
        i if i >= 337.5 && i < 562.5 => "NE",
        i if i >= 562.5 && i < 787.5 => "ENE",
        i if i >= 787.5 && i < 1012.5 => "E",
        i if i >= 1012.5 && i < 1237.5 => "ESE",
        i if i >= 1237.5 && i < 1462.5 => "SE",
        i if i >= 1462.5 && i < 1687.5 => "SSE",
        i if i >= 1687.5 && i < 1912.5 => "S",
        i if i >= 1912.5 && i < 2137.5 => "SSW",
        i if i >= 2137.5 && i < 2362.5 => "SW",
        i if i >= 2362.5 && i < 2587.5 => "WSW",
        i if i >= 2587.5 && i < 2812.5 => "W",
        i if i >= 2812.5 && i < 3037.5 => "WNW",
        i if i >= 3037.5 && i < 3262.5 => "NW",
        i if i >= 3262.5 && i < 3487.5 => "NNW",
        _ => "N"
    };
    if w == 0 { dir = "C";} 

    println!("{dir} {w}");
}